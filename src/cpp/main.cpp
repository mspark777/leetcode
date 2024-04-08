#include <iostream>
#include <vector>

class Solution {
 public:
  int countStudents(std::vector<int>& students, std::vector<int>& sandwiches) {
    const int CIRCLE = 0;
    const int SQUARE = 1;
    int circleStudentCount = 0;
    int squareStudentCount = 0;

    for (int student : students) {
      circleStudentCount += student == CIRCLE;
      squareStudentCount += student == SQUARE;
    }

    for (int sandwich : sandwiches) {
      if (sandwich == CIRCLE) {
        if (circleStudentCount < 1) {
          return squareStudentCount;
        } else {
          circleStudentCount -= 1;
        }
      } else if (sandwich == SQUARE) {
        if (squareStudentCount < 1) {
          return circleStudentCount;
        } else {
          squareStudentCount -= 1;
        }
      }
    }

    return 0;
  }
};

struct Input {
  std::vector<int> students;
  std::vector<int> sandwiches;
};

int main() {
  const Input inputs[] = {
      {{1, 1, 0, 0}, {0, 1, 0, 1}}, {{1, 1, 1, 0, 0, 1}, {1, 0, 0, 0, 1, 1}}

  };

  for (auto input : inputs) {
    Solution s;
    std::cout << s.countStudents(input.students, input.sandwiches) << std::endl;
  }
  return 0;
}
