#include "core/AssetManager.hpp"
#include <stdexcept>

AssetManager& AssetManager::getInstance() {
    static AssetManager instance;
    return instance;
}

void AssetManager::loadTexture(const std::string& name, const std::string& path) {
    if (textures.find(name) != textures.end()) {
        return;
    }

    auto texture = std::make_shared<Texture>();
    if (!texture->load(path)) {
        throw std::runtime_error("Failed to load texture: " + path);
    }
    textures[name] = texture;
}

void AssetManager::loadModel(const std::string& name, const std::string& path) {
    if (models.find(name) != models.end()) {
        return;
    }

    auto model = std::make_shared<Model>();
    if (!model->load(path)) {
        throw std::runtime_error("Failed to load model: " + path);
    }
    models[name] = model;
}

void AssetManager::loadSound(const std::string& name, const std::string& path) {
    if (sounds.find(name) != sounds.end()) {
        return;
    }

    auto sound = std::make_shared<Sound>();
    if (!sound->load(path)) {
        throw std::runtime_error("Failed to load sound: " + path);
    }
    sounds[name] = sound;
}

std::shared_ptr<Texture> AssetManager::getTexture(const std::string& name) {
    auto it = textures.find(name);
    if (it == textures.end()) {
        throw std::runtime_error("Texture not found: " + name);
    }
    return it->second;
}

std::shared_ptr<Model> AssetManager::getModel(const std::string& name) {
    auto it = models.find(name);
    if (it == models.end()) {
        throw std::runtime_error("Model not found: " + name);
    }
    return it->second;
}

std::shared_ptr<Sound> AssetManager::getSound(const std::string& name) {
    auto it = sounds.find(name);
    if (it == sounds.end()) {
        throw std::runtime_error("Sound not found: " + name);
    }
    return it->second;
}
