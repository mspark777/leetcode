#include "./main.h"

char *toLowerCase(char *s)
{
	const char TERM = 'a' - 'A';
	char *result = calloc(strlen(s) + 1, sizeof(char));
	for (int i = 0; s[i] != 0; i++) {
		const char ch = s[i];
		result[i] = ('A' <= ch && ch <= 'Z') ? ch + TERM : ch;
	}

	return result;
}
