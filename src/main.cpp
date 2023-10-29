#include <cmath>
#include <iostream>

class Solution {
public:
  int poorPigs(int buckets, int minutesToDie, int minutesToTest) {
    const double fb = static_cast<double>(buckets);
    const double fd = static_cast<double>(minutesToDie);
    const double ft = static_cast<double>(minutesToTest);
    const double result = std::ceil(std::log2(fb) / std::log2(ft / fd + 1.0));
    return static_cast<int>(result);
  }
};

int main() {
  const int input[][3] = {{4, 15, 15}, {4, 15, 30}, {125, 1, 4}};
  const int count = (int)(sizeof(input) / sizeof(int[3]));

  for (int i = 0; i < count; i += 1) {
    auto nums = input[i];
    const int result = Solution().poorPigs(nums[0], nums[1], nums[2]);
    std::cout << result << std::endl;
  }

  return 0;
}
