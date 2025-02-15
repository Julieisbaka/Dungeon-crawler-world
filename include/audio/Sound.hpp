#pragma once
#include <string>
#include <AL/al.h>
#include <AL/alc.h>

class Sound {
public:
    Sound();
    ~Sound();

    bool load(const std::string& path);
    void play();
    void stop();
    void setVolume(float volume);
    void setLooping(bool loop);

private:
    ALuint buffer;
    ALuint source;
    bool isLoaded;
};
