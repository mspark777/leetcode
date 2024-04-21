#include <iostream>
#include <vector>

class Solution {
 public:
  bool validPath(int n, std::vector<std::vector<int>>& edges, int source,
                 int destination) {
    if (n == 1) {
      return true;
    }

    std::vector<bool> visited(n, false);
    visited[source] = true;

    bool loop = true;
    while (loop) {
      loop = false;
      for (const std::vector<int>& edge : edges) {
        const int u = edge[0];
        const int v = edge[1];
        if (visited[u] != visited[v]) {
          visited[u] = true;
          visited[v] = true;
          loop = true;
        }

        if (visited[destination]) {
          return true;
        }
      }
    }

    return false;
  }
};

struct Input {
  int n;
  std::vector<std::vector<int>> edges;
  int source;
  int destination;
};

int main() {
  const Input inputs[] = {
      {3, {{0, 1}, {1, 2}, {2, 0}}, 0, 2},
      {6, {{0, 1}, {0, 2}, {3, 5}, {5, 4}, {4, 3}}, 0, 5},
  };

  for (auto input : inputs) {
    Solution s;
    const auto result =
        s.validPath(input.n, input.edges, input.source, input.destination);

    std::cout << result << std::endl;
  }

  return 0;
}
