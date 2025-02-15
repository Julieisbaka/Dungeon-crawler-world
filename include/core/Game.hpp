#pragma once
#include "Window.hpp"
#include "InputManager.hpp"
#include "AssetManager.hpp"
#include "../graphics/Renderer.hpp"
#include "../physics/PhysicsSystem.hpp"
#include "../game/World.hpp"

class Game {
public:
    Game();
    ~Game();

    void run();

private:
    void update(float deltaTime);
    void render();

    Window window;
    Renderer renderer;
    World world;
    bool running;
    PhysicsSystem physics;
    float gameTime{0.0f};
};
