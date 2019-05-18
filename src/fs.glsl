
out vec4 frag;

void main() {
  frag = vec4(0.6,0.6,0.3, 1.0);
  frag = pow(frag, vec4(1.0/2.2));
}
