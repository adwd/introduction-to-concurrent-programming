mod blocking_echo_server;
mod coroutine;
mod ioselect;
mod nonblocking_epoll;
mod scheduling;
mod tokio_echo;

fn main() {
    // blocking_echo_server::main();
    // nonblocking_epoll::main();
    coroutine::main();
    scheduling::main();
    // ioselect::main();
    tokio_echo::main();
}
