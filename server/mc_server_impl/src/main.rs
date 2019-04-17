extern crate bit_utils;
extern crate byteorder;
extern crate mc_varint;
extern crate rjni;

pub mod net;
pub mod api;

fn main() {
    net::listen(8081);
}
