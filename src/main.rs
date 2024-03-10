use std::{
    io::{BufRead, BufReader, Read},
    net::{TcpListener, TcpStream},
};

use controllers::{statement_controller, transaction_controller};
use dotenv::dotenv;
use http_status::{http_response::http_response, http_type::HttpType};
use serde_json::json;

pub mod controllers;
pub mod dto;
pub mod http_status;
pub mod model;
pub mod persistence;
pub mod repositories;

fn id_is_number(url: &str) -> Option<i32> {
    let start_id = url.find("/clientes/").unwrap();

    let id_start = start_id + "/clientes/".len();

    let id_end = url[id_start..].find('/').unwrap();
    let id_str = &url[id_start..id_start + id_end];

    match id_str.parse::<i32>() {
        Ok(id) => Some(id),
        Err(_) => None,
    }
}

fn get_content_length(reader: &mut BufReader<&TcpStream>) -> u64 {
    let mut content_length: u64 = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.contains("Content-Length") {
            let content_length_split: Vec<&str> = line.split(':').collect();
            content_length = content_length_split
                .get(1)
                .unwrap()
                .trim()
                .parse::<u64>()
                .unwrap();
        }

        if line.is_empty() {
            break;
        }
    }

    content_length
}

fn main() {
    dotenv().ok();

    let listener = TcpListener::bind("0.0.0.0:8080").expect("Couldn't connect to the server...");

    for stream in listener.incoming() {
        let socket = stream.unwrap();
        let mut reader = BufReader::new(&socket);

        let mut headline: String = String::new();
        let _ = reader.read_line(&mut headline);

        let content_length: u64 = get_content_length(&mut reader);

        let mut body = String::new();
        if content_length > 0 {
            let _ = reader.take(content_length).read_to_string(&mut body);
        }

        match headline.as_str() {
            url if url.contains("GET") => statement_controller::controller(url, socket),
            url if url.contains("POST") => transaction_controller::controller(url, body, socket),
            _ => http_response(
                socket,
                HttpType::BadRequest,
                json!({"message": "Bad Request"}),
            ),
        };
    }
}
