#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int compress(char *chars, int chars_size) {
  int newlen = 0;
  for (int i = 0; i < chars_size; i += 0) {
    int group_len = 1;

    for (int j = i + group_len; j < chars_size; j += 1) {
      if (chars[i] == chars[j]) {
        group_len += 1;
      } else {
        break;
      }
    }

    chars[newlen] = chars[i];
    newlen += 1;

    if (group_len > 1) {
      char temp[11] = {
          0,
      };
      sprintf(temp, "%d", group_len);
      for (int k = 0; temp[k] != 0; k += 1) {
        chars[newlen] = temp[k];
        newlen += 1;
      }
    }

    i += group_len;
  }

  return newlen;
}

char *allocinput(const char *src) {
  unsigned long len = strlen(src);
  char *dest = malloc(len + 1);
  strcpy(dest, src);
  dest[len] = 0;

  return dest;
}

int main() {
  char *inputs[] = {allocinput("aabbccc"), allocinput("a"),
                    allocinput("abbbbbbbbbbbb")};

  for (int i = 0; i < (int)(sizeof(inputs) / sizeof(inputs[0])); i += 1) {
    char *chars = inputs[i];
    const int result = compress(chars, strlen(chars));
    printf("%d %s\n", result, chars);

    free(chars);
    inputs[i] = NULL;
  }
  return 0;
}
