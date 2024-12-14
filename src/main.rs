use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs,
    path::Path,
    time::Instant,
};

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    bind_to: String,
    html_path: String,
    not_found_path: String,
    debug: bool,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            bind_to: "127.0.0.1:8080".to_string(),
            html_path: "html".to_string(),
            not_found_path: "404.html".to_string(),
            debug: false,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: Config = confy::load("easy-http", None)?;
    let listener = TcpListener::bind(&cfg.bind_to).expect("Failed to bind to IP!");

    for stream in listener.incoming() {
        let now = Instant::now();

        let stream = stream.expect("Could not handle connection!");
        handle_connection(stream, &cfg);

        let elapsed_time = now.elapsed();
        if cfg.debug {
            println!("Request took {:#?}", elapsed_time);
        };
    }
    
    Ok(())
}

fn handle_connection(mut stream: TcpStream, cfg: &Config) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    let uri = if parts.len() > 1 { parts[1] } else { "/" };

    let (status_line, filename) = if uri == "/" {
        ("HTTP/1.1 200 OK", format!("{}/index.html", cfg.html_path))
    } else {
        let path = format!("{}{}", cfg.html_path, uri);
        if Path::new(&path).exists() {
            ("HTTP/1.1 200 OK", path)
        } else {
            ("HTTP/1.1 404 NOT FOUND", cfg.not_found_path.clone())
        }
    };

    let contents = fs::read_to_string(filename).expect("Failed to read file!");
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("Failed to write to stream!");
}

