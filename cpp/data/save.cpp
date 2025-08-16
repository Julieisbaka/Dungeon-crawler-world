// This file handles saving player data and gamestates
// `Scheme/player.json`
// `saves/`
// The saves folder will contain a folder for each save. The name of the folder
// will be the name of the Save. When a player loads into a save the variable
// `current_save` is set to the name of said save. The code will then check if
// the folder exists within `saves`, and if it does not then it will be created.
// If it does then the data in the folder will be loaded.
#include <string>

static std::string dir = "saves/";
std::string path =
    dir + current_save; // identifier "current_save" is undefinedC/C++(20)
