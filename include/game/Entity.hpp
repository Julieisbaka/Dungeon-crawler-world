#pragma once
#include <glm/glm.hpp>
#include <memory>
#include <vector>

class Component;

class Entity {
public:
    Entity(const std::string& name) : name(name) {}

    void update(float deltaTime) {
        for (auto& component : components) {
            component->update(deltaTime);
        }
    }

    void addComponent(std::shared_ptr<Component> component) {
        components.push_back(component);
    }

    template<typename T>
    std::shared_ptr<T> getComponent() {
        for (const auto& component : components) {
            std::shared_ptr<T> castedComponent = std::dynamic_pointer_cast<T>(component);
            if (castedComponent) {
                return castedComponent;
            }
        }
        return nullptr;
    }

    const glm::vec3& getPosition() const { return position; }
    void setPosition(const glm::vec3& pos) { position = pos; }

private:
    std::string name;
    glm::vec3 position{0.0f};
    std::vector<std::shared_ptr<Component>> components;
};
