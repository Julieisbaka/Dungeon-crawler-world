// Minimal GLAD implementation
#include "../../include/glad/glad.h"
#include <stdio.h>
#include <stdlib.h>

static void* get_proc_address(const char* namez);

int gladLoadGLLoader(GLADloadproc load) {
    get_proc_address = (void* (*)(const char*))load;
    return 1;
}

int gladLoadGL() {
    return gladLoadGLLoader((GLADloadproc)get_proc_address);
}

void *get_proc_address(const char *namez)
{
    return nullptr;
}
