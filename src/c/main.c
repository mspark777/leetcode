#include <stdio.h>

#define bool int

int minCost(char *colors, int *needed_time, int needed_time_size) {
  int total = 0;
  int cur = needed_time[0];

  for (int i = 1; i < needed_time_size; i += 1) {
    if (colors[i] != colors[i - 1]) {
      cur = 0;
    }

    int needed = needed_time[i];
    if (cur < needed) {
      total += cur;
      cur = needed;
    } else {
      total += needed;
    }
  }

  return total;
}

int main() {
  int needed_time0[] = {1, 2, 3, 4, 5};
  printf("%d\n",
         minCost("abaac", needed_time0, sizeof(needed_time0) / sizeof(int)));

  int needed_time1[] = {1, 2, 3};
  printf("%d\n",
         minCost("abc", needed_time1, sizeof(needed_time1) / sizeof(int)));

  int needed_time2[] = {1, 2, 3, 4, 1};
  printf("%d\n",
         minCost("aabaa", needed_time2, sizeof(needed_time2) / sizeof(int)));

  return 0;
}
