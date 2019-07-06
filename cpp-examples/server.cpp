// Copyright (c) 2019 RoccoDev
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#include <iostream>

extern "C" {
    void server_load() {
        std::cout << "Server loaded!" << std::endl;
    }

    void server_unload() {
        std::cout << "Server unloaded!" << std::endl;
    }

    char* player_connect(char* uuid, char* name) {
        // `uuid` == nullptr when the server is in offline mode.

        std::cout << "Player " << player << " connected." << std::endl;
        return "Kicked.";
    }
}
