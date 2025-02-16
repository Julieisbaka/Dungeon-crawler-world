#include "../../include/core/Game.hpp"
#include <glad/glad.h>
#include <chrono>

Game::Game() : window(1280, 720, "Dungeon Crawler World"), running(true) {
    // Initialize input system
    InputManager::getInstance().initialize(window.getHandle());

    // Setup OpenGL state
    glEnable(GL_DEPTH_TEST);
    glEnable(GL_CULL_FACE);
    glCullFace(GL_BACK);

    // Set clear color
    glClearColor(0.1f, 0.1f, 0.1f, 1.0f);

    int width, height;
    window.getFramebufferSize(width, height);
    glViewport(0, 0, width, height);
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
    // Clear buffers
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

    // Update viewport in case of window resize
    int width, height;
    window.getFramebufferSize(width, height);
    glViewport(0, 0, width, height);

    // Renderer will handle actual rendering here
    renderer.render(world);
}
