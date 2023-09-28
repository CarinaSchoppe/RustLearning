use std::io::{Read, stdin, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

struct Client {
    socket: TcpStream,
    ip: String,
    port: i32,
}

impl Client {}

impl Client {
    fn new(socket: TcpStream, ip: String) -> Client {
        return Client {
            socket: socket,
            ip: ip,
            port: 8080,
        };
    }

    fn start(&mut self) {
        let mut buffer = [0; 8196];
        loop {

            //print the message from the socket to the console
            match self.socket.read(&mut buffer) {
                Ok(_) => {
                    println!("{}",
                             String::from_utf8_lossy(&buffer[..]));
                }
                Err(error) => {
                    println!("Error: Could not read from socket");
                }
            }
        }
    }
}


struct Server {
    socket_server: Option<TcpListener>,
    client_list: Vec<Client>,
    ip: String,
    port: i32,
}

impl Server {
    fn new(ip: String, port: i32) -> Server {
        return Server {
            socket_server: None,
            client_list: Vec::new(),
            ip: ip,
            port: port,
        };
    }

    fn start(&mut self) {
        let socket_server = match TcpListener::bind(format!("{}:{}", self.ip, self.port)) {
            Ok(socket) => {
                println!("Server started on port {}", self.port);
                Ok(socket)
            }
            Err(error) => {
                println!("Error: Could not bind to address");
                Err(error)
            }
        };

        self.socket_server = socket_server.ok();
    }

    fn handle_incoming_connection(&self) {
        for socket in self.socket_server.as_ref().unwrap().incoming() {
            match socket {
                Ok(socket) => {
                    let mut socket_clone = socket.try_clone();
                    match socket_clone {
                        Ok(socket_clone) => {
                            self.handle_input(socket_clone);
                        }
                        Err(error) => {
                            println!("Error: Could not clone socket");
                        }
                    }

                    let ip = socket.local_addr().unwrap().to_string();
                    let mut client = Client::new(socket, ip);
                    println!("Client connected from {}", client.ip);
                    client.start();
                }
                Err(error) => {
                    println!("Error: Could not connect to client");
                }
            }
        }
    }

    fn handle_input(&self, mut socket: TcpStream) {
        let mut buffer = [0; 8196];
        //get input from keyboard
        let mut input = String::new();
        thread::spawn(move || {
            loop {
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        socket.write(input.as_bytes()).unwrap();
                    }
                    Err(_) => {}
                }
            }
        });
    }
}

fn main() {
    thread::spawn(|| {
        let mut server = Server::new(String::from("127.0.0.1"), 8080);
        server.start()
    }).join();
}
