use std::env::args;
use std::io::{stdin, Read};
use std::process::Command;

fn main() {
    if let Some(cmd) = args().nth(1) {
        match cmd.as_str() {
            "file" => read_file_go(),
            "text" => copy_text(),
            "in" => stdin_go(),
            _ => print_help(),
        }
    } else {
        print_help();
    }
}

fn stdin_go() {
    let mut lines = stdin().lines();
    let mut stdin_input = String::new();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if stdin_input.len() > 0 {
            stdin_input.push_str("\n");
        }
        stdin_input.push_str(&line);
    }
    copy(stdin_input);
}

fn copy(stdin_input: String) {
    if let Err(e) = Command::new("wl-copy").arg(stdin_input).spawn() {
        eprintln!("Failed to copy in: {:?}", e);
    }
}

fn read_file_go() {
    use std::fs::File;
    if let Some(path) = args().nth(2) {
        if let Ok(mut file) = File::open(&path) {
            let mut buf = String::new();
            let _ = file.read_to_string(&mut buf);
            copy(buf);
        } else {
            eprintln!("Failed to open {}.", { &path });
        }
    } else {
        eprintln!("No file provided.")
    }
}
fn copy_text() {
    let mut content = String::new();
    for (i, word) in args().enumerate() {
        if i > 1 {
            content.push_str(word.as_str());
            content.push(' ');
        }
    }
    copy(content);
}
fn print_help() {
    println!("Usage Example:");
    println!("\tcopy file filename");
    println!("\tcopy in < ~/.config/fish/config.fish");
    println!("\techo hello | copy in");
    println!("\tcopy text 'yes'");
    println!("\tcopy help");
    println!("\tMake sure you exec 'wl-copy'.");
}
