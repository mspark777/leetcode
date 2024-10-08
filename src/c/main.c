#include "./main.h"

int minSwaps(char *s)
{
	int top = 0;
	for (int i = 0; s[i] != 0; i += 1) {
		const char ch = s[i];
		if (ch == '[') {
			top += 1;
		} else if (top > 0) {
			top -= 1;
		}
	}

	return (top + 1) / 2;
}
