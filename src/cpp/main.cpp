#include <iostream>
#include <vector>

class Solution {
 public:
  int trap(std::vector<int>& height) {
    int left = 0;
    int right = static_cast<int>(height.size()) - 1;
    int leftMax = 0;
    int rightMax = 0;
    int result = 0;

    while (left < right) {
      const int lheight = height[left];
      const int rheight = height[right];
      if (lheight < rheight) {
        left += 1;
        if (lheight >= leftMax) {
          leftMax = lheight;
        } else {
          result += leftMax - lheight;
        }
      } else {
        right -= 1;
        if (rheight >= rightMax) {
          rightMax = rheight;
        } else {
          result += rightMax - rheight;
        }
      }
    }

    return result;
  }
};

struct Input {
  std::vector<int> height;
};

int main() {
  const Input inputs[] = {{{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1}},
                          {{4, 2, 0, 3, 2, 5}}};

  for (auto input : inputs) {
    Solution s;
    const auto result = s.trap(input.height);
    std::cout << result << std::endl;
  }

  return 0;
}
