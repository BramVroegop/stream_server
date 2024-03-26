use std::{
    io::{Read, Write},
    net::TcpListener,
};

#[derive(Debug)]
struct StreamFrameResult {
    x: i32,
    y: i32,
    z: i32,
}

impl StreamFrameResult {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn to_string(&self) -> String {
        format!("x:{},y:{},z:{}", self.x, self.y, self.z)
    }
}

struct Server {
    port: u16,
}

impl Server {
    fn new(port: u16) -> Self {
        Self { port }
    }

    fn listen(&self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).unwrap();
        let mut current_stream_frame = StreamFrameResult::new(0, 0, 0);
        let mut _buf = [0; 1024];
        if let Ok((mut socket, _)) = listener.accept() {
            println!("Connected!");
            loop {
                let _set_non_blocking = socket.set_nonblocking(true);
                let bytes_received = socket.read(&mut _buf);
                match bytes_received {
                    Ok(bytes) if bytes == 0 => {
                        println!("Client Disconnected");
                        break;
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                    Err(e) => {
                        println!("Connection interrupted: {e}");
                        break;
                    }
                    _ => {}
                }

                let stream_string = current_stream_frame.to_string();
                let formatted_stream_string =
                    format!("l:{}\n{}", stream_string.len(), stream_string);
                let _ = socket.write_all(formatted_stream_string.as_bytes());

                current_stream_frame = StreamFrameResult::new(
                    current_stream_frame.x + 1,
                    current_stream_frame.y - 1,
                    current_stream_frame.z + 1,
                );
            }
        } else {
            panic!();
        }
    }
}

fn main() {
    let svr = Server::new(4321);

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let _ = svr.listen();
}
