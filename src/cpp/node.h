#ifndef __LEETCODE_CPP_NODE_H__
#define __LEETCODE_CPP_NODE_H__

#include <vector>

class Node {
 public:
  int val;
  std::vector<Node*> children;

  Node();
  Node(int val);
  Node(int val, std::vector<Node*> children);
};

#endif
