#pragma once
#include <string>
#include <GL/glew.h>

class Texture {
public:
    Texture();
    ~Texture();

    bool load(const std::string& path);
    void bind(unsigned int slot = 0) const;
    void unbind() const;

private:
    unsigned int rendererID;
    std::string filePath;
    unsigned char* localBuffer;
    int width, height, BPP;
};
