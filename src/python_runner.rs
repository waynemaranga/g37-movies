// File: src/python_runner.rs

use std::io;
use std::io::Write;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use termion::clear;
use termion::color;
use termion::cursor;
use termion::style;

pub fn execute_python_script(
    script_path: &str,
    interpreter: &str,
    script_name: &str,
) -> io::Result<()> {
    print!(
        "{}{}{}{}Running {}... ",
        clear::All,
        cursor::Goto(1, 1),
        color::Fg(color::Green),
        style::Bold,
        script_name
    );
    io::stdout().flush()?; // Ensure the message is immediately visible

    // Simulate a loading bar
    for _ in 0..10 {
        print!("{}", color::Fg(color::Blue));
        io::stdout().flush()?;
        print!("█");
        io::stdout().flush()?;
        sleep(Duration::from_millis(200));
    }

    // Execute the Python script
    let status = Command::new(interpreter).arg(script_path).status()?;
    if status.success() {
        println!(
            "{}{}{}{}{} executed successfully!",
            color::Fg(color::Green),
            style::Bold,
            script_name,
            color::Fg(color::Reset),
            style::Reset
        );
    } else {
        println!(
            "{}{}{}{}Failed to execute {}. Exit code: {:?}",
            color::Fg(color::Red),
            style::Bold,
            script_name,
            color::Fg(color::Reset),
            script_name,
            status.code().unwrap_or_default()
        );
    }
    Ok(())
}
