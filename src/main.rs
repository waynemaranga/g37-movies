#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_imports)]
// --------------------------------------------------------
use std::env;
use std::io;
use std::thread;
use std::time::Duration;
use termion::color;

mod python_runner;
mod recommender;
mod summary;

fn print_ascii_art() {
    // ASCII art
    println!(
        "{}{}{}",
        termion::color::Fg(termion::color::Yellow),
        r#"
        Hey there lucky user! Welcome to the GET Movies Recommender System.
        Don't mind the spelling error. It's intentional. I promise.
        Here's some ASCII art to get you started.
        
         __  ____  ______   __  __               _            
        / / |___ \|____  | |  \/  |             (_)           
       / /_   __) |   / /  | \  / |  ___ __   __ _   ___  ___ 
      | '_ \ |__ <   / /   | |\/| | / _ \\ \ / /| | / _ \/ __|
      | (_) |___) | / /    | |  | || (_) |\ V / | ||  __/\__ \
       \___/|____/ /_/     |_|  |_| \___/  \_/  |_| \___||___/

        Nice? No? Well, I tried. Anyway, let's get started with the movies.
        There's a file called data.txt in the db folder... Let's clean it up.                                                    
"#,
        termion::color::Fg(termion::color::Reset),
    );

    thread::sleep(Duration::from_secs(2)); // add a delay of 2 seconds after printing ASCII art

    println!("\nPress Enter to continue..."); // prompt the user to press Enter to continue
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}

fn main() -> io::Result<()> {
    print_ascii_art(); // print ASCII art to the console with delay

    // run Python scripts with progress bar in the terminal
    let python_scripts = vec![
        ("src/scripts/clean.py", "data cleaning operation"),
        ("src/scripts/movies.py", "movie database creator script"),
        ("src/scripts/profiler.py", "user profile creator script"),
        ("src/scripts/users.py", "user identifier script"),
    ];

    let current_dir = env::current_dir()?; // get the current directory
    let python_interpreter = "python"; // specify the Python interpreter to use // TODO: make this configurable for use with virtualenvs

    for (script_path, script_name) in python_scripts {
        // run Python script using the python_runner module
        let script_path = current_dir.join(script_path); // join the current directory with the script path
        python_runner::execute_python_script(
            &script_path.to_string_lossy(),
            python_interpreter,
            script_name,
        )?;

        thread::sleep(Duration::from_secs(2)); // add a delay of 2 seconds after running each script

        match script_name {
            // match statement to display a message after each script is run
            // TODO: make this more DRY
            "data cleaning operation" => {
                println!("\nThat's done, There's new files in the data folder now\nWe'll use them to create a list of users, their profiles\nand some movies.")
            }
            "movie database creator script" => {
                println!("\nMovie database creation completed. Next step: User profile creation.")
            }
            "user profile creator script" => {
                println!("\nUser profile creation completed. Next step: User identification.")
            }
            "user identifier script" => {
                println!("\nThat was simple. Now using the clean data, and the profiles, we're going to summarise the data.\n Next step: Running summary script.")
            }
            _ => (),
        }

        // prompt the user to press Enter before running the next script
        println!("\nPress Enter to continue...");
        let mut buffer = String::new(); // create a new String buffer
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line"); // read the line from the user without error handling
    }

    // run summary script
    summary::run_summary_script()?;
    // add a delay of 3 seconds after summary script execution
    thread::sleep(Duration::from_secs(3));

    // display description of the next step
    println!(
        "\nSummary script completed. Check this list out, because using it, we'll make some very well informed [lmao] recommendations.\n Netflix wishes it was this good. \nNext step: Running recommender...\n"
    );

    recommender::run_recommend_script_with_progress()?; // run recommender script with progress bar

    // display the goodbye message
    println!("Now your great taste is about to get even better.\nCheck out the repository at https://github.com/waynemaranga/Recommender-System. \nHere's twenty emojis for no particular reason: \nBye!");

    Ok(())
}
