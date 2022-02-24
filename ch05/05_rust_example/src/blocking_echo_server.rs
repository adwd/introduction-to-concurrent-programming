use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpListener,
};

pub fn main() {
    let listener = TcpListener::bind("127.0.0.1:10000").unwrap();

    while let Ok((stream, _addr)) = listener.accept() {
        let stream0 = stream.try_clone().unwrap();
        let mut reader = BufReader::new(stream0);
        let mut writer = BufWriter::new(stream);

        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        let _len = writer.write(buf.as_bytes()).unwrap();
        writer.flush().unwrap();
    }
}
