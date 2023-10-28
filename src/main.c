#include <stdio.h>

int countVowelPermutation(int n) {
  const long long MOD = 1000000007;

  long long a = 1;
  long long e = 1;
  long long i = 1;
  long long o = 1;
  long long u = 1;

  for (int j = 1; j < n; j += 1) {
    const long long nexta = e + i + u;
    const long long nexte = a + i;
    const long long nexti = e + o;
    const long long nexto = i;
    const long long nextu = i + o;

    a = nexta % MOD;
    e = nexte % MOD;
    i = nexti % MOD;
    o = nexto % MOD;
    u = nextu % MOD;
  }

  return (int)((a + e + i + o + u) % MOD);
}

int main() {
  const int input[] = {1, 2, 5, 144};
  const int count = (int)(sizeof(input) / sizeof(int));

  for (int i = 0; i < count; i += 1) {
    const int result = countVowelPermutation(input[i]);
    printf("%d\n", result);
  }

  return 0;
}
