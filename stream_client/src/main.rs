use std::{
    io::{self, Read, Write},
    net::TcpStream,
    time::Duration,
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
}

struct Client {
    ip: String,
    port: u16,
}

impl Client {
    fn new(ip: String, port: u16) -> Self {
        Self { ip, port }
    }

    fn start(&self, cb: fn(frame: StreamFrameResult)) {
        let mut stream = TcpStream::connect(format!("{}:{}", self.ip, self.port)).unwrap();
        println!("Connected!");

        let mut buf = [0; 1024];
        let mut msg_len = 0;
        let mut msg_len_str = String::new();
        let mut stream_frame_string = String::new();
        loop {
            let _set_timeout = stream.set_read_timeout(Some(Duration::from_secs(2)));

            match stream.read(&mut buf) {
                Ok(e) if e == 0 => {
                    println!("Server disconnected");
                    break;
                }
                Ok(_) => {}
                Err(e) => {
                    println!("Connection interrupted: {e}");
                    break;
                }
            }

            let string_buf = String::from_utf8(buf.to_vec()).unwrap();
            let mut it = string_buf.chars().into_iter();

            while let Some(next_char) = it.next() {
                if msg_len == 0 {
                    while let Some(nn) = it.next() {
                        if nn == '\n' {
                            let len_str = msg_len_str.split(":").last().unwrap_or("0");
                            msg_len = len_str.parse::<usize>().unwrap();
                            msg_len_str.clear();
                            break;
                        }
                        msg_len_str += &nn.to_string();
                    }
                    continue;
                }

                if stream_frame_string.len() < msg_len {
                    stream_frame_string += &next_char.to_string();
                    continue;
                }

                let mut values = [0; 3];
                let mut i = 0;

                let mut sit = stream_frame_string.chars().into_iter();
                while let Some(nn) = sit.next() {
                    if nn == ':' {
                        let mut value_str = String::new();

                        while let Some(nnn) = sit.next() {
                            if nnn == ',' {
                                break;
                            }
                            value_str.push(nnn);
                        }
                        println!("{}", value_str.len());
                        if value_str.len() == 0 {
                            println!("{stream_frame_string}")
                        }

                        values[i] = value_str.parse::<i32>().unwrap();
                        i += 1;
                    }
                }

                stream_frame_string.clear();
                msg_len = 0;
                cb(StreamFrameResult::new(values[0], values[1], values[2]));
            }
        }
    }
}

fn main() {
    let client = Client::new(String::from("127.0.0.1"), 4321);

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let _ = client.start(|frame: StreamFrameResult| {
        // print!("\x1B[1;1H");
        // io::stdout().flush().unwrap();
        // print!("{:#?}", frame);
    });
}
