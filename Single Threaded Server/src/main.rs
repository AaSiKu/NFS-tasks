use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut http_request = String::new();

    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line).unwrap();
        if line.trim().is_empty() {
            break;
        }
        http_request.push_str(&line);
    }

    // println!("HTTP header: {:#?}", http_request); // Uncomment this line to see the request
    let request_line = http_request.lines().next().unwrap();
    // println!("Request Line: {:#?}", request_line); // Uncomment this line to see the request line
    let (mut status_line, filename) =
        if request_line == "GET / HTTP/1.1" || request_line == "GET /index.html HTTP/1.1" {
            ("HTTP/1.1 200 OK", "index.html")
        } else if request_line == "GET /Services HTTP/1.1" {
            ("HTTP/1.1 200 OK", "services.html")
        } else if request_line == "GET /Contact HTTP/1.1" {
            ("HTTP/1.1 200 OK", "Contact.html")
        } else if request_line == "GET /About HTTP/1.1" {
            ("HTTP/1.1 200 OK", "About.html")
        } else if request_line == "GET /Services HTTP/1.1" {
            ("HTTP/1.1 200 OK", "services.html")
        } else if request_line == "GET /Login HTTP/1.1" {
            ("HTTP/1.1 200 OK", "login.html")
        } else if request_line == "GET /Random HTTP/1.1" {
            ("HTTP/1.1 200 OK", "random.html")
        } else if !request_line.contains("GET") {
            ("HTTP/1.1 405 METHOD NOT ALLOWED", "405.html")
        }
        // I tried to implement the POST method but it didn't work!
        // else if request_line == "POST /login HTTP/1.1"{
        //     let mut username = String::new();
        //     let mut password = String::new();
        //     username = credentials.split("&").collect::<Vec<&str>>()[0].split("=").collect::<Vec<&str>>()[1].to_string();
        //     password = credentials.split("&").collect::<Vec<&str>>()[1].split("=").collect::<Vec<&str>>()[1].to_string();
        //     println!("Username: {:#?}", username);
        //     println!("Password: {:#?}", password);
        //     ("HTTP/1.1 200 OK", "login.html")
        // }
        else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            status_line = "HTTP/1.1 500 INTERNAL SERVER ERROR";
            match fs::read_to_string("500.html") {
                Ok(contents) => contents,
                Err(_) => "Internal Server Error".to_string(),
            }
        }
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
