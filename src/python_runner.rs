#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_imports)]
// --------------------------------------------------------
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
    io::stdout().flush()?; // flush method to ensure the output is printed immediately

    // simulate a loading bar. should be replaced with a real progress bar
    for _ in 0..10 {
        // print!("#");

        print!("{}", color::Fg(color::Blue));
        io::stdout().flush()?;
        print!("█");
        io::stdout().flush()?;
        sleep(Duration::from_millis(200));
    }

    // Execute the Python script
    let status = Command::new(interpreter).arg(script_path).status()?; // TODO: make this configurable for use with virtualenvs
    if status.success() {
        // print!("{}{}{}{} executed successfully!", color::Fg(color::Green), style::Bold, script_name, color::Fg(color::Reset));
        println!(
            "\n{}{}{}{}{} executed successfully!",
            color::Fg(color::Green),
            style::Bold,
            script_name,
            color::Fg(color::Reset),
            style::Reset
        );
    } else {
        // error handling for Python script execution
        // FIXME: TRY TO GET THE ERROR MESSAGE FROM PYTHON TERMINAL
        println!(
            "{}{}{}{}Failed to execute {}. Exit code: {:?}",
            color::Fg(color::Red),
            style::Bold,
            script_name,
            color::Fg(color::Reset),
            script_name,
            status.code().unwrap_or_default()
        ); // should be replaced with a real error message
    }
    Ok(()) //TODO: success codes should be returned along with Ok(())
}
