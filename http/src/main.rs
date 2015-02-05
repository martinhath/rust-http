#![feature(io)]
#![feature(core)]

extern crate http;

use std::old_io::{Listener, Acceptor};
use std::thread::Thread;

fn main() {
    let stream = http::server::get_http_stream();
    let mut listener = stream.listen().unwrap();

    for strm in listener.incoming() {
        match strm {
            Err(..) => {},
            Ok(s)   => {
                Thread::spawn(move || {
                    http::server::new_conn(s.clone());
                });
            }

        }
    }
    drop(listener)
}
