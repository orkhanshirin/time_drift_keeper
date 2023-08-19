use std::fs::File;
use std::io::{self, prelude::*, BufWriter, BufReader};
use std::thread;
use std::time::{Duration, Instant};

use chrono::Local;
use clap::{App, Arg};

const OUTPUT_FILE: &str = "time_measurements.txt";

fn record_time() -> io::Result<()> {
    let interval = Duration::from_secs(1);
    let output_file = File::create(OUTPUT_FILE)?;
    let mut writer = BufWriter::new(output_file);

    let start_time = Instant::now();

    println!("Recording time... Press Ctrl+C to stop.");

    loop {
        let current_time = Local::now();
        let formatted_time = current_time.format("%s.%f");

        // write the formatted time to the output file
        writeln!(writer, "{}", formatted_time)?;

        // flush the buffer to ensure the write is immediate
        writer.flush()?;

        // sleep for the specified interval
        let elapsed = Instant::now().duration_since(start_time);
        if let Some(sleep_time) = interval.checked_sub(elapsed) {
            thread::sleep(sleep_time);
        }
    }
}

fn measure_time_drift() -> io::Result<()> {
    let input_file = File::open(OUTPUT_FILE)?;
    let reader = BufReader::new(input_file);
    let mut previous_time = None;

    for line in reader.lines() {
        let time_str = line?;
        let current_time = time_str.parse::<f64>().unwrap();

        if let Some(prev_time) = previous_time {
            let drift = current_time - prev_time;
            println!("Time Drift: {drift} seconds");
        }

        previous_time = Some(current_time);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let matches = App::new("Time Drift CLI Tool")
        .version("1.0")
        .author("Your Name")
        .about("Records time or measures time drift")
        .arg(
            Arg::with_name("action")
                .help("Action to perform")
                .possible_values(&["record", "measure"])
                .required(true),
        )
        .get_matches();

    match matches.value_of("action").unwrap() {
        "record" => record_time(),
        "measure" => measure_time_drift(),
        _ => unreachable!(),
    }
}
