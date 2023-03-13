#ifndef __LEETCODE_LIB_H__
#define __LEETCODE_LIB_H__

struct list_node {
  int val;
  struct list_node *next;
};

struct list_node *new_list_node(int val, struct list_node *next);
struct list_node *new_list(const int *vals, const int val_count);
struct list_node *new_cycle_list(const int *vals, int val_count, int pos);
void free_list_node(struct list_node *head, struct list_node *cycle);

struct tree_node {
  int val;
  struct tree_node *left;
  struct tree_node *right;
};

struct tree_node *new_tree_node(int val, struct tree_node *left,
                                struct tree_node *right);
struct tree_node *new_tree_left(int val, struct tree_node *left);
struct tree_node *new_tree_right(int val, struct tree_node *right);
struct tree_node *new_tree_val(int val);
void free_tree_node(struct tree_node *node);

#endif
