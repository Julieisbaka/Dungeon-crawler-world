#pragma once
#include <glm/glm.hpp>
#include <memory>
#include <vector>

class Component;

class Entity {
public:
    Entity(const std::string& name);

    void update(float deltaTime);
    void addComponent(std::shared_ptr<Component> component);

    template<typename T>
    std::shared_ptr<T> getComponent();

    const glm::vec3& getPosition() const { return position; }
    void setPosition(const glm::vec3& pos) { position = pos; }

private:
    std::string name;
    glm::vec3 position{0.0f};
    std::vector<std::shared_ptr<Component>> components;
};
