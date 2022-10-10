// Minimal Rust text editor. Genuinely the bare minimum required to be considered one.

use termion::{
    event::Key,
    input::TermRead,
    raw::IntoRawMode
};

use std::{
    io::{Write, stdout, stdin, BufReader, Read},
    fs::File,
    env::{current_dir, args}
};

// Added result just so that I can use ? instead of unwrapping everything :)
fn main() {

    let args: Vec<String> = args().collect();
    
    if args.len() > 1 {

        // Get the file path from provided arguments as a String.

        let file_name = args[1].to_string();
        let directory = current_dir().unwrap();
        let directory_string = directory.to_string_lossy();

        let file_path = format!("{}{}{}", &directory_string, "/", &file_name);

        if let Ok(new_file) = File::open(&file_path) {
            
            // If the file exists already, read its contents and pass that into the editor,
            // as well as the file path.

            let mut buf_reader = BufReader::new(&new_file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();

            run(contents, &file_path)

        } 
        
        else {

            // If the file doesn't exist already, pass an empty string as the contents,
            // as well as the file path.

            run("".to_string(), &file_path);

        }

    }

    else {

        println!("File name not specified.");

    }

}

fn run(contents: String, file_path: &String) {

    // Now we're cooking with raw mode!

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Creates a variable with the contents in it.

    let mut buffer = contents;

    // Clears the terminal, goes to the origin, makes sure the cursor is visible,
    // then writes the edited buffer to the terminal.

    write!(stdout,
        "{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Show,
        newline_to_carriage_return(&buffer)
    ).unwrap();
    stdout.flush().unwrap();


    // In termion "for c in {}" essentially causes a loop. Don't understand why but it works.

    for c in stdin.keys() {

        // Clears everything after the cursor to create a blank window without creating
        // unnecessary blank space before the cursor.

        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::AfterCursor
        ).unwrap();

        match c.unwrap() {

            // Quit on ctrl(q), push any other character into the buffer,
            // remove the last character with backspace. 
            // When I said bare minimum, I meant it.

            Key::Ctrl('q') => break,

            Key::Char(c) => {

                buffer.push(c);

            }

            Key::Backspace => {

                buffer.pop();

            },

            // On ctrl(s), delete everything in the file and replace it with the buffer,
            // this time without the newline -> carriage return edit.

            Key::Ctrl('s') => {

                let mut new_file = File::create(&file_path).unwrap();

                new_file.write(buffer.as_bytes()).unwrap();
                

            },

            // Anything else does nothing.

            _ => {}
        }

        // Writes the buffer to the terminal.

        write!(
            stdout,
            "{}",
            newline_to_carriage_return(&buffer)
        ).unwrap();

        stdout.flush().unwrap();

    }

}

fn newline_to_carriage_return(current_text: &String) -> String {

    // Takes in a string and returns it with any \n characters as \r characters,
    // without altering the original string.
    // This may not be the ideal way of correctly writing things but
    // it was my way of doing it and it works.

    let mut string_as_chars: Vec<char> = current_text.chars().collect();
    let len = string_as_chars.len();

    let slice = ['\r'];

    for i in 0..len {

        let current_char: &char = string_as_chars.get(i).unwrap();

        if current_char == &'\n' {

            string_as_chars.splice(i+1..i+1, slice);
    
        }

    }

    return string_as_chars.into_iter().collect();

}
