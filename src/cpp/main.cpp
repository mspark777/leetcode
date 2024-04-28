#include <algorithm>
#include <iostream>
#include <iterator>
#include <unordered_map>
#include <vector>

class Solution {
 public:
  std::vector<int> sumOfDistancesInTree(int n,
                                        std::vector<std::vector<int>>& edges) {
    this->graph.clear();
    this->count = std::vector(n, 1);
    this->result = std::vector(n, 0);

    for (const std::vector<int>& edge : edges) {
      const int u = edge[0];
      const int v = edge[1];
      this->graph[u].push_back(v);
      this->graph[v].push_back(u);
    }

    this->dfs(0, -1);
    this->dfs2(0, -1);

    return this->result;
  }

 private:
  void dfs(const int node, const int parent) {
    for (const int child : this->graph[node]) {
      if (child != parent) {
        this->dfs(child, node);
        this->count[node] += this->count[child];
        this->result[node] += this->result[child] + this->count[child];
      }
    }
  }

  void dfs2(const int node, const int parent) {
    for (const int child : this->graph[node]) {
      if (child != parent) {
        this->result[child] = this->result[node] - this->count[child] +
                              (this->count.size() - this->count[child]);
        this->dfs2(child, node);
      }
    }
  }

  std::unordered_map<int, std::vector<int>> graph;
  std::vector<int> count;
  std::vector<int> result;
};

struct Input {
  int n;
  std::vector<std::vector<int>> edges;
};

int main() {
  const Input inputs[] = {
      {6, {{0, 1}, {0, 2}, {2, 3}, {2, 4}, {2, 5}}}, {1, {}}, {2, {{1, 0}}}};

  for (auto input : inputs) {
    Solution s;
    const auto result = s.sumOfDistancesInTree(input.n, input.edges);
    std::copy(result.begin(), result.end(),
              std::ostream_iterator<int>(std::cout, " "));
    std::cout << std::endl;
  }

  return 0;
}
