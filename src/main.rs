use solitaire::{
    self,
    core::{Error, moves::Move},
};
use std::io::{Result, Write, stdin, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn main() -> Result<()> {
    let mut game = solitaire::setup();
    while !game.is_over() {
        clear_screen()?;
        println!("\n{game}");
        solitaire::print_table(game.table());
        let input = take_input()?;
        let next_move = Move::from(input);
        if let Err(e) = game.play(next_move) {
            match e.downcast::<Error>() {
                Ok(err) => match err.as_ref() {
                    Error::Quit => {
                        clear_screen()?;
                        return Ok(());
                    }
                    Error::Help => {
                        clear_screen()?;
                        solitaire::print_help();
                        println!("Press Enter to continue");
                    }
                    Error::History => {
                        clear_screen()?;
                        game.print_history();
                        println!("Press Enter to continue");
                    }
                    Error::Win => break,
                    _ => {
                        let mut stdout = stdout().into_raw_mode()?;
                        let err = ansi_term::Colour::Red.paint(err.to_string());
                        write!(stdout, "{}{}", termion::cursor::Goto(1, 1), err)?;
                        stdout.flush()?;
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
    clear_screen()?;
    let mut stdout = stdout().into_raw_mode()?;
    writeln!(
        stdout,
        "{}You Won!{}",
        termion::cursor::Show,
        termion::cursor::Goto(1, 2)
    )?;
    println!("{game}");
    Ok(())
}

fn take_input() -> Result<String> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;
    let mut input = String::new();
    for (i, k) in stdin.keys().enumerate() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(i as u16 + 1, 1),
            termion::cursor::Hide
        )?;
        let s = match k? {
            Key::Char('\n') => break,
            Key::Char('l') => return Ok(String::from("L")),
            Key::Char('?') => return Ok(String::from("?")),
            Key::Char('n') => return Ok(String::from("N")),
            Key::Char('a') => return Ok(String::from("A")),
            Key::Backspace | Key::Char('u') => return Ok(String::from("U")),
            Key::Esc | Key::Char('q') => return Ok(String::from("Q")),
            Key::Char(c) => {
                println!("{}", c.to_ascii_uppercase());
                c
            }
            _ => return Ok(String::from("?")),
        };
        let s = String::from(s);
        input.push_str(&s[..]);
        if input.len() == 2 {
            return Ok(input);
        }
    }
    Ok(input)
}

fn clear_screen() -> Result<()> {
    let mut stdout = stdout().into_raw_mode()?;
    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )?;
    stdout.flush()
}
