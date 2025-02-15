#pragma once
#include <GLFW/glfw3.h>
#include <unordered_map>
#include <glm/glm.hpp>

class InputManager {
public:
    static InputManager& getInstance();
    void initialize(GLFWwindow* window);
    void update();

    bool isKeyPressed(int key) const;
    bool isKeyHeld(int key) const;
    const glm::vec2& getMousePosition() const;
    const glm::vec2& getMouseDelta() const;

private:
    InputManager() = default;
    static void mouseCallback(GLFWwindow* window, double xpos, double ypos);
    static void keyCallback(GLFWwindow* window, int key, int scancode, int action, int mods);

    std::unordered_map<int, bool> keyStates;
    glm::vec2 mousePos{0.0f};
    glm::vec2 mouseDelta{0.0f};
    glm::vec2 lastMousePos{0.0f};
};
