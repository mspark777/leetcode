#include "./main.h"

int countConsistentStrings(char *allowed, char **words, int words_size)
{
	int bits = 0;
	for (int i = 0; allowed[i] != 0; i += 1) {
		bits |= 1 << (allowed[i] - 'a');
	}

	int result = 0;
	for (int i = 0; i < words_size; i += 1) {
		int is_consistent = 1;
		char *word = words[i];

		for (int j = 0; word[j] != 0; j += 1) {
			int bit = (bits >> (word[j] - 'a')) & 1;
			if (bit == 0) {
				is_consistent = 0;
				break;
			}
		}

		result += is_consistent;
	}

	return result;
}
