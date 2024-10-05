use clap::Parser;
use colored::*;
use std::fs;
use std::process::Command;
use terminal_size::{terminal_size, Width};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The mandatory string argument
    command: String,

    /// Ignore errors during execution
    #[arg(short = 'i', long = "ignore-errors", default_value = "false")]
    ignore_errors: bool,
}

fn main() {
    let cli = Cli::parse();

    let terminal_width = if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        50
    };
    // Use U+2500 which appears as smooth line in terminals like wezterm that supports ligatures
    let separator = "─".repeat(terminal_width as usize);

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
                    println!("{}", path_display_name.to_string().cyan().bold());
                    println!("{}", separator.white());

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
                                println!("{}", separator.red());
                            }

                            if !output.stdout.is_empty() {
                                print!("{}", String::from_utf8_lossy(&output.stdout));
                                println!("{}", separator.white());
                            }

                            if !output.stderr.is_empty() && !cli.ignore_errors {
                                println!(
                                    "{}",
                                    String::from_utf8_lossy(&output.stderr).red().bold()
                                );
                                println!("{}", separator.red());
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
