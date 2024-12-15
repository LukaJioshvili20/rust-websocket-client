mod menu;
mod ws_client;

use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use menu::{handle_input, print_menu};
use std::io::{stdout, Result};
use ws_client::connect_to_websocket;

#[tokio::main]
async fn main() -> Result<()> {
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
                execute!(stdout(), cursor::MoveTo(0, 0))?;
                if let Err(e) = connect_to_websocket(options[selected].0).await {
                    eprintln!("❌ WebSocket Error: {}", e);
                }
                break;
            }
            Some("Esc") => {
                println!("\n❌ No option selected. Exiting...");
                break;
            }
            _ => {}
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), cursor::Show)?;
    Ok(())
}
