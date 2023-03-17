#include "./utils.h"
#include <stdio.h>

int main() {
  struct trie *trie = new_trie();

  const char *apple = "apple";
  const char *app = "app";
  trie_insert(trie, apple);
  printf("%d\n", trie_search(trie, apple));
  printf("%d\n", trie_search(trie, app));
  printf("%d\n", trie_starts_with(trie, app));
  trie_insert(trie, app);
  printf("%d\n", trie_search(trie, app));

  trie_free(trie);
  return 0;
}
