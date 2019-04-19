# Copyright (c) 2019 RoccoDev
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

# Build the example
cd rust-examples
cargo build --release
cd ..

# Move the library to the lib folder
mkdir server/mc_server_impl/lib
cd rust-examples/target/release/
mv server.dll.lib server.lib
mv *.lib ../../../server/mc_server_impl/lib
mv *.dylib ../../../server/mc_server_impl/lib
mv *.so ../../../server/mc_server_impl/lib
cd ../../../

# Move the main executable
cd server/mc_server_impl
cargo build --release

cd ..