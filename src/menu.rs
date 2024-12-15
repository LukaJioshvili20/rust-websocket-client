use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Result, Write};

pub fn print_menu(options: &[(&str, &str)], selected: usize) -> Result<()> {
    execute!(stdout(), cursor::MoveTo(0, 0), Clear(ClearType::All))?;
    println!("--- Select a WebSocket Path ---\r\n");

    for (i, (option, _)) in options.iter().enumerate() {
        execute!(stdout(), cursor::MoveTo(0, (2 + i) as u16))?;
        if i == selected {
            execute!(
                stdout(),
                SetForegroundColor(Color::Black),
                SetBackgroundColor(Color::White),
                Print(format!("> {}", option)),
                ResetColor
            )?;
        } else {
            println!("  {}", option);
        }
    }

    execute!(stdout(), cursor::MoveTo(0, (2 + options.len() + 1) as u16))?;
    print!("{}", options[selected].1);

    stdout().flush()
}

pub fn handle_input(selected: &mut usize, options_len: usize) -> Option<&'static str> {
    if let Event::Key(key) = event::read().unwrap() {
        match key.code {
            KeyCode::Up => {
                if *selected > 0 {
                    *selected -= 1;
                }
            }
            KeyCode::Down => {
                if *selected < options_len - 1 {
                    *selected += 1;
                }
            }
            KeyCode::Enter => return Some("Enter"),
            KeyCode::Esc => return Some("Esc"),
            _ => {}
        }
    }
    None
}
