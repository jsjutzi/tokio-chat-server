# RoosterAPI

A Rust chat server.

You'll need to install:

- [Rust](https://www.rust-lang.org/tools/install)

Local Setup:

Launch Server:

```bash
cargo run server
```

Launch Client:

```bash
cargo run client
```

To send a message, just type and hit the Enter key. This chat server operates on a shared broadcast. You should see on receiving terminals format of

```bash
[your_username]: [message]
```

On your sending terminal you will see your own messages in the format of:

```bash
You: [message]
```

Color coding is implemented for improved readability.
