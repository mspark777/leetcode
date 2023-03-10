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

#endif
