rem Copyright (c) 2019 RoccoDev
rem 
rem This software is released under the MIT License.
rem https:\\opensource.org\licenses\MIT

rem Build the example
cd rust-examples
cargo build --release
cd ..

rem Move the library to the lib folder
mkdir server\mc_server_impl\lib
cd rust-examples\target\release\
move server.dll.lib server.lib
move *.lib ..\..\..\server\mc_server_impl\lib
move *.dylib ..\..\..\server\mc_server_impl\lib
move *.so ..\..\..\server\mc_server_impl\lib
cd ..\..\..\

rem Move the main executable
cd server\mc_server_impl
cargo build --release

cd ..