#include <stdlib.h>

int min(int left, int right)
{
	return left < right ? left : right;
}

char **commonChars(char **words, int words_size, int *return_size)
{
	int counts[26] = {
		0,
	};

	for (int i = 0; words[0][i] != 0; i += 1) {
		const char ch = words[0][i];
		counts[ch - 'a'] += 1;
	}

	for (int i = 1; i < words_size; i += 1) {
		int temp_counts[26] = {
			0,
		};

		for (int j = 0; words[i][j] != 0; j += 1) {
			const char ch = words[i][j];
			temp_counts[ch - 'a'] += 1;
		}

		for (int j = 0; j < 26; j += 1) {
			counts[j] = min(counts[j], temp_counts[j]);
		}
	}

	int result_size = 0;
	for (int i = 0; i < 26; i += 1) {
		result_size += counts[i];
	}
	*return_size = result_size;

	char **result = malloc(sizeof(char *) * result_size);
	char **pos = result;
	for (int i = 0; i < 26; i += 1) {
		const int count = counts[i];
		for (int j = 0; j < count; j += 1) {
			char *str = calloc(2, sizeof(char));
			str[0] = (char)(i + 'a');
			*pos = str;
			pos += 1;
		}
	}

	return result;
}
