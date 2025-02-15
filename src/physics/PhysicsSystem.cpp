#include "physics/PhysicsSystem.hpp"
#include "../../include/physics/PhysicsSystem.hpp"
#include <glm/geometric.hpp>
#include <cmath>

void PhysicsSystem::update(float deltaTime) {
    for (auto& collider : colliders) {
        if (collider.isDynamic) {
            // Apply gravity
            collider.position.y += gravity * deltaTime;

            // Check collisions with other colliders
            for (auto& other : colliders) {
                if (&other != &collider && checkCollision(collider, other)) {
                    resolveCollision(collider, other);
                }
            }
        }
    }
}

void PhysicsSystem::addCollider(const Collider& collider) {
    colliders.push_back(collider);
}

bool PhysicsSystem::checkCollision(const Collider& a, const Collider& b) {
    return (abs(a.position.x - b.position.x) * 2 < (a.size.x + b.size.x)) &&
           (abs(a.position.y - b.position.y) * 2 < (a.size.y + b.size.y)) &&
           (abs(a.position.z - b.position.z) * 2 < (a.size.z + b.size.z));
}

void PhysicsSystem::resolveCollision(Collider& a, Collider& b) {
    if (!a.isDynamic) return;

    glm::vec3 normal = glm::normalize(b.position - a.position);
    glm::vec3 overlap = (a.size + b.size) * 0.5f - glm::abs(a.position - b.position);
    a.position -= normal * overlap;
}
