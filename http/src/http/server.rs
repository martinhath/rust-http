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
    println!("{}", msg)
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
    let string = String::from_str(s);
    println!("------MESSAGE------");
    println!("{}", s);
    println!("--------END--------\n\n");

    let headers = parse_headers(s);
    for (k, v) in headers.iter() {
        println!("Key: [{}]\tValue: [{}]", k, v)
    }

    string
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
