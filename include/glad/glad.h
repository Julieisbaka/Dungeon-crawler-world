// OpenGL loader generated from https://glad.dav1d.de/
// Generated with settings: Language: C/C++, Specification: OpenGL, API: gl=4.5, Profile: core
#ifndef GLAD_H
#define GLAD_H

#include <KHR/khrplatform.h>

typedef void* GLADloadproc;
typedef unsigned int GLenum;
typedef unsigned char GLboolean;
typedef unsigned int GLbitfield;
typedef void GLvoid;
typedef signed char GLbyte;
typedef short GLshort;
typedef int GLint;
typedef unsigned char GLubyte;
typedef unsigned short GLushort;
typedef unsigned int GLuint;
typedef int GLsizei;
typedef float GLfloat;
typedef double GLdouble;
typedef char GLchar;
typedef short GLhalf;
typedef ptrdiff_t GLsizeiptr;
typedef ptrdiff_t GLintptr;

#ifdef __cplusplus
extern "C" {
#endif

GLAPI int gladLoadGL(void);
GLAPI int gladLoadGLLoader(GLADloadproc);

#ifdef __cplusplus
}
#endif

#endif
