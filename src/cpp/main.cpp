#include <iostream>
#include <iterator>
#include <tuple>
#include <unordered_set>
#include <vector>

class Solution {
 public:
  std::vector<int> intersection(std::vector<int> &nums1,
                                std::vector<int> &nums2) {
    std::unordered_set<int> set1(nums1.begin(), nums1.end());
    std::unordered_set<int> set2(nums2.begin(), nums2.end());

    std::vector<int> result;
    for (const auto &num : set1) {
      if (set2.find(num) != set2.end()) {
        result.push_back(num);
      }
    }

    return result;
  }
};

int main() {
  std::tuple<std::vector<int>, std::vector<int>> input[] = {
      make_tuple(std::vector{1, 2, 2, 1}, std::vector{2, 2}),
      make_tuple(std::vector{4, 9, 5}, std::vector{9, 4, 9, 8, 4}),
  };

  for (auto &i : input) {
    Solution s;
    std::vector<int> result = s.intersection(std::get<0>(i), std::get<1>(i));
    std::copy(result.begin(), result.end(),
              std::ostream_iterator<int>(std::cout, " "));
    std::cout << std::endl;
  }
}
