use ::std::net::{TcpListener, TcpStream};
use ::tp::ThreadPool;

fn handle_connection(stream: TcpStream) {}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }
}
