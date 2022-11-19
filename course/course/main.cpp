#ifdef __APPLE__
#define GL_SILENCE_DEPRECATION // disable deprecation warnings
#endif

// install and add dependecies using this way https://www.youtube.com/watch?v=MHlbNbWlrIM
#include <iostream>
#include <limits>
#include <cmath>
#include <GLFW/glfw3.h>
#include <embree3/rtcore.h>
#include "Sphere.h"

const float INF = std::numeric_limits<float>::infinity();

const int WIDTH = 1280;
const int HEIGHT = 720;
const glm::vec3 o = glm::vec3(0.0f, 0.0f, 0.0f);
const Sphere spheres[3] = {
    {
        glm::vec3(0.0f, -1.0f, 3.0f),
        1.0f,
        glm::vec3(255.0f, 0.0f, 0.0f)
    },
    {
        glm::vec3(2.0f, 0.0f, 4.0f),
        1.0f,
        glm::vec3(0.0f, 0.0f, 255.0f)
        
    },
    {
        glm::vec3(-2.0f, 0.0f, 4.0f),
        1.0f,
        glm::vec3(0.0f, 255.0f, 0.0f)
        
    },
};

// o - observer point, d - direction
// TODO: rewrite as shader
glm::vec2 intersect_ray_sphere(glm::vec3 observer, glm::vec3 direction, Sphere s) {
    glm::vec3 co = observer - s.center;
    float a = glm::dot(direction, direction);
    float b = 2 * glm::dot(co, direction);
    float c = glm::dot(co, co) - s.radius * s.radius;
    float discriminant = b * b - 4 * a * c;
    if (discriminant < 0) return glm::vec2(INF, INF);
    else {
        float t1 = (-b + sqrt(discriminant)) / (2.0 * a);
        float t2 = (-b - sqrt(discriminant)) / (2.0 * a);
        return glm::vec2(t1, t2);
    }
};

glm::vec3 trace_ray(glm::vec3 observer, glm::vec3 direction, float observer_min, float observer_max) {
    float closest_intersect = INF;
    Sphere closest_sphere;
    for (Sphere s: spheres) {
        glm::vec2 intersects = intersect_ray_sphere(observer, direction, s);
        // TODO: refactor to lambda
        if (intersects[0] < closest_intersect && observer_min < intersects[0] && intersects[0] < observer_max) {
            closest_intersect = intersects[0];
            closest_sphere = s;
        }
        if (intersects[1] < closest_intersect && observer_min < intersects[1] && intersects[1] < observer_max) {
            closest_intersect = intersects[1];
            closest_sphere = s;
        }
    }
    return closest_sphere.radius != 0 ? closest_sphere.color : glm::vec3(1.0f, 1.0f, 1.0f); // background default color
};

int main(int argc, const char * argv[]) {
    glfwInit();
    GLFWwindow* window = glfwCreateWindow(WIDTH, HEIGHT, __FILE__, NULL, NULL);
    glfwMakeContextCurrent(window);
    glfwSetWindowAttrib(window, GLFW_RESIZABLE, false);
    glClearColor(0.0f, 0.0f, 0.0f, 0.0f);
    while (!glfwWindowShouldClose(window)) {
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        glBegin(GL_POINTS);
        for (double w = 0; w < WIDTH; w++)
            glVertex2d(0.9f, 0.0f);
            //for (double h = 0; h < HEIGHT; h++) {
                //std::cout << w << std::endl;
                //std::cout << h << std::endl;
            //}
        glEnd();
        glfwSwapBuffers(window);
        glfwPollEvents();
    }
    glfwTerminate();
    return 0;
}
