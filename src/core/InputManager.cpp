#include "core/InputManager.hpp"

InputManager& InputManager::getInstance() {
    static InputManager instance;
    return instance;
}

void InputManager::initialize(GLFWwindow* window) {
    glfwSetCursorPosCallback(window, mouseCallback);
    glfwSetKeyCallback(window, keyCallback);
    glfwSetInputMode(window, GLFW_CURSOR, GLFW_CURSOR_DISABLED);
}

void InputManager::update() {
    mouseDelta = mousePos - lastMousePos;
    lastMousePos = mousePos;
}

bool InputManager::isKeyPressed(int key) const {
    auto it = keyStates.find(key);
    return it != keyStates.end() && it->second;
}

bool InputManager::isKeyHeld(int key) const {
    return glfwGetKey(glfwGetCurrentContext(), key) == GLFW_PRESS;
}

const glm::vec2& InputManager::getMousePosition() const {
    return mousePos;
}

const glm::vec2& InputManager::getMouseDelta() const {
    return mouseDelta;
}

void InputManager::mouseCallback(GLFWwindow*, double xpos, double ypos) {
    InputManager::getInstance().mousePos = glm::vec2(xpos, ypos);
}

void InputManager::keyCallback(GLFWwindow*, int key, int, int action, int) {
    if (action == GLFW_PRESS) {
        InputManager::getInstance().keyStates[key] = true;
    } else if (action == GLFW_RELEASE) {
        InputManager::getInstance().keyStates[key] = false;
    }
}
