#include "./main.h"

int findTheLongestSubstring(char *s)
{
	const char A = 'a';
	const char E = 'e';
	const char I = 'i';
	const char O = 'o';
	const char U = 'u';

	int prefix_xor = 0;
	int char_map[26] = {
		0,
	};
	char_map[0] = 1;
	char_map[E - A] = 2;
	char_map[I - A] = 4;
	;
	char_map[O - A] = 8;
	char_map[U - A] = 16;

	int mp[32] = {};
	memset(mp, -1, sizeof(mp));
	int result = 0;

	for (int i = 0; s[i] != 0; i += 1) {
		prefix_xor ^= char_map[s[i] - A];
		if (mp[prefix_xor] == -1 && prefix_xor != 0) {
			mp[prefix_xor] = i;
		}

		int len = i - mp[prefix_xor];
		if (result < len) {
			result = len;
		}
	}

	return result;
}
