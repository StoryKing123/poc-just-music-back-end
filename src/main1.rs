use async_std::io::ReadExt;
use async_std::io::WriteExt;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use futures::stream::StreamExt;
use std::fs;
use std::io::prelude::*;

use song::get_song_url;
mod song;

// #[tokio::main]
// async fn main() {
//     // get_song_url(String::from("美人鱼"), String::from("林俊杰")).await;
//     let url_res = get_song_url(String::from("美人鱼"), String::from("林俊杰"));
//     println!("{}", url_res.await.unwrap());
// }

// #[async_std::main]
#[tokio::main]
async fn main() {
   
    // let listener = TcpListener::bind("127.0.0.1:3002").await.unwrap();

    // listener
    //     .incoming()
    //     .for_each_concurrent(None, |tcpstream| async move {
    //         let tcpstream = tcpstream.unwrap();
    //         handle_connection(tcpstream).await;
    //     })
    //     .await;
    // for stream in listener.incoming() {
    // let stream = stream.unwrap();
    //
    // handle_connection(stream);
    // }
}

// --snip--

async fn handle_connection(mut stream: TcpStream) {
    // --snip--
    println!("handle connection");

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    // let (status_line, filename) = if buffer.starts_with(get) {
    //     ("HTTP/1.1 200 OK", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };

    // let contents = fs::read_to_string(filename).unwrap();

    // let response = format!(
    //     "{}\r\nContent-Length: {}\r\n\r\n{}",
    //     status_line,
    //     contents.len(),
    //     contents
    // );
    let url_res = get_song_url(String::from("美人鱼"), String::from("林俊杰"));
    println!("{}", url_res.await.unwrap());
    let (status_line, filename) = ("HTTP/1.1 200 OK", "hello.html");
    let response = format!("{}\r\n\r\n{}", status_line, "32131");

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
