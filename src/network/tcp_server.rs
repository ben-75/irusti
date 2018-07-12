use std::io::Write;
use std::net::TcpListener;
use std::thread;
use scoped_pool::Pool;

use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;

pub fn start_tcp_server(tcp_port :i32) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}",tcp_port)).unwrap();
    debug!("TCP server starting on port {}", tcp_port);
//    for stream in listener.incoming() {
//        let stream = stream.unwrap();
//
//        handle_connection(stream);
//    }

    let pool = Pool::new(4);

    pool.scoped(|scope| {
        for stream in listener.incoming() {
            match stream{
                Ok(stream)=>{
                    scope.execute(move || handle_connection(stream));
                },
                Err(_)=>{
                    break;
                }
            }


        }
    });

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let get_sleep = b"GET /sleep";
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    if buffer.starts_with(get_sleep) {
        thread::sleep(Duration::from_secs(5));

        let response = "HTTP/1.1 200 OK\r\n\r\nHello Rust after 5";

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }else{
        let response = "HTTP/1.1 200 OK\r\n\r\nHello Rust";

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }


}