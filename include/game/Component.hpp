#pragma once
#include <memory>

class Entity;

class Component {
public:
    Component() = default;
    virtual ~Component() = default;

    virtual void start() {}
    virtual void update(float deltaTime) {}
    virtual void onEnable() {}
    virtual void onDisable() {}

    void setEntity(std::shared_ptr<Entity> newEntity) { entity = newEntity; }
    std::shared_ptr<Entity> getEntity() const { return entity.lock(); }

protected:
    std::weak_ptr<Entity> entity;
};
