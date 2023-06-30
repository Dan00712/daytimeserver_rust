use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufWriter};
use chrono::prelude::*;
use std::error::Error;

pub fn run_server(ip: &str, port: i32) {
    let endpoint = get_endpoint(ip, port);
    let listener = TcpListener::bind(endpoint).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    loop{
        let mut buf = [0; 1];
        let er = stream.read(&mut buf);
        if let Err(_) = er {
            break;
        }
        let cmd = char::from(buf[0]);

        if let Err(_) = handle_cmd(&stream, cmd) {
            break;
        }
    }
}

fn handle_cmd(stream: &TcpStream, cmd: char) -> Result<(), Box<dyn Error>> {
    let mut writer = BufWriter::new(stream);

    let now = Local::now();
    if cmd == 't' {
        let time_str = now.format("%H:%M:%S").to_string();
        let time_str = time_str + "\n";
        writer.write(time_str.as_bytes())?;
    }
    if cmd == 'd' {
        let date_str = now.format("%Y-%m-%d\n").to_string();
        writer.write(date_str.as_bytes())?;
    }

    writer.flush()?;

    Ok(())
}

fn get_endpoint(ip: &str, port: i32) -> String {
    let ip = String::from(ip);
    let port = port.to_string();
    let port = &port[..];

    let endpoint = ip + ":" + port;

    endpoint
}
