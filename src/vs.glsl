in vec3 pos;
in vec3 col;
in vec3 norm;
out vec3 FragPos;
out vec3 Color;
out vec3 Normal;
uniform mat4 mvp_transform;
uniform mat4 model_transform;
uniform mat3 normal_transform;


void main() {
  FragPos = vec3(model_transform * vec4(pos, 1.0));
  Color = col;
  Normal = normal_transform * norm;
  gl_Position = mvp_transform * vec4(pos, 1.0);
}
