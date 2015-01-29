#![allow(unstable)]

use std::io::TcpListener;

pub fn print_stuff() {
    println!("module fuck yeah")
}

pub fn get_http_stream() -> TcpListener {
    TcpListener::bind("127.0.0.1:4000").unwrap()
}

pub fn new_conn() {
    println!("halla gutta")
}

