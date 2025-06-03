use std::io::{Result, Write, stdin, stdout};

use solitaire::{self, core::moves::Move};
fn main() -> Result<()> {
    let mut game = solitaire::setup();
    while !game.is_over() {
        clear_screen();
        solitaire::print_table(game.table());
        let input = match take_input() {
            Ok(input) => input,
            Err(_) => "".to_string(),
        };
        let next_move = Move::from(input);
        match game.play(next_move) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{} {:?}", e, e.source());
            }
        }
    }
    clear_screen();
    println!("{game}");
    Ok(())
}

fn take_input() -> Result<String> {
    let mut buffer = String::new();
    print!("> ");
    let _ = stdout().flush();
    let stdin = stdin();
    stdin.read_line(&mut buffer)?;
    Ok(buffer)
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
