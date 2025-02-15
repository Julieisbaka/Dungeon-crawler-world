# Source files
set(SOURCES
    game/main.cpp
    physics/PhysicsSystem.cpp
)

# Create executable
add_executable(DungeonCrawler ${SOURCES})

# Link libraries
target_link_libraries(DungeonCrawler PRIVATE glm::glm)

# Set include directories
target_include_directories(DungeonCrawler PRIVATE
    ${CMAKE_SOURCE_DIR}/include
)
