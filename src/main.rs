use std::io::{Result, Write, stdin, stdout};

use solitaire::{
    self,
    core::{Error, moves::Move},
};
fn main() -> Result<()> {
    let mut game = solitaire::setup();
    while !game.is_over() {
        clear_screen();
        println!("{game}");
        solitaire::print_table(game.table());
        let input = take_input("\n>")?;
        let next_move = Move::from(input);
        if let Err(e) = game.play(next_move) {
            match e.downcast::<Error>() {
                Ok(err) => match err.as_ref() {
                    Error::Quit => {
                        clear_screen();
                        return Ok(());
                    }
                    Error::Help => {
                        clear_screen();
                        solitaire::print_help();
                    }
                    Error::History => {
                        clear_screen();
                        game.print_history();
                    }
                    Error::Win => break,
                    _ => {
                        let err = ansi_term::Colour::Red.paint(err.to_string());
                        eprintln!("{}", err);
                    }
                },
                Err(err) => {
                    let err = ansi_term::Colour::Red.paint(format!("Internal error: {}", err));
                    eprintln!("{}", err);
                }
            }
            take_input("")?;
        }
    }
    clear_screen();
    println!("You Won!");
    println!("{game}");
    Ok(())
}

fn take_input(hint: &str) -> Result<String> {
    let mut buffer = String::new();
    print!("{hint} ");
    let _ = stdout().flush();
    let stdin = stdin();
    stdin.read_line(&mut buffer)?;
    Ok(buffer)
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
