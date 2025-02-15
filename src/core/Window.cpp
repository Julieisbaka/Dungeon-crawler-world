// filepath: d:\New folder\Dungeon-crawler-world\src\core\Window.cpp
#include "../../include/core/Window.hpp"
#include <stdexcept>

Window::Window(int width, int height, const char* title)
    : width(width), height(height) {
    if (!glfwInit()) {
        throw std::runtime_error("Failed to initialize GLFW");
    }

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 5);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    window = glfwCreateWindow(width, height, title, nullptr, nullptr);
    if (!window) {
        glfwTerminate();
        throw std::runtime_error("Failed to create GLFW window");
    }

    glfwMakeContextCurrent(window);
    glfwSwapInterval(1); // Enable vsync
}

Window::~Window() {
    if (window) {
        glfwDestroyWindow(window);
    }
    glfwTerminate();
}

void Window::update() {
    glfwSwapBuffers(window);
    glfwPollEvents();
}

bool Window::shouldClose() const {
    return glfwWindowShouldClose(window);
}

GLFWwindow* Window::getHandle() const {
    return window;
}

void Window::getFramebufferSize(int& width, int& height) const {
    glfwGetFramebufferSize(window, &width, &height);
}
