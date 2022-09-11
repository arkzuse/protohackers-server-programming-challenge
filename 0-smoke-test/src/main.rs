use std::{
    io::{Read, Write, BufReader, BufWriter},
    net::TcpListener,
    thread,
};

fn main() {
    let server = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in server.incoming() {
        let stream = stream.unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut writer = BufWriter::new(stream.try_clone().unwrap());

        thread::spawn(move || {
            let mut data = vec![];
            reader.read_to_end(&mut data).unwrap();
            writer.write(&data).unwrap();
        });
    }
}
