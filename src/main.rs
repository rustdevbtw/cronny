use chrono::Local;
use cron::Schedule;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    process::Command,
    str::FromStr,
    thread::sleep,
    time::Duration,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* Usage:
     * cronny file.cron
     */
    let file = std::env::args().nth(1).expect("Usage: cronny file.cron");
    let contents = fs::read_to_string(fs::canonicalize(&file)?).expect("No such file or directory");
    let lines: Vec<&str> = contents.lines().collect();
    let line = lines.first().unwrap();
    let full: Vec<&str> = line.split(" :: ").collect();
    let expr = full.first().unwrap();
    let sh = full.get(1).unwrap();
    let schedule = Schedule::from_str(expr)?;
    let local = Local::now();
    let tz = local.timezone();
    for dt in schedule.upcoming(tz) {
        let diff = dt.signed_duration_since(local).num_seconds() as u64;
        if diff == 0 {
            let out = if cfg!(target_os = "window") {
                Command::new("cmd").args(["/C", sh]).output()
            } else {
                Command::new("sh").args(["-c", sh]).output()
            };
            let rout = if let Ok(o) = out {
                let stdout = o.stdout;
                let stderr = o.stderr;
                if !stdout.is_empty() {
                    stdout
                } else {
                    stderr
                }
            } else {
                "".as_bytes().to_vec()
            };
            let output = std::str::from_utf8(rout.as_slice()).expect("");
            let logr = std::env::var("CRNY_LOG");
            let log = if let Ok(l) = logr { l } else { "1".to_string() }.parse::<u8>()?;
            if log == 1 {
                let mut logfile = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(format!("{file}.log"))
                    .expect("Could not open log file");
                logfile
                    .write_all(format!("[{expr} ({dt})] {sh}: {output}\n").as_bytes())
                    .expect("Failed to write to logfile!");
            }
        } else {
            sleep(Duration::from_secs(diff));
            let out = if cfg!(target_os = "windows") {
                Command::new("cmd").args(["/C", sh]).output()
            } else {
                Command::new("sh").args(["-c", sh]).output()
            };
            let rout = if let Ok(o) = out {
                let stdout = o.stdout;
                let stderr = o.stderr;
                if !stdout.is_empty() {
                    stdout
                } else {
                    stderr
                }
            } else {
                "".as_bytes().to_vec()
            };
            let output = std::str::from_utf8(rout.as_slice()).expect("");
            let logr = std::env::var("CRNY_LOG");
            let log = if let Ok(l) = logr { l } else { "1".to_string() }.parse::<u8>()?;
            if log == 1 {
                let mut logfile = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(format!("{file}.log"))
                    .expect("Could not open log file");
                logfile
                    .write_all(format!("[{expr} ({dt})] {sh}: {output}\n").as_bytes())
                    .expect("Failed to write to logfile!");
            }
        }
    }
    Ok(())
}
