use std::{io::Write, net::TcpStream};

use super::http_type::HttpType;

pub fn http_response<T: std::fmt::Display>(mut socket: TcpStream, status: HttpType, body: T) {
    let _ = socket.write_all(
        format!(
            "HTTP/1.1 {}\r\nContent-Type: application/json\r\n\r\n{}",
            status as u32, body
        )
        .as_bytes(),
    );
}
