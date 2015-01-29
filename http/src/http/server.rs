#![allow(unstable)]

use std::io::{TcpListener, TcpStream};
use std::str;

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


fn read_to_crlf(stream: &mut TcpStream) -> String{
    const BUFFER_SIZE: usize = 4096;
    let mut buffer: &mut [u8; BUFFER_SIZE] = &mut [0; BUFFER_SIZE];
    let n = stream.read(buffer).unwrap();

    if n >= 4096 {
        panic!("Buffer full!")
    }else{
        println!("read {} bytes", n)
    }

    let s = String::from_str(str::from_utf8(buffer).unwrap());
    println!("------MESSAGE------");
    println!(s);
    println!("--------END--------");
    s
}

