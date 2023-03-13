#include "./lib.h"
#include <stdio.h>
#include <stdlib.h>

/**
 * struct list_node
 */

struct list_node *new_list_node(int val, struct list_node *next) {
  struct list_node *node = malloc(sizeof(struct list_node));
  node->val = val;
  node->next = next;

  return node;
}

struct list_node *new_list(const int *vals, const int val_count) {
  struct list_node head = {};
  struct list_node *tail = &head;

  for (int i = 0; i < val_count; i += 1) {
    struct list_node *node = new_list_node(vals[i], NULL);
    tail->next = node;
    tail = node;
  }

  return head.next;
}

struct list_node *new_cycle_list(const int *vals, int val_count, int pos) {
  struct list_node head = {};
  struct list_node *tail = &head;
  struct list_node *target = NULL;

  for (int i = 0; i < val_count; i += 1) {
    struct list_node *node = new_list_node(vals[i], NULL);
    if (i == pos) {
      target = node;
    }

    tail->next = node;
    tail = node;
  }

  tail->next = target;
  return head.next;
}

void free_list_node(struct list_node *head, struct list_node *cycle) {
  while (head != cycle) {
    struct list_node *next = head->next;
    free(head);

    head = next;
  }

  if (cycle == NULL) {
    return;
  }

  head = cycle->next;
  while (head != cycle) {
    struct list_node *next = head->next;
    free(head);

    head = next;
  }

  free(cycle);
}

/**
 * struct tree_node
 */

struct tree_node *new_tree_node(int val, struct tree_node *left,
                                struct tree_node *right) {
  struct tree_node *node = malloc(sizeof(struct tree_node));
  node->val = val;
  node->left = left;
  node->right = right;

  return node;
}

struct tree_node *new_tree_left(int val, struct tree_node *left) {
  return new_tree_node(val, left, NULL);
}

struct tree_node *new_tree_right(int val, struct tree_node *right) {
  return new_tree_node(val, NULL, right);
}

struct tree_node *new_tree_val(int val) {
  return new_tree_node(val, NULL, NULL);
}

void free_tree_node(struct tree_node *node) {
  if (node != NULL) {
    free_tree_node(node->left);
    free_tree_node(node->right);
    free(node);
  }
}
