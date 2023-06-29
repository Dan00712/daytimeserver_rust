use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufWriter};
use chrono::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    loop{
        let mut buf = [0; 1];
        let er = stream.read(&mut buf);
        if let Err(_) = er {
            break;
        }
        let cmd = char::from(buf[0]);

        handle_cmd(&stream, cmd);
    }
}

fn handle_cmd(stream: &TcpStream, cmd: char) {
    let mut writer = BufWriter::new(stream);

    let now = Local::now();
    if cmd == 't' {
        let time_str = now.format("%H:%M:%S").to_string();
        let time_str = time_str + "\n";
        if writer.write(time_str.as_bytes()).unwrap_or(0) == 0 {
            return;
        }
    }
    if cmd == 'd' {
        let date_str = now.format("%Y-%m-%d\n").to_string();
        if writer.write(date_str.as_bytes()).unwrap_or(0) == 0 {
            return;
        }
    }

    if let Err(_) = writer.flush(){
        return;
    }
}
