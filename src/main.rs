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
    // ASCII art of your choice with added color
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

        // Add a delay of 2 seconds after script execution
        thread::sleep(Duration::from_secs(2));

        // Display description of the next step
        match script_name {
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

        // Prompt the user to press Enter before running the next script
        println!("\nPress Enter to continue...");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
    }

    // Run summary script
    summary::run_summary_script()?;
    // Add a delay of 3 seconds after summary script execution
    thread::sleep(Duration::from_secs(3));

    // Display description of the next step
    println!(
        "\nSummary script completed. Check this list out, because using it, we'll make some very well informed [lmao] recommendations.\n Netflix wishes it was this good. \nNext step: Running recommender...\n"
    );

    // Run recommender script with progress bar
    recommender::run_recommend_script_with_progress()?;

    // Display "bye" message
    println!("Now your great taste is about to get even better.\nCheck out the repository at https://github.com/waynemaranga/Recommender-System. \nHere's twenty emojis for no particular reason: \nBye!");

    Ok(())
}
