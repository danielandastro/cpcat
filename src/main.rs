use arboard::Clipboard;
use std::env;
use std::fs;
use std::process;
use std::thread;
use std::time::Duration;
fn main() {
    // 1. Get the file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ccat <file_path>");
        process::exit(1);
    }

    let file_path = &args[1];

    // 2. Read the file contents
    let content = match fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            process::exit(1);
        }
    };

    // 3. Send to clipboard
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(content).unwrap();
    
    println!("Successfully copied contents of {} to clipboard.", file_path);
    thread::sleep(Duration::from_millis(100));
}