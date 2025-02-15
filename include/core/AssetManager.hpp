#pragma once
#include <string>
#include <unordered_map>
#include <memory>

class Texture;
class Model;
class Sound;

class AssetManager {
public:
    static AssetManager& getInstance();

    void loadTexture(const std::string& name, const std::string& path);
    void loadModel(const std::string& name, const std::string& path);
    void loadSound(const std::string& name, const std::string& path);

    std::shared_ptr<Texture> getTexture(const std::string& name);
    std::shared_ptr<Model> getModel(const std::string& name);
    std::shared_ptr<Sound> getSound(const std::string& name);

private:
    std::unordered_map<std::string, std::shared_ptr<Texture>> textures;
    std::unordered_map<std::string, std::shared_ptr<Model>> models;
    std::unordered_map<std::string, std::shared_ptr<Sound>> sounds;
};
