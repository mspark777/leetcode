#include <stdio.h>
#include <string.h>

#define bool int

int numDecodings(char *s) {
  const int slen = strlen(s);
  if (slen == 0) {
    return 0;
  } else if (s[0] == '0') {
    return 0;
  } else if (slen == 1) {
    return 1;
  }

  const int ZERO = (int)'0';

  int result = 1;
  int memo = 1;

  for (int i = 1; i < slen; i += 1) {
    const int code1 = ((int)s[i]) - ZERO;
    const int code10 = ((((int)s[i - 1]) - ZERO) * 10) + code1;

    int cnt = 0;
    if (code1 != 0) {
      cnt += result;
    }

    if ((code10 >= 10) && (code10 <= 26)) {
      cnt += memo;
    }

    memo = result;
    result = cnt;
  }

  return result;
}

int main() {
  char *input[] = {"12", "226", "06"};

  for (unsigned long i = 0; i < sizeof(input) / sizeof(input[0]); i += 2) {
    const bool result = numDecodings(input[i]);
    printf("%d\n", result);
  }

  return 0;
}
