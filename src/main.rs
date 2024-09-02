use std::{
    io::{BufRead, BufReader, Read},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

use controllers::{statement_controller, transaction_controller};
use dotenv::dotenv;
use http_status::{http_response::http_response, http_type::HttpType};
use persistence::database::Database;
use queue::Queue;
use serde_json::{json, Value};

pub mod controllers;
pub mod dto;
pub mod http_status;
pub mod model;
pub mod persistence;
pub mod queue;
pub mod repositories;

fn id_is_number(url: &str) -> Option<u32> {
    let start_id = url.find("/clientes/").unwrap();

    let id_start = start_id + "/clientes/".len();

    let id_end = url[id_start..].find('/').unwrap();
    let id_str = &url[id_start..id_start + id_end];

    match id_str.parse::<u32>() {
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

fn core(socket: TcpStream, db: Arc<Queue<Database>>) {
    let mut reader = BufReader::new(&socket);

    let mut headline: String = String::new();
    let _ = reader.read_line(&mut headline);

    let content_length: u64 = get_content_length(&mut reader);

    let mut request_body = String::new();
    if content_length > 0 {
        let _ = reader
            .take(content_length)
            .read_to_string(&mut request_body);
    }

    let mut status: HttpType = HttpType::BadRequest;
    let mut response_body: Value = json!({"message": "Bad Request"});

    match headline.as_str() {
        url if url.contains("GET") => {
            let (response_status, response_json) = statement_controller::controller(url, db);

            status = response_status;
            response_body = response_json;
        }
        url if url.contains("POST") => {
            let (response_status, response_json) =
                transaction_controller::controller(url, request_body, db);

            status = response_status;
            response_body = response_json;
        }
        _ => {}
    };

    http_response(socket, status, response_body);
}

fn main() {
    dotenv().ok();

    let listener = TcpListener::bind("0.0.0.0:8080").expect("Couldn't connect to the server...");

    let db_queue = Arc::new(Queue::<Database>::default());
    let client_queue = Arc::new(Queue::<TcpStream>::default());

    (0..10).for_each(|_| {
        db_queue.push_front(Database::new().unwrap());
    });

    (0..8).for_each(|_| {
        let client_pool = Arc::clone(&client_queue);
        let db_pool = Arc::clone(&db_queue);

        thread::spawn(move || loop {
            core(client_pool.pop_back(), db_pool.clone());
        });
    });

    for stream in listener.incoming() {
        client_queue.push_front(stream.unwrap());
    }
}
