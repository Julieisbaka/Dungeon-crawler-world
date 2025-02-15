#include "game/Entity.hpp"
#include "game/Component.hpp"

Entity::Entity(const std::string& name) : name(name) {}

void Entity::update(float deltaTime) {
    for (auto& component : components) {
        component->update(deltaTime);
    }
}

void Entity::addComponent(std::shared_ptr<Component> component) {
    components.push_back(component);
}

template<typename T>
std::shared_ptr<T> Entity::getComponent() {
    for (auto& component : components) {
        if (auto cast = std::dynamic_pointer_cast<T>(component)) {
            return cast;
        }
    }
    return nullptr;
}
