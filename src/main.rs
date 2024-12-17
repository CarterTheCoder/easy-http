use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs,
    path::Path,
    time::Instant,
    thread,
};

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    bind_to: String,
    html_path: String,
    not_found_path: String,
    debug: bool,
}

struct AppInfo {
    name: String,
    version: String,
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
    let info = AppInfo {
        name: String::from("easy-http"),
        version: String::from("0.1.1"),
    };
    println!("Welcome to {} version {}!", info.name, info.version);

    let cfg: Config = confy::load("easy-http", None)?;
    println!("Listening on {}", cfg.bind_to);
    if cfg.debug {
        println!("Debug mode is enabled! HTTP requests will be timed.");
    }

    let listener = TcpListener::bind(&cfg.bind_to).expect("Failed to bind to IP!");

    for stream in listener.incoming() {
        let now = Instant::now();
        let builder = thread::Builder::new()
            .name("conn_handler".to_string()
        );
        let stream = stream.expect("Could not handle connection!");
        let cfg_clone = cfg.clone();

        builder.spawn(move || {
            handle_connection(stream, &cfg_clone);
        }).expect("Failed to create new thread!");

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
    let not_found_path = cfg.html_path.clone() + "404.html";

    let (status_line, filename) = if uri == "/" {
        ("HTTP/1.1 200 OK", format!("{}/index.html", cfg.html_path))
    } else {
        let path = format!("{}{}", cfg.html_path, uri);
        if Path::new(&path).exists() {
            ("HTTP/1.1 200 OK", path)
        } else {
            ("HTTP/1.1 404 NOT FOUND", not_found_path.clone())
        }
    };

    let contents = fs::read_to_string(filename).expect("Failed to read file!");
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("Failed to write to stream!");
}

