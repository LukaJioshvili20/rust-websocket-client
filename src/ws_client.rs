use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, Clear, ClearType},
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::io::stdout;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::signal;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Error, Message},
};

fn cleanup_terminal() {
    let mut stdout = stdout();
    disable_raw_mode().unwrap();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
}

async fn handle_input<S>(mut write: S)
where
    S: SinkExt<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("\nReceived CTRL+C. Sending close handshake...");
            let _ = write.send(Message::Close(None)).await;
            std::process::exit(0);
        }

        _ = async {
            while let Ok(Some(line)) = reader.next_line().await {
                if line == "/exit" {
                    println!("Exiting... Sending close handshake.");
                    let _ = write.send(Message::Close(None)).await;
                    std::process::exit(0);
                }

                if let Err(e) = write.send(Message::Text(line.into())).await {
                    eprintln!("Failed to send message: {:?}", e);
                    break;
                }
            }
        } => {}
    }
}

async fn handle_output(mut read: impl StreamExt<Item = Result<Message, Error>> + Unpin) {
    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => println!("{}", text),
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        }
    }
}

pub async fn connect_to_websocket(path: &str) -> Result<(), Error> {
    let url = format!("ws://0.0.0.0:8765{}", path);

    cleanup_terminal();

    println!("Connecting to {}...", url);

    match connect_async(&url).await {
        Ok((ws_stream, _)) => {
            println!("Connected (press CTRL+C to quit):");

            let (write, read) = ws_stream.split();

            let input_task = tokio::spawn(handle_input(write));
            let output_task = tokio::spawn(handle_output(read));

            let _ = tokio::try_join!(input_task, output_task);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
