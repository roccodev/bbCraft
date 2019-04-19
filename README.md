<!--
 Copyright (c) 2019 RoccoDev
 
 This software is released under the MIT License.
 https://opensource.org/licenses/MIT
-->

# bbCraft
[![Build status](https://api.travis-ci.com/RoccoDev/bbCraft.svg?branch=master)](https://travis.ci/RoccoDev/bbCraft)

A barebones Minecraft server, written in Rust.  

Its sole purpose is to simply disconnect all incoming clients with a custom message.  
This is useful if, for example, you are running an auth server and don't want to generate all the server files.

## Installation
* Install Rust and Cargo
* Run `build.sh` (use Git Bash on Windows)
* The server comes with a sample server pre-installed ([source](rust-examples)).
    * To make a custom server, you have to build a dynamic library in any language of your choice.
    * Refer to the example for more details.
    * The library must be called:
        * `libserver.so` on Linux;
        * `server.dll` on Windows;
        * `libserver.dylib` on MacOS.

## License
MIT
