use solitaire::{
    self,
    core::{Error, moves::Move},
};
use std::io::{Result, Write, stdin, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn main() -> Result<()> {
    let mut game = solitaire::setup();
    while !game.is_over() {
        clear_screen();
        println!("\n{game}");
        solitaire::print_table(game.table());
        let input = take_input();
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
                        println!("Press Enter to continue");
                    }
                    Error::History => {
                        clear_screen();
                        game.print_history();
                        println!("Press Enter to continue");
                    }
                    Error::Win => break,
                    _ => {
                        let mut stdout = stdout().into_raw_mode().unwrap();
                        let err = ansi_term::Colour::Red.paint(err.to_string());
                        write!(stdout, "{}{}", termion::cursor::Goto(1, 1), err).unwrap();
                        stdout.flush().unwrap();
                    }
                },
                Err(err) => {
                    let err = ansi_term::Colour::Red.paint(format!("Internal error: {}", err));
                    eprintln!("{}", err);
                }
            }
            let _ = take_input();
        }
    }
    clear_screen();
    println!("You Won!");
    println!("{game}");
    Ok(())
}

fn take_input() -> String {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut input = String::new();
    for (i, k) in stdin.keys().enumerate() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(i as u16 + 1, 1),
            termion::cursor::Hide
        )
        .unwrap();
        let s = match k.as_ref().unwrap() {
            Key::Char('\n') => break,
            Key::Char('l') => return String::from("l"),
            Key::Char('?') => return String::from("?"),
            Key::Char('n') => return String::from("n"),
            Key::Char('a') => return String::from("a"),
            Key::Backspace | Key::Char('u') => return String::from("u"),
            Key::Esc | Key::Char('q') => return String::from("q"),
            Key::Char(c) => {
                println!("{c}");
                *c
            }
            _ => return String::from("h"),
        };
        let s = String::from(s);
        input.push_str(&s[..]);
        if input.len() == 2 {
            return input;
        }
    }
    input
}

fn clear_screen() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    stdout.flush().unwrap();
}
