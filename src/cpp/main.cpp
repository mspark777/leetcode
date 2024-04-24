#include <iostream>

class Solution {
 public:
  int tribonacci(int n) {
    if (n < 1) {
      return 0;
    }

    int t0 = 0;
    int t1 = 1;
    int t2 = 1;
    for (int i = 2; i < n; i += 1) {
      const int temp = t2;
      t2 += t1 + t0;
      t0 = t1;
      t1 = temp;
    }

    return t2;
  }
};

struct Input {
  int n;
};

int main() {
  const Input inputs[] = {{4}, {25}, {0}};

  for (auto input : inputs) {
    Solution s;
    const auto result = s.tribonacci(input.n);

    std::cout << result << std::endl;
  }

  return 0;
}
