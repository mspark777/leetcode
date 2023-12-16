#include <stdio.h>

#define bool int

bool isAnagram(char *s, char *t) {
  int counts[26] = {
      0,
  };

  int A = 'a';
  for (char *iter = s; *iter != 0; ++iter) {
    counts[*iter - A] += 1;
  }

  for (char *iter = t; *iter != 0; ++iter) {
    counts[*iter - A] -= 1;
  }

  for (int i = 0; i < 26; i += 1) {
    if (counts[i] != 0) {
      return 0;
    }
  }

  return 1;
}

int main() {
  char *input[] = {"anagram", "nagaram", "rat", "car"};

  for (unsigned long i = 0; i < sizeof(input) / sizeof(input[0]); i += 2) {
    const bool result = isAnagram(input[i], input[i + 1]);
    printf("%d\n", result);
  }

  return 0;
}
