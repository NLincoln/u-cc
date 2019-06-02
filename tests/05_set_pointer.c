int main() {
  int c = 4;
  int* b = &c;
  *b = 2;
  return *b;
}
