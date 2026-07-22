use arboard::Clipboard;
use std::env;
use std::fs;
use std::process;
use std::thread;
use std::time::Duration;
fn main() {
let mut verbose = false;
    let mut file_path = String::new();

    // Skip the first argument (the executable name) and parse the rest
    for arg in env::args().skip(1) {
        if arg == "-v" || arg == "--verbose" {
            verbose = true;
        } else {
            // Assume any non-flag argument is the file path
            file_path = arg;
        }
    }

    if file_path.is_empty() {
        eprintln!("Usage: ccat [-v] <file_path>");
        process::exit(1);
    }

    let content = fs::read_to_string(&file_path).unwrap_or_else(|e| {
        // Errors always print to stderr, regardless of verbosity
        eprintln!("Error reading file '{}': {}", file_path, e);
        process::exit(1);
    });

    let mut clipboard = Clipboard::new().unwrap_or_else(|e| {
        eprintln!("Error initializing clipboard: {}", e);
        process::exit(1);
    });

    clipboard.set_text(content).unwrap_or_else(|e| {
        eprintln!("Error setting clipboard text: {}", e);
        process::exit(1);
    });
    
    // Only print success message if the verbose flag WAS passed
    if verbose {
        println!("Copied contents of {} to clipboard.", file_path);
    }

    // Give the OS clipboard manager time to grab the text before exiting
    thread::sleep(Duration::from_millis(100));
}