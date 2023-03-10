#include "./lib.h"
#include <stdio.h>
#include <stdlib.h>

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
