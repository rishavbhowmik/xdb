use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         handle_connection(stream);
//     }
// }

fn handle_connection(buf_reader: BufReader<&mut TcpStream>) -> std::string::String {
    buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect()
}

use std::sync::mpsc::{Sender};

pub fn start_server(req_stream: Sender<Vec<u8>>) {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let buf_reader = BufReader::new(&mut stream);
        let body = handle_connection(buf_reader);

        req_stream.send(body.as_bytes().to_vec());
        let status_line = "HTTP/1.1 200 OK";
        let contents = "All Ok";
        let length = contents.len();
        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
}
