# rust-websocket-client

A terminal-based WebSocket client built in Rust. This application allows you to connect to WebSocket endpoints, send messages interactively, and receive real-time responses.

## Features

- Supports connecting to multiple WebSocket paths:
  - `/echo` – Sends back any message you send.
  - `/math` – Processes simple mathematical expressions.
  - `/global-chat` – Connects to a public chatroom.
- Clean terminal interface with real-time message display.
- Interactive input/output, replicating `wscat` behavior.
- Graceful connection handling and termination with `/exit` command.

---

## Prerequisites

To build and run the client, you need:

- **Rust** (latest stable version) installed.  
  If Rust is not installed, follow the [Rust installation guide](https://www.rust-lang.org/learn/get-started).
- Rust Websocket Server running.

## Rust Websocket Server

1. Clone the repository:

   ```bash
   git clone https://github.com/LukaJioshvili20/rust-websocket-server.git
   cd rust-websocket-server
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Run the WebSocket Server:

   ```bash
   cargo run
   ```

---

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/LukaJioshvili20/rust-websocket-client.git
   cd rust-websocket-client
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Run the WebSocket client:

   ```bash
   cargo run
   ```

---

## Usage

1. **Start the WebSocket client**:

   ```bash
   cargo run
   ```

2. **Select a WebSocket Path**:
   Use the arrow keys to navigate and Enter to select:

   ```
   --- Select a WebSocket Path ---
   > /echo
     /math
     /global-chat
   ```

3. **Interact**:
   - After connecting, type your message and press Enter to send.
   - Received messages are displayed in real-time.
   - Type `/exit` to gracefully close the connection.

---

## Example

### `/echo` Path

```plaintext
Connecting to ws://0.0.0.0:8765/echo...

Connected (press CTRL+C to quit):

hello
hello
this is a test
this is a test
/exiting
Exiting...
```

### `/math` Path

```plaintext
Connecting to ws://0.0.0.0:8765/math...

Connected (press CTRL+C to quit):

10 * 2
20
5 + 5
10
```

### `/global-chat` Path

```plaintext
Connecting to ws://0.0.0.0:8765/global-chat...

Connected (press CTRL+C to quit):

hello from luka
Hi there!
```

---

## Code Structure

The project is organized as follows:

```
.
├── Cargo.toml        # Project dependencies and metadata
├── src
│   ├── main.rs       # Entry point: menu and WebSocket path selection
│   └── ws_client.rs  # WebSocket communication logic
└── README.md         # This file
```

---

## Dependencies

The project uses the following libraries:

- **tokio**: Async runtime for Rust.
- **tokio-tungstenite**: WebSocket client implementation.
- **futures**: Utilities for streams and sinks.
- **crossterm**: Terminal interface handling.

---

## Contributing

Contributions are welcome! If you'd like to improve or extend the functionality, feel free to submit a PR or open an issue.

---

## License

This project is licensed under the **MIT License**. See [LICENSE](LICENSE) for details.
