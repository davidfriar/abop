
out vec4 frag;

void main() {
  frag = vec4(1.0,0.0,0.0, 1.0);
  frag = pow(frag, vec4(1.0/2.2));
}
