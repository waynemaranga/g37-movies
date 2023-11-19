// File: src/summary.rs

use std::fs;
use std::io;
use std::path::Path;
use termion::color;
use termion::style;

pub fn run_summary_script() -> io::Result<()> {
    println!(
        "{}{}{}{}{}",
        color::Fg(color::Green),
        style::Bold,
        style::Underline,
        "Running summary script...",
        style::Reset
    );

    // Execute the summary script
    let status = std::process::Command::new("python")
        .arg("src/scripts/summary.py")
        .status()?;

    if status.success() {
        println!(
            "{}{}Summary script executed successfully!{}",
            color::Fg(color::Green),
            style::Bold,
            style::Reset
        );

        // Check if summary.txt exists before printing insights
        if Path::new("src/data/summary.txt").exists() {
            // Print insights from summary.txt
            print_summary_insights()?;
        } else {
            println!(
                "{}{}Summary.txt not found. Generating insights from summary.json only.{}",
                color::Fg(color::Yellow),
                style::Bold,
                style::Reset
            );
        }
    } else {
        println!(
            "{}{}Failed to execute summary script. Exit code: {:?}{}",
            color::Fg(color::Red),
            style::Bold,
            status.code().unwrap_or_default(),
            style::Reset
        );
    }

    Ok(())
}

fn print_summary_insights() -> io::Result<()> {
    // Read summary.txt file
    let summary_txt = fs::read_to_string("src/data/summary.txt")?;

    // Print insights
    println!(
        "{}{}Summary Insights:{}{}",
        color::Fg(color::Blue),
        style::Bold,
        style::Reset,
        color::Fg(color::Reset)
    );

    // Print contents of summary.txt
    println!("{}", summary_txt);

    Ok(())
}
