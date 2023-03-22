use colored::*;
use std::io;
use std::process::exit;
use ansi_escapes;

use tokio::{
    io::{stdin, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpStream, ToSocketAddrs},
    runtime::Runtime,
    time::{sleep, Duration}
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

struct User {
    username: String,
    password: String,
}

pub(crate) fn main() -> Result<()> {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        try_main("127.0.0.1:8080").await;
    });

    Ok(())
}

async fn try_main(addr: impl ToSocketAddrs) -> Result<()> {
    let mut login_username = String::new();
    let mut password = String::new();

    println!("Enter your username : ");
    io::stdin()
        .read_line(&mut login_username)
        .expect("Failed to validate user");

    println!("Enter your password : ");
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to provide password");

    if password.trim() != "password" {
        println!("Invalid password, session terminated!");
        exit(0)
    }

    let username_length = login_username.len();
    login_username.truncate(username_length - 1);

    let user = User {
        username: login_username,
        password: password,
    };

    println!("User Authenticated - Logging in as: {} ...", user.username);

    sleep(Duration::from_secs(2)).await;
    
    // Clear all of terminal output - cross-platform
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("***************** New Session *****************");

    let mut stream = TcpStream::connect(addr).await?;
    let (read, mut writer) = stream.split();
    let reader = BufReader::new(read);

    let mut lines_from_server = reader.lines();

    let stdin = BufReader::new(stdin());

    let mut lines_from_stdin = stdin.lines();

    loop {
        tokio::select! {
            line = lines_from_server.next_line() => match line {
                Ok(Some(line)) => {
                    println!("{}", line.bright_blue().bold());
                },
                Ok(None) => break,
                Err(_) => break
            },
            line = lines_from_stdin.next_line() => match line {
                Ok(Some(line)) => {
                    // Send message to server
                    let mut formatted_message_server = String::from(user.username.clone());
                    formatted_message_server.push_str(": ");
                    formatted_message_server.push_str(&line);

                    // Log message in own terminal
                    let mut formatted_message_self = String::from("You: ");
                    formatted_message_self.push_str(&line);

                    print!("{}", ansi_escapes::EraseLines(2));
                    println!("{}", formatted_message_self.green());

                    writer.write_all(formatted_message_server.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                }
                Ok(None) => break,
                Err(_) => break
            }
        }
    }
    Ok(())
}
