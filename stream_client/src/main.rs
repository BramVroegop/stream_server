use std::{
    io::{self, Read, Write},
    net::TcpStream,
    str::Chars,
    time::Duration,
};

#[derive(Debug)]
struct StreamFrameResult {
    _x: i32,
    _y: i32,
    _z: i32,
}

impl StreamFrameResult {
    fn new(_x: i32, _y: i32, _z: i32) -> Self {
        Self { _x, _y, _z }
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

    fn _get_message_length(
        &self,
        it: &mut Chars<'_>,
        msg_len: &mut Option<usize>,
        msg_len_str: &mut String,
    ) {
        while let Some(nn) = it.next() {
            if nn != '\n' {
                msg_len_str.push(nn);
                continue;
            }

            let len_str = msg_len_str.split(":").last().unwrap_or("0");
            *msg_len = Some(len_str.parse::<usize>().unwrap());
            msg_len_str.clear();
            break;
        }
    }

    fn _get_values(&self, sit: &mut Chars<'_>) -> [i32; 3] {
        let mut values = [0; 3];
        let mut i = 0;
        while let Some(nn) = sit.next() {
            if nn == ':' {
                let mut value_str = String::new();

                while let Some(nnn) = sit.next() {
                    if nnn == ',' {
                        break;
                    }
                    value_str.push(nnn);
                }
                values[i] = value_str.parse::<i32>().unwrap();
                i += 1;
            }
        }
        values
    }

    fn start(&self, cb: fn(frame: StreamFrameResult)) {
        let mut stream = TcpStream::connect(format!("{}:{}", self.ip, self.port)).unwrap();
        println!("Connected!");

        let mut buf = [0; 1024];
        let mut msg_len = None;
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
                if msg_len == None {
                    msg_len_str.push(next_char);
                    self._get_message_length(&mut it, &mut msg_len, &mut msg_len_str);
                    continue;
                }

                if stream_frame_string.len() < msg_len.unwrap() {
                    stream_frame_string.push(next_char);
                    continue;
                }

                let mut sit = stream_frame_string.chars().into_iter();
                let values = self._get_values(&mut sit);

                stream_frame_string.clear();
                msg_len = None;
                cb(StreamFrameResult::new(values[0], values[1], values[2]));
            }
        }
    }
}

fn main() {
    let client = Client::new(String::from("127.0.0.1"), 4321);

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let _ = client.start(|frame: StreamFrameResult| {
        print!("\x1B[1;1H");
        io::stdout().flush().unwrap();
        print!("{:#?}", frame);
    });
}
