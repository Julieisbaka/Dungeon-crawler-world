#pragma once
#include <vector>
#include <glm/glm.hpp>

struct Collider {
    glm::vec3 position;
    glm::vec3 size;
    bool isDynamic;
};

class PhysicsSystem {
public:
    void update(float deltaTime);
    void addCollider(const Collider& collider);
    bool checkCollision(const Collider& a, const Collider& b);
    void resolveCollision(Collider& a, Collider& b);

private:
    std::vector<Collider> colliders;
    float gravity = -9.81f;
};
