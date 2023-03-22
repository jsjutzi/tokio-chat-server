type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) fn main() -> Result<()> {
    println!("main");
    Ok(())
}

// async fn try_main(addr: &str) -> Result<()> {
//     let mut login_username = String::new();
//     let mut password = String::new();

//     println!("Enter your username : ");
//     io::stdin()
//         .read_line(&mut login_username)
//         .expect("Failed to validate user");

//     println!("Enter your password : ");
//     io::stdin()
//         .read_line(&mut password)
//         .expect("Failed to provide password");

//     if password.trim() != "password" {
//         println!("Invalid password, session terminated!");
//         exit(0)
//     }

//     let user = User {
//         username: login_username,
//         password: password
//     };

//     println!("Logged in as : {}", user.username);

//     let stream = TcpStream::connect(addr).await?;
//     let (reader, mut writer) = (&stream, &stream);
//     let reader = BufReader::new(reader);
//     let mut lines_from_server = futures::StreamExt::fuse(reader.lines());

//     let stdin = BufReader::new(stdin());
    
//     writer.write_all(user.username.as_bytes()).await?;
    

//     let mut lines_from_stdin = futures::StreamExt::fuse(stdin.lines());


//     loop {
//         select! {
//             line = lines_from_server.next().fuse() => match line {
//                 Some(line) => {
//                     let line = line?;
//                     println!("{}", line);
//                 },
//                 None => break,
//             },
//             line = lines_from_stdin.next().fuse() => match line {
//                 Some(line) => {
//                     let line = line?;
//                     writer.write_all(line.as_bytes()).await?;
//                     writer.write_all(b"\n").await?;
//                 }
//                 None => break,
//             }
//         }
//     }
//     Ok(())
// }