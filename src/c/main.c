#include "./main.h"

int minimumDeletions(char *s)
{
	int result = 0;
	int b_count = 0;

	for (int i = 0; s[i] != '\0'; i += 1) {
		const char ch = s[i];
		if (ch == 'b') {
			b_count += 1;
		} else {
			const int min_deletions = result + 1;
			if (min_deletions < b_count) {
				result = min_deletions;
			} else {
				result = b_count;
			}
		}
	}

	return result;
}
