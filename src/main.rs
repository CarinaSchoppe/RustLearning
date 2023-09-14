use std::env::var;
use std::fmt::Error;
use std::fs::read;
use std::io::{stdin, Read, Write};
use std::iter::Flatten;
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread::{sleep, spawn};

struct Server {
    name: String,
    port: i32,
    ip: String,
    socket_server: TcpListener,
}

impl Server {
    fn new(name: &str, ip: &str) -> Server {
        let mut port = 8080;
        Server {
            name: String::from(name.trim()),
            port,
            socket_server: TcpListener::bind(format!("{}:{}", &ip, port)).unwrap(),
            ip: String::from(ip),
        }
    }

    fn handle_connection(&self) -> Result<(), Error> {
        for stream in self.socket_server.incoming() {
            println!("Connection established!");
            match stream {
                Ok(stream) => {
                    println!("New client!");
                    self.handle_incoming_connections(stream);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Ok(())
    }

    fn handle_incoming_connections(&self, mut stream: TcpStream) -> Result<(), Error> {
        let mut stream_copy = stream.try_clone().unwrap();
        spawn(move || loop {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            stream_copy.write(input.as_bytes()).unwrap();
            stream_copy.flush().unwrap();
        });
       'read_loop: loop {
            let mut buffer = [0; 1024];
            //read from stream but close connection and stop reading and dont throw exception
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Response Client: {}", String::from_utf8_lossy(&buffer[..]));
                }
                Err(e) => {
                    println!("Error: {}", e);
                    return Ok(());
                }
            }


            println!("Request Client: {}", String::from_utf8_lossy(&buffer[..]));
            stream.flush().unwrap();
        }
    }
}

struct Client {
    name: String,
    ip: String,
    port: i32,
    socket_client: TcpStream,
}

impl Client {
    fn new(name: &str, ip: &str, port: i32) -> Client {
        Client {
            name: String::from(name),
            ip: String::from(ip),
            port,
            socket_client: TcpStream::connect(format!("{}:{}", ip, port)).unwrap(),
        }
    }
    fn start(&mut self) {
        //print connected when socket is connected
        println!("Connected!");
        let mut socket_client = self.socket_client.try_clone().unwrap();
        spawn(move || loop {
            let mut buffer = [0; 1024];
            socket_client.read(&mut buffer).unwrap();
            println!("Response Server: {}", String::from_utf8_lossy(&buffer[..]));
        });

        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            self.socket_client.write(input.as_bytes()).unwrap();
            self.socket_client.flush().unwrap();
        }
    }
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 1 {
        println!("Usage: {} <server|client>1", args[0]);

        return;
    } else {
        println!("{} {}", args[0], args.len().to_string());
    }

    let command = args[1].trim();

    match command {
        "server" => {
            println!("Starting server...");
            let mut server = Server::new("Test", "127.0.0.1");
            server.handle_connection();
        }
        "client" => {
            println!("Starting client...");

            let mut client = Client::new("Test Client 1", "127.0.0.1", 8080);
            client.start();
        }
        _ => {
            println!("Usage: {} <server|client>", args[0]);
            return;
        }
    }
}
