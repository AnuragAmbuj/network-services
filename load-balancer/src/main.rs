mod services;

use pingora::server::Server;

fn main() {
    let mut lb_server = Server::new(None).unwrap();
    lb_server.bootstrap();
    lb_server.run_forever();
}
