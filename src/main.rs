extern crate bit_utils;
extern crate byteorder;
extern crate mc_varint;

mod net;

fn main() {
    net::listen(8081);
}
