#include "./main.h"

int maximumGain(char *s, int x, int y)
{
	const int n = strlen(s);
	if (x < y) {
		int temp = x;
		x = y;
		y = temp;

		int left = 0;
		int right = n - 1;
		while (left < right) {
			const char l = s[left];
			const char r = s[right];
			s[right] = l;
			s[left] = r;

			left += 1;
			right -= 1;
		}
	}

	int a_count = 0;
	int b_count = 0;
	int result = 0;

	for (int i = 0; i < n; i += 1) {
		const char ch = s[i];

		if (ch == 'a') {
			a_count += 1;
		} else if (ch == 'b') {
			if (a_count > 0) {
				a_count -= 1;
				result += x;
			} else {
				b_count += 1;
			}
		} else {
			result += (a_count < b_count ? a_count : b_count) * y;
			a_count = 0;
			b_count = 0;
		}
	}

	result += (a_count < b_count ? a_count : b_count) * y;
	return result;
}
