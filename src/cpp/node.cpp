#include "./node.h"

Node::Node() {}

Node::Node(int val) { this->val = val; }

Node::Node(int val, std::vector<Node*> children) {
  this->val = val;
  this->children = children;
}
