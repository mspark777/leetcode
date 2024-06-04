
int longestPalindrome(char *s)
{
	int lowers[26] = {
		0,
	};
	int uppers[26] = {
		0,
	};

	for (int i = 0; s[i] != 0; i += 1) {
		const char ch = s[i];
		if (('a' <= ch) && (ch <= 'z')) {
			lowers[ch - 'a'] += 1;
		} else {
			uppers[ch - 'A'] += 1;
		}
	}

	int result = 0;
	for (int i = 0; i < 26; i += 1) {
		const int count = lowers[i];
		if (count < 1) {
			continue;
		}

		const int result_odd = result & 1;
		const int count_odd = count & 1;
		result += count - (result_odd && count_odd);
	}

	for (int i = 0; i < 26; i += 1) {
		const int count = uppers[i];
		if (count < 1) {
			continue;
		}

		const int result_odd = result & 1;
		const int count_odd = count & 1;
		result += count - (result_odd && count_odd);
	}

	return result;
}
