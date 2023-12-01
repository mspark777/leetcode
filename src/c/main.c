#include <stdio.h>

#define bool int

bool arrayStringsAreEqual(char** word1, int word1_size, char** word2, int word2_size) {
  int w_idx1 = 0;
  int w_idx2 = 0;
  int ch_idx1 = 0;
  int ch_idx2 = 0;

  while (w_idx1 < word1_size && w_idx2 < word2_size) {
    char *w1 = word1[w_idx1];
    char *w2 = word2[w_idx2];
    char ch1 = w1[ch_idx1];
    char ch2 = w2[ch_idx2];

    if (ch1 == 0) {
      ch_idx1 = 0;
      w_idx1 += 1;
    } 

    if (ch2 == 0) {
      ch_idx2 = 0;
      w_idx2 += 1;
    }

    if (ch1 == 0 || ch2 == 0) {
      continue;
    }

    if (ch1 != ch2) {
      return 0;
    }

    ch_idx1 += 1;
    ch_idx2 += 1;
  }

  return w_idx1 == word1_size && w_idx2 == word2_size;
}


int main() {
  char* word0_0 = "ab";
  char* word0_1 = "c";
  char *word0[] = {word0_0, word0_1};

  char* word1_0 = "a";
  char* word1_1 = "bc";
  char *word1[] = {word1_0, word1_1};
  printf("%d\n", arrayStringsAreEqual(word0, 2, word1, 2));

  return 0;

}
