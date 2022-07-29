use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use std::{
    error::Error,
    io,
    io::{prelude::*, BufReader},
};

pub fn client() -> Result<(), Box<dyn Error>> {
    let mut conn = LocalSocketStream::connect("/home/jmhuang/os/example1.sock")?;
    conn.write_all(b"Hello from client!\n")?;

    let mut conn = BufReader::new(conn);
    let mut buffer = String::new();
    conn.read_line(&mut buffer)?;

    println!("Server answered: {}", buffer);

    Ok(())
}

pub fn server() -> Result<(), Box<dyn Error>> {
    fn handle_error(connection: io::Result<LocalSocketStream>) -> Option<LocalSocketStream> {
        connection
            .map_err(|error| eprintln!("Incoming connection failed: {}", error))
            .ok()
    }

    let listener = LocalSocketListener::bind("/home/jmhuang/os/example1.sock")?;
    for mut conn in listener.incoming().filter_map(handle_error) {
        println!("Incoming connection!");

        conn.write_all(b"Hello from server!\n")?;

        // Add buffering to the connection to read a line.
        let mut conn = BufReader::new(conn);
        let mut buffer = String::new();
        conn.read_line(&mut buffer)?;

        println!("Client answered: {}", buffer);
    }
    Ok(())
}
