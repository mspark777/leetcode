#include "./main.h"

bool checkInclusion(char *s1, char *s2)
{
	const int s1len = strlen(s1);
	const int s2len = strlen(s2);

	if (s1len > s2len) {
		return 0;
	}

	int *chars1 = calloc(sizeof(int), 26);
	int *chars2 = calloc(sizeof(int), 26);

	for (int i = 0; i < s1len; i += 1) {
		chars1[s1[i] - 'a'] += 1;
		chars2[s2[i] - 'a'] += 1;
	}

	int count = 0;
	for (int i = 0; i < 26; i += 1) {
		count += chars1[i] == chars2[i];
	}

	const int loop = s2len - s1len;
	for (int i = 0; i < loop; i += 1) {
		if (count == 26) {
			break;
		}

		int l = s2[i] - 'a';
		int r = s2[i + s1len] - 'a';
		chars2[r] += 1;
		if (chars2[r] == chars1[r]) {
			count += 1;
		} else if (chars2[r] == chars1[r] + 1) {
			count -= 1;
		}

		chars2[l] -= 1;
		if (chars2[l] == chars1[l]) {
			count += 1;
		} else if (chars2[l] == chars1[l] - 1) {
			count -= 1;
		}
	}

	free(chars1);
	free(chars2);
	return count == 26;
}
