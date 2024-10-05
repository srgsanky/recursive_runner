use atty::Stream;
use clap::Parser;
use colored::{
    Color::{self, *},
    Colorize,
};
use std::fs;
use std::process::Command;
use terminal_size::{terminal_size, Width};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The mandatory string argument
    command: String,

    /// Ignore errors during execution
    #[arg(short = 'i', long = "ignore-errors", default_value_t = false)]
    ignore_errors: bool,
}

const DEFAULT_SEPARATOR_WIDTH: usize = 50;

fn print_with_color(text: &str, color: Option<Color>, is_terminal: bool) {
    if is_terminal {
        match color {
            Some(c) => println!("{}", text.color(c)),
            None => println!("{}", text),
        }
    } else {
        println!("{}", text);
    }
}

fn main() {
    let cli = Cli::parse();

    let is_terminal;
    let terminal_width;

    // Use color only if stdout is a terminal
    if atty::is(Stream::Stdout) {
        is_terminal = true;

        terminal_width = if let Some((Width(w), _)) = terminal_size() {
            w as usize
        } else {
            DEFAULT_SEPARATOR_WIDTH
        };
    } else {
        is_terminal = false;
        terminal_width = DEFAULT_SEPARATOR_WIDTH;
    }

    // Use U+2500 which appears as smooth line in terminals like wezterm that supports ligatures
    let separator = "─".repeat(terminal_width);

    if let Ok(entries) = fs::read_dir(".") {
        entries.for_each(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let path_display_name = path.display();
                    // Ignore hidden directories like .git
                    if path_display_name.to_string().starts_with("./.") {
                        return;
                    }

                    print_with_color(&path_display_name.to_string(), Some(Cyan), is_terminal);
                    print_with_color(&separator, Some(White), is_terminal);

                    let output = Command::new("sh")
                        .arg("-c")
                        .arg(&cli.command)
                        .current_dir(path)
                        // Set TERM environment variable so that commands will prefer colored output
                        .env("TERM", "xterm-256color")
                        .output();

                    match output {
                        Ok(output) => {
                            if !output.status.success() && !cli.ignore_errors {
                                println!("Status: {}", output.status.code().unwrap());
                                print_with_color(&separator, Some(Red), is_terminal);
                            }

                            if !output.stdout.is_empty() {
                                print!("{}", String::from_utf8_lossy(&output.stdout));
                                print_with_color(&separator, Some(White), is_terminal);
                            }

                            if !output.stderr.is_empty() && !cli.ignore_errors {
                                print_with_color(
                                    &String::from_utf8_lossy(&output.stderr),
                                    Some(Red),
                                    is_terminal,
                                );
                                print_with_color(&separator, Some(Red), is_terminal);
                            }
                        }
                        Err(e) => eprintln!("Failed to execute command: {}", e),
                    }

                    println!("\n")
                }
            }
        });
    } else {
        println!("Failed to read current directory");
    }
}
