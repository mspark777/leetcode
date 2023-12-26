#include <stdio.h>
#include <stdlib.h>

#define bool int

int numRollsToTarget(int n, int k, int target) {
  const int MOD = 1000000007;
  int *dp = calloc(target + 1, sizeof(int));

  dp[0] = 1;

  for (int i = 1; i <= n; i += 1) {
    for (int j = target; j >= 0; j -= 1) {
      dp[j] = 0;

      for (int p = 1; p <= k; p += 1) {
        if (j < p) {
          break;
        }

        dp[j] = (dp[j] + dp[j - p]) % MOD;
      }
    }
  }

  const int result = dp[target];
  free(dp);
  return result;
}

struct input {
  int n;
  int k;
  int target;
};

int main() {
  struct input inputs[] = {
      {.n = 1, .k = 6, .target = 3},
      {.n = 2, .k = 6, .target = 7},
      {.n = 30, .k = 30, .target = 500},
  };

  for (unsigned long i = 0; i < sizeof(inputs) / sizeof(struct input); i += 1) {
    struct input *input = &inputs[i];
    const bool result = numRollsToTarget(input->n, input->k, input->target);
    printf("%d\n", result);
  }

  return 0;
}
