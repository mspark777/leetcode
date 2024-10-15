#include "./main.h"

int countBinarySubstrings(char *s)
{
	int count = 0;
	int prev = 0;
	int cur = 1;
	for (int i = 1; s[i] != 0; i += 1) {
		const char pch = s[i - 1];
		const char ch = s[i];
		if (ch == pch) {
			cur += 1;
			continue;
		}

		count += prev < cur ? prev : cur;
		prev = cur;
		cur = 1;
	}

	return count + (prev < cur ? prev : cur);
}
