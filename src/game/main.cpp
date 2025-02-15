#include "../../include/core/Game.hpp"
#include <exception>
#include <iostream>

int main() {
    try {
        Game game;
        game.run();
        return 0;
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }
}
