use std::{io::{Read, Result, Write}, net::{TcpListener, TcpStream}};
use chrono;

fn generate_response(method: &str, path: &str) -> String {
    let current_date = chrono::Local::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
    let response_body = handle_methods(method, path);
    let head = match response_body {
        "<h1>Not Found</h1>" => "404 Not Found",
        _ => "200 OK",
    };  
    let response = format!(
        "HTTP/1.1 {}\r\n\
        Date: {}\r\n\
        Connection: close\r\n\
        Content-Length: {}\r\n\
        Content-Type: text/html; charset=UTF-8\r\n\
        \r\n\
        {}",
        head,   
        current_date,
        response_body.len(),
        response_body
    );
    response
}

fn handle_methods(method: &str, path: &str) -> &'static str {
    match (method, path) {
        ("GET", "/") => "<h1>Hello, World!</h1>",
        ("GET", "/about") => "<h1>About</h1>",
        _ => "<h1>Not Found</h1>",
    }
}

fn parse_request(request: &str) -> (&str, &str) {
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap();
    let path = parts.next().unwrap();
    (method, path)
}   

fn handle_conection(mut stream: &TcpStream) -> Result<()> {
    println!("Incoming connection from: {}", stream.peer_addr()?);  

    let mut buffer = [0; 1024];

    stream.read(&mut buffer)?;

    println!("{}", std::str::from_utf8(&buffer).unwrap());
    match std::str::from_utf8(&buffer){
        Ok(request) => {
            let (method, path) = parse_request(request);
            let response = generate_response(method, path);         
            stream.write_all(response.as_bytes())?;
        }
        Err(e) => eprintln!("Failed to parse request: {}", e),
    }

    Ok(())
}   

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        match stream {
            Ok(stream) => handle_conection(&stream)?,
            Err(e) => println!("Failed to establish a connection: {}", e),
        }
    }
    Ok(())
}
