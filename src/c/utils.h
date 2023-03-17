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

struct trie_node {
  struct trie_node **links;
  int link_count;
  int ended;
};

struct trie_node *new_trie_node();
struct trie_node *trie_node_get(const struct trie_node *self, char ch);
int trie_node_contains_key(const struct trie_node *self, char ch);
void trie_node_put(struct trie_node *self, char ch, struct trie_node *node);
void trie_node_set_end(struct trie_node *self);
int trie_node_is_end(const struct trie_node *self);
void trie_node_free(struct trie_node *node);

struct trie {
  struct trie_node *root;
};

struct trie *new_trie();
void trie_insert(struct trie *self, const char *word);
int trie_search(const struct trie *self, const char *word);
int trie_starts_with(const struct trie *self, const char *prefix);
void trie_free(struct trie *self);

#endif
