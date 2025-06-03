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
        let input = take_input(">")?;
        let next_move = Move::from(input);
        match game.play(next_move) {
            Ok(_) => (),
            Err(e) => {
                match e.downcast::<Error>() {
                    Ok(err) => match err.as_ref() {
                        Error::Quit => {
                            let err = ansi_term::Colour::Red.paint(err.to_string());
                            eprintln!("{}", err);
                            return Ok(());
                        }
                        Error::Help => {
                            clear_screen();
                            print_help();
                        }
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

fn print_help() {
    println!(
        "Controls:\n
    H                   Print help\n
    Q                   Quit game\n
    L                   Print move history\n
    [Pile1][Pile2]      Automatically move cards from Pile1 to Pile2\n
    [Pile1][Pile2][n]   Move n cards from Pile1 to Pile2\n
    D                   Draw a card from the uncovered pile\n
    U                   Undo last move\n
    Pile can be any between 1-7, P, C, D, H, S"
    );
}
