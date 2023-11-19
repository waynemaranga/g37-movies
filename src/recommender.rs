#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_imports)]
// #[allow(unused)]
// --------------------------------------------------------
use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::time::Duration;
use termion::color;

pub fn run_recommend_script_with_progress() -> io::Result<()> {
    let script_path = "src/scripts/recommender_input.py"; // path to the Python script to execute

    let status = Command::new("python").arg(script_path).status()?; // status of the Python script execution
                                                                    // TODO: make this configurable for use with virtualenvs

    // Check the exit status
    if status.success() {
        // print!("{}{}{}{} executed successfully!", color::Fg(color::Green), style::Bold, script_name, color::Fg(color::Reset));
        println!(
            "{}Recommender script executed successfully!",
            color::Fg(color::Green)
        );
    } else {
        // error handling for Python script execution
        println!(
            "{}Failed to execute Recommender script. Exit code: {:?}",
            color::Fg(color::Red),
            status.code().unwrap_or_default()
        );
    }

    // Display "bye" message
    println!("Buh-bye");

    Ok(())
}

fn print_progress_bar(progress: usize, total: usize) {
    // FIXME: redundant code, should be refactored
    let percentage = (progress * 100) / total;
    let num_hashes = (progress * 20) / total;

    print!("[");
    for _ in 0..num_hashes {
        // progress bar with square brackets
        print!("#");
    }
    for _ in num_hashes..20 {
        print!("-");
    }
    print!("] {}%", percentage);
    print!("{}", termion::cursor::Left(23)); // Move the cursor back to overwrite the progress bar
    io::stdout().flush().unwrap();
}
