#![allow(unstable)]

use std::io::{TcpListener, TcpStream};
use std::str;

use std::collections::HashMap;

use utils;

pub fn print_stuff() {
    println!("module fuck yeah")
}

pub fn get_http_stream() -> TcpListener {
    TcpListener::bind("127.0.0.1:4000").unwrap()
}

pub fn new_conn(mut stream: TcpStream) {
    let msg: String = read_to_crlf(&mut stream);

    let (rqline, headers, data) = split_request(msg.as_slice());

    println!("Request line:");
    println!("{}", rqline);
    println!("Headers:");
    println!("{}", headers);
    println!("Data:");
    println!("{}", data);
    let hdr_map = parse_headers(headers);


    let hdr = format!("HTTP/1.1 200 OK\r\n\
                       halla gutta \r\n\
                       hue\r\n\r\n");
    stream.write_str(hdr.as_slice());
    stream.write_str("halla gutta hva skjer a?\r\n");
}

/// Takes a HTTP request as parameter and splits it into
/// (request line, headers, data).
fn split_request(string: &str) -> (&str, &str, &str) {
    let nl_index = utils::contains(string, '\n').unwrap();
    let requestline: &str = &string[..nl_index];
    let mut it = string[nl_index+1..].split_str("\r\n\r\n");
    let headers = it.next().unwrap();
    let data = it.next().unwrap();
    (requestline, headers, data)
}

fn read_to_crlf(stream: &mut TcpStream) -> String {
    const BUFFER_SIZE: usize = 4096;
    let mut buffer: &mut [u8; BUFFER_SIZE] = &mut [0; BUFFER_SIZE];
    let n = stream.read(buffer).unwrap();

    if n >= 4096 {
        panic!("Buffer full!")
    } else {
        println!("read {} bytes", n)
    }

    let s: &str = str::from_utf8(buffer).unwrap();
    String::from_str(s)
}

fn parse_headers(headers: &str) -> HashMap<&str, &str> {
   let mut keystore = HashMap::new(); 
   for l in headers.lines() {
        let i: Option<usize> = utils::contains(l, ':');
        match i {
            Some(n) => {
                let k = l[..n].trim();
                let v = l[n+1..].trim();
                keystore.insert(k, v);
            },
            None(..) => {}
        }
   }
   keystore
}
