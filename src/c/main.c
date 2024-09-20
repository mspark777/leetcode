#include "./main.h"

char *shortestPalindrome(char *s)
{
	long long hash_base = 29;
	long long mod_value = 1e9 + 7;
	long long forward_hash = 0;
	long long reverse_hash = 0;
	long long power_value = 1;
	long long palindrome_end_index = -1;

	int slen = 0;
	for (int i = 0; s[i] != 0; i += 1) {
		slen += 1;

		char ch = s[i];
		int j = ch - 'a';
		forward_hash = (forward_hash * hash_base + (j + 1)) % mod_value;

		reverse_hash =
			(reverse_hash + (j + 1) * power_value) % mod_value;
		power_value = (power_value * hash_base) % mod_value;

		if (forward_hash == reverse_hash) {
			palindrome_end_index = i;
		}
	}

	int suffix_len = slen - (palindrome_end_index + 1);
	char *suffix = s + palindrome_end_index + 1;
	char *result = calloc(slen + suffix_len + 1, sizeof(char));

	for (int i = 0; i < suffix_len; i += 1) {
		result[i] = suffix[suffix_len - (i + 1)];
	}

	strcpy(result + suffix_len, s);

	return result;
}
