#include "./utils.h"
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

static int trie_node_get_index(char ch);
static int trie_node_in_range(const struct trie_node *self, int i);

struct trie_node *new_trie_node() {
  const int count = 26;
  struct trie_node **links = calloc(count, sizeof(struct trie_node *));
  struct trie_node *node = malloc(sizeof(struct trie_node));
  node->links = links;
  node->link_count = count;
  node->ended = 0;

  return node;
}

struct trie_node *trie_node_get(const struct trie_node *self, char ch) {
  int i = trie_node_get_index(ch);
  return trie_node_in_range(self, i) ? self->links[i] : NULL;
}

int trie_node_contains_key(const struct trie_node *self, char ch) {
  return trie_node_get(self, ch) != NULL;
}

void trie_node_put(struct trie_node *self, char ch, struct trie_node *node) {
  int i = trie_node_get_index(ch);
  if (trie_node_in_range(self, i)) {
    self->links[i] = node;
  }
}

void trie_node_set_end(struct trie_node *self) { self->ended = 1; }

int trie_node_is_end(const struct trie_node *self) { return self->ended != 0; }

void trie_node_free(struct trie_node *node) {
  for (int i = 0; i < node->link_count; i += 1) {
    struct trie_node *link = node->links[i];
    if (link != NULL) {
      trie_node_free(link);
    }
  }

  free(node->links);
  free(node);
}

static int trie_node_get_index(char ch) { return ch - 'a'; }

static int trie_node_in_range(const struct trie_node *self, int i) {
  return (i >= 0) && (i < self->link_count);
}

static struct trie_node *trie_search_prefix(const struct trie *self,
                                            const char *prefix);
struct trie *new_trie() {
  struct trie_node *root = new_trie_node();
  struct trie *self = malloc(sizeof(struct trie));
  self->root = root;

  return self;
}

void trie_insert(struct trie *self, const char *word) {
  struct trie_node *node = self->root;
  for (int i = 0; word[i] != 0; i += 1) {
    char ch = word[i];
    if (!trie_node_contains_key(node, ch)) {
      trie_node_put(node, ch, new_trie_node());
    }

    node = trie_node_get(node, ch);
  }

  trie_node_set_end(node);
}

int trie_search(const struct trie *self, const char *word) {
  struct trie_node *node = trie_search_prefix(self, word);
  return (node != NULL) && trie_node_is_end(node);
}

int trie_starts_with(const struct trie *self, const char *prefix) {
  return trie_search_prefix(self, prefix) != NULL;
}

void trie_free(struct trie *self) {
  trie_node_free(self->root);
  free(self);
}

static struct trie_node *trie_search_prefix(const struct trie *self,
                                            const char *prefix) {
  struct trie_node *node = self->root;
  for (int i = 0; prefix[i] != 0; i += 1) {
    char ch = prefix[i];
    node = trie_node_get(node, ch);
    if (node == NULL) {
      return NULL;
    }
  }

  return node;
}
