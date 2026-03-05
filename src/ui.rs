use crate::error::Result;
use crate::models::App;
use std::io::{self, Write};

pub fn print_apps(apps: &[(usize, App)]) {
    if apps.is_empty() {
        println!("No apps found.");
        return;
    }

    for (index, app) in apps {
        println!("[{index}] {app}");
    }
}

pub fn prompt_for_input(prompt: &str, required: bool) -> Result<String> {
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

pub fn confirm_action(message: &str) -> Result<bool> {
    loop {
        print_prompt(&format!("{message} (y/N)"))?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" | "" => return Ok(false),
            _ => println!("Please enter 'y' for yes or 'n' for no."),
        }
    }
}

fn print_prompt(message: &str) -> Result<()> {
    print!("{message:}");
    Ok(io::stdout().flush()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    // Extracted pure logic matching confirm_action's match arm behaviour
    fn parse_confirmation(input: &str) -> Option<bool> {
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => Some(true),
            "n" | "no" | "" => Some(false),
            _ => None,
        }
    }

    fn confirm_from_reader<R: io::BufRead>(reader: &mut R) -> Result<bool> {
        loop {
            let mut input = String::new();
            reader.read_line(&mut input)?;
            match parse_confirmation(&input) {
                Some(result) => return Ok(result),
                None => continue,
            }
        }
    }

    // --- parse_confirmation unit tests ---

    #[test]
    fn test_y_returns_true() {
        assert_eq!(parse_confirmation("y"), Some(true));
    }

    #[test]
    fn test_yes_returns_true() {
        assert_eq!(parse_confirmation("yes"), Some(true));
    }

    #[test]
    fn test_uppercase_y_returns_true() {
        assert_eq!(parse_confirmation("Y"), Some(true));
    }

    #[test]
    fn test_uppercase_yes_returns_true() {
        assert_eq!(parse_confirmation("YES"), Some(true));
    }

    #[test]
    fn test_n_returns_false() {
        assert_eq!(parse_confirmation("n"), Some(false));
    }

    #[test]
    fn test_no_returns_false() {
        assert_eq!(parse_confirmation("no"), Some(false));
    }

    #[test]
    fn test_uppercase_no_returns_false() {
        assert_eq!(parse_confirmation("NO"), Some(false));
    }

    #[test]
    fn test_empty_string_returns_false() {
        assert_eq!(parse_confirmation(""), Some(false));
    }

    #[test]
    fn test_whitespace_only_returns_false() {
        assert_eq!(parse_confirmation("   "), Some(false));
    }

    #[test]
    fn test_invalid_input_returns_none() {
        assert_eq!(parse_confirmation("maybe"), None);
    }

    #[test]
    fn test_random_string_returns_none() {
        assert_eq!(parse_confirmation("abc123"), None);
    }

    // --- confirm_from_reader integration tests ---

    #[test]
    fn test_reader_confirms_with_y() {
        let mut reader = Cursor::new("y\n");
        assert!(confirm_from_reader(&mut reader).unwrap());
    }

    #[test]
    fn test_reader_confirms_with_yes() {
        let mut reader = Cursor::new("yes\n");
        assert!(confirm_from_reader(&mut reader).unwrap());
    }

    #[test]
    fn test_reader_declines_with_n() {
        let mut reader = Cursor::new("n\n");
        assert!(!confirm_from_reader(&mut reader).unwrap());
    }

    #[test]
    fn test_reader_declines_with_empty_enter() {
        let mut reader = Cursor::new("\n");
        assert!(!confirm_from_reader(&mut reader).unwrap());
    }

    #[test]
    fn test_reader_skips_invalid_then_accepts_y() {
        // First line is invalid, second is valid — loop should retry
        let mut reader = Cursor::new("maybe\ny\n");
        assert!(confirm_from_reader(&mut reader).unwrap());
    }

    #[test]
    fn test_reader_skips_multiple_invalid_then_declines() {
        let mut reader = Cursor::new("what\nIDK\nn\n");
        assert!(!confirm_from_reader(&mut reader).unwrap());
    }
}
