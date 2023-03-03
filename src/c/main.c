#include <stdio.h>
#include <string.h>

int strStr(char *haystack, char *needle) {
  const int m = (int)strlen(needle);
  const int n = (int)strlen(haystack);

  for (int start = 0; start <= n - m; start += 1) {
    for (int i = 0; i < m; i += 1) {
      if (needle[i] != haystack[start + i]) {
        break;
      }

      if (i == (m - 1)) {
        return start;
      }
    }
  }

  return -1;
}

int main() {
  char *inputs[][2] = {{"sadbutsad", "sad"}, {"leetcode", "leeto"}};

  for (int i = 0; i < (int)(sizeof(inputs) / sizeof(inputs[0])); i += 1) {
    const int result = strStr(inputs[i][0], inputs[i][1]);
    printf("%d\n", result);
  }
  return 0;
}
