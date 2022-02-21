mod bakery;
mod barrier;
mod channel;
mod cond;
mod mutex;
mod rwlock;
mod semaphore;

fn main() {
    mutex::main();
    cond::main();
    rwlock::main();
    barrier::main();
    semaphore::main();
    channel::main();
    bakery::main();
}
