// recommender.rs

use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::time::Duration;
use termion::color;

pub fn run_recommend_script_with_progress() -> io::Result<()> {
    // Specify the path to the recommender_input.py script
    let script_path = "src/scripts/recommender_input.py";

    // Execute the Python script using the "python" command
    let status = Command::new("python").arg(script_path).status()?;

    // Check the exit status
    if status.success() {
        println!(
            "{}Recommender script executed successfully!",
            color::Fg(color::Green)
        );
    } else {
        println!(
            "{}Failed to execute Recommender script. Exit code: {:?}",
            color::Fg(color::Red),
            status.code().unwrap_or_default()
        );
    }

    // Display "bye" message
    println!("bye");

    Ok(())
}

fn print_progress_bar(progress: usize, total: usize) {
    let percentage = (progress * 100) / total;
    let num_hashes = (progress * 20) / total;

    print!("[");
    for _ in 0..num_hashes {
        print!("#");
    }
    for _ in num_hashes..20 {
        print!("-");
    }
    print!("] {}%", percentage);
    print!("{}", termion::cursor::Left(23)); // Move the cursor back to overwrite the progress bar
    io::stdout().flush().unwrap();
}
