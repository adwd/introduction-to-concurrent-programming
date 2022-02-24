mod blocking_echo_server;
mod nonblocking_epoll;

fn main() {
    blocking_echo_server::main();
    nonblocking_epoll::main();
}
