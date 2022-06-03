use std::io::{stdout, Write};

pub fn clear_screen() {
    print!("{esc}c", esc = 27 as char);  
    stdout().flush().unwrap();
}