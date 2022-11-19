#ifndef Sphere_h
#define Sphere_h

#include <glm/glm.hpp>
#include <glm/gtx/string_cast.hpp> // for debug glm::to_string(vec)

struct Sphere {
    glm::vec3 center;
    float radius = 0; // zero used to check if struct is empty
    glm::vec3 color;
};

#endif
