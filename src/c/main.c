#include <stdio.h>

#define bool int

int findComplement(int num) {
  unsigned int mask = ~0;

  while (num & mask) {
    mask <<= 1;
  }

  return ~mask ^ num;
}

int main() {
  int inputs[] = {5, 1};
  int input_count = sizeof(inputs) / sizeof(inputs[0]);

  for (int i = 0; i < input_count; i += 1) {
    int result = findComplement(inputs[i]);
    printf("%d\n", result);
  }
  return 0;
}
