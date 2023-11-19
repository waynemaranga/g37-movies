// main.rs

use std::env;
use std::io;
use std::thread;
use std::time::Duration;
use termion::color;

mod python_runner;
mod recommender;
mod summary;

fn print_ascii_art() {
    // ASCII art of your choice with added color
    println!(
        "{}{}{}",
        termion::color::Fg(termion::color::Yellow),
        r#"
        
         __  ____  ______   __  __               _            
        / / |___ \|____  | |  \/  |             (_)           
       / /_   __) |   / /  | \  / |  ___ __   __ _   ___  ___ 
      | '_ \ |__ <   / /   | |\/| | / _ \\ \ / /| | / _ \/ __|
      | (_) |___) | / /    | |  | || (_) |\ V / | ||  __/\__ \
       \___/|____/ /_/     |_|  |_| \___/  \_/  |_| \___||___/
                                                            
"#,
        termion::color::Fg(termion::color::Reset),
    );

    // Add a delay of 2 seconds
    thread::sleep(Duration::from_secs(2));

    // Prompt the user to press Enter to continue
    println!("\nPress Enter to continue...");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}

fn main() -> io::Result<()> {
    // Print ASCII art with delay and prompt before running Python scripts
    print_ascii_art();

    // Run Python scripts
    let python_scripts = vec![
        ("src/scripts/clean.py", "data cleaning operation"),
        ("src/scripts/movies.py", "movie database creator script"),
        ("src/scripts/profiler.py", "user profile creator script"),
        ("src/scripts/users.py", "user identifier script"),
    ];

    let current_dir = env::current_dir()?;
    let python_interpreter = "python";

    for (script_path, script_name) in python_scripts {
        let script_path = current_dir.join(script_path);
        python_runner::execute_python_script(
            &script_path.to_string_lossy(),
            python_interpreter,
            script_name,
        )?;
    }

    // Run summary script
    summary::run_summary_script()?;
    // Run recommender script with progress bar
    recommender::run_recommend_script_with_progress()?;

    // Display "bye" message
    println!("bye");

    Ok(())
}
