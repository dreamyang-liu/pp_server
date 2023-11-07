mod worker_pool_file;
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;

use worker_pool_file::worker_pool::{WorkerPool, WorkerPoolAbstract};

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
            },
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream, worker_pool: &mut WorkerPool) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    match stream.write(response) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
    let acquire_result = worker_pool.acquire_worker();
    if acquire_result.is_ok() {
        println!("Acquired worker on port {}", acquire_result.unwrap());
    } else {
        println!("Failed to acquire worker: {}", acquire_result.err().unwrap());
    }
}

fn handle_client(stream: TcpStream, worker_pool: &mut WorkerPool) {
    handle_read(&stream);
    handle_write(stream, worker_pool);
}

fn main() {
    let mut uds_worker_pool = WorkerPool {
        workload_limit: 3,
        worker_pool_capacity: 10,
        worker_path: "/Users/lmy/Projects/Programming Lanaguage/rust/pp_server/httpserver".to_string(),
        workers_list: std::collections::LinkedList::new(),
    };
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening for connections on port {}", 8080);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, &mut uds_worker_pool);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}