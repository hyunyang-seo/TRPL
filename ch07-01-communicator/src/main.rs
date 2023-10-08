extern crate ch07_01_communicator;

fn main() {
    ch07_01_communicator::client::connect();
    ch07_01_communicator::network::connect();
    ch07_01_communicator::network::server::connect();
}