#include "../../include/core/Game.hpp"
#include <chrono>

Game::Game() : window(1280, 720, "Dungeon Crawler World"), running(true) {
    // Initialize subsystems
    InputManager::getInstance().initialize(window.getHandle());
}

Game::~Game() = default;

void Game::run() {
    auto lastFrame = std::chrono::high_resolution_clock::now();

    while (running && !window.shouldClose()) {
        auto currentFrame = std::chrono::high_resolution_clock::now();
        float deltaTime = std::chrono::duration<float>(currentFrame - lastFrame).count();
        lastFrame = currentFrame;

        // Update game state
        update(deltaTime);

        // Render frame
        render();

        // Update window
        window.update();
    }
}

void Game::update(float deltaTime) {
    // Update input system
    InputManager::getInstance().update();

    // Update physics
    physics.update(deltaTime);

    // Update game time
    gameTime += deltaTime;

    // Example of handling escape key to exit
    if (InputManager::getInstance().isKeyPressed(GLFW_KEY_ESCAPE)) {
        running = false;
    }
}

void Game::render() {
    // Clear frame
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

    // Rendering will be implemented here
}
