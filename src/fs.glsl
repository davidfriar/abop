out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;
in vec3 Color;

//uniform vec3 lightPos;
uniform vec3 view_pos;
//uniform vec3 lightColor;

void main()
{
    vec3 lightPos = vec3(3.0, 3.0, 3.0);
    vec3 lightColor = vec3(1.0, 1.0, 1.0);

    // ambient
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * lightColor;

    // diffuse
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    // specular
    float specularStrength = 0.5;
    vec3 viewDir = normalize(view_pos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * lightColor;

    vec3 result = (ambient + diffuse + specular) * Color;
    FragColor = vec4(result, 1.0);
}
