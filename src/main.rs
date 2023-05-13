mod nakada;
use nakada::server::DBServer::{Server};

fn main() {
    let server = match Server::bind("127.0.0.1:5994") {
        Ok(val) => val,
        Err(err) =>
            {
                println!("Failed to bind DB Server {err}\n");
                return
            }
    };
}
