use std::time::Duration;

use tiny_http::{Header, Response, Server};

mod decrypt;
mod hid;
mod prometheus;

fn main() {
    pretty_env_logger::init();

    let thread = std::thread::spawn(hid::read_hid);

    let server = Server::http("0.0.0.0:20110").unwrap();

    println!("Serving metrics on http://localhost:20110");

    loop {
        // check worker thread health every so often
        if thread.is_finished() {
            thread.join().unwrap();
        }
        let Some(request) = server.recv_timeout(Duration::from_millis(500)).unwrap() else {
            continue;
        };
        let _ = request
            .respond(
                Response::from_data(prometheus::publish())
                    .with_header("Content-Type: text/plain".parse::<Header>().unwrap()),
            )
            .inspect_err(|e| log::error!("Error sending HTTP response: {e}"));
    }
}
