use crate::models::App;
use std::io::{self, Write};

pub fn print_apps(apps: &[(usize, App)]) {
    if apps.is_empty() {
        println!("No apps found.");
        return;
    }

    for (index, app) in apps {
        println!("[{}] {}", index, app);
    }
}

pub fn prompt_for_input(prompt: &str, required: bool) -> io::Result<String> {
    loop {
        print_prompt(prompt)?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim();

        if required && trimmed.is_empty() {
            println!("This field is required. Please enter a value.");
            continue;
        }

        return Ok(trimmed.to_string());
    }
}

pub fn confirm_action(message: &str) -> io::Result<bool> {
    loop {
        print_prompt(&format!("{} (y/N)", message))?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" | "" => return Ok(false),
            _ => println!("Please enter 'y' for yes or 'n' for no."),
        }
    }
}

fn print_prompt(message: &str) -> io::Result<()> {
    print!("{}: ", message);
    io::stdout().flush()
}
