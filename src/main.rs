use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{stdout, Result, Write};

fn print_menu(options: &[(&str, &str)], selected: usize) -> Result<()> {
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

fn handle_selection(options: &[(&str, &str)], selected: usize) -> Result<()> {
    let mut stdout = stdout();

    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        cursor::Show
    )?;

    print!("✅ You selected: {}\r\n\r\n", options[selected].0);
    print!("{}\r\n\r\n", options[selected].1);
    print!("Press Enter to exit... \r\n");
    stdout.flush()?;

    loop {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Enter {
                break;
            }
        }
    }

    Ok(())
}

fn handle_exit() -> Result<()> {
    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    println!("❌ No option selected. Exiting...");
    Ok(())
}

fn handle_input(selected: &mut usize, options_len: usize) -> Option<&'static str> {
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

fn main() -> Result<()> {
    enable_raw_mode()?;
    let options = vec![
        ("/echo", "Echo Path: Sends back any message you send."),
        ("/math", "Math Path: Performs simple math calculations."),
        (
            "/global-chat",
            "Global Chat: Connects you to a public chatroom.",
        ),
    ];

    let mut selected = 0;

    loop {
        print_menu(&options, selected)?;

        match handle_input(&mut selected, options.len()) {
            Some("Enter") => {
                handle_selection(&options, selected)?;
                break;
            }
            Some("Esc") => {
                handle_exit()?;
                break;
            }
            _ => {}
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), cursor::Show)?;
    Ok(())
}
