#pragma once
#include <GLFW/glfw3.h>

class Window {
public:
    Window(int width = 1280, int height = 720, const char* title = "Dungeon Crawler World");
    ~Window();

    void update();
    bool shouldClose() const;
    GLFWwindow* getHandle() const;
    void getFramebufferSize(int& width, int& height) const;

private:
    GLFWwindow* window{nullptr};
    int width;
    int height;
};
