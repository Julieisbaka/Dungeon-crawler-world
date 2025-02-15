#include "core/Game.hpp"

Game::Game() : window(1920, 1080, "Dungeon Crawler World"), running(true) {
    renderer.initialize();
    InputManager::getInstance().initialize(window.getGLFWWindow());

    // Load initial assets
    auto& assets = AssetManager::getInstance();
    assets.loadTexture("terrain", "assets/textures/terrain.png");
    assets.loadModel("player", "assets/models/player.obj");
}

Game::~Game() {
    renderer.cleanup();
}

void Game::run() {
    float lastTime = 0.0f;

    while (running && !window.shouldClose()) {
        float currentTime = glfwGetTime();
        float deltaTime = currentTime - lastTime;
        lastTime = currentTime;

        update(deltaTime);
        render();

        window.swapBuffers();
        window.pollEvents();
    }
}

void Game::update(float deltaTime) {
    gameTime += deltaTime;
    InputManager::getInstance().update();
    physics.update(deltaTime);
    world.update(deltaTime);
}

void Game::render() {
    renderer.clear();
    world.render(renderer);
}
