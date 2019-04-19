# Copyright (c) 2019 RoccoDev
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

# Build the example
cd rust-examples
cargo build --release
cd ..

# Move the library to the lib folder
mv rust-examples/target/release/* server/mc_server_impl/lib

# Move the main executable
cd server/mc_server_impl
cargo build --release

cd ..