use ns_tutorial::local_socket::server;

pub fn main() {
    if let Err(e) = server() {
        println!("local-server error: {:?}", e);
    }
}
