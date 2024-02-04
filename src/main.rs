/* Este es un proyecto sin acabar que he creado para aprender Rust. Si tienes conocimientos del lenguaje, y algo de tiempo,
agradecería que mejoraras este proyecto o me dieras sugerencias. Eso me ayudaría mucho a mejorar :D

This is an unfinished project created to learn Rust. If you have knowledge of the language and some time,
I would appreciate it if you could improve this project or give suggestions. That would help me a lot to improve :D
*/

#[macro_use]
extern crate crossterm;
use crossterm::cursor;
use crossterm::terminal::{Clear, ClearType};
use std::io::stdout;
use console::Term;

//fn execute_in_background() {}

fn clear_screen() {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
}

fn read_character() {
    let stdout = Term::buffered_stdout();
    
    loop {
        if let Ok(character) = stdout.read_char() {
            println!("{character}");
        }
    }

    /*
    loop {
        let character = stdout.read_char();
        println!("{:?}", character);
    }
    */
}

fn main() {
    clear_screen();
    read_character();
}
