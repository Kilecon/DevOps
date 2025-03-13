use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn get_port() -> String {
    env::var("PING_LISTEN_PORT").unwrap_or_else(|_| "8080".to_string()) // Port par défaut 8080
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // Buffer de lecture
    if let Ok(size) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..size]);
        
        // Vérifier si la requête est GET /ping
        if request.starts_with("GET /ping ") {
            let headers: Vec<&str> = request.split("\r\n").collect();
            let json_headers: String = format!(
                "{{\"headers\": [{}]}}",
                headers.iter().skip(1).filter(|h| !h.is_empty()).map(|h| format!("\"{}\"", h)).collect::<Vec<String>>().join(", ")
            );

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                json_headers.len(),
                json_headers
            );
            stream.write_all(response.as_bytes()).unwrap();
        } else {
            // Réponse 404 pour toute autre requête
            let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

fn main() {
    let port = get_port();
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).expect("Impossible de lier le port");

    println!("Serveur en écoute sur le port {}", port);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_client(stream);
        }
    }
}
