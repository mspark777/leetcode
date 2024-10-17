#include "./main.h"

char *to_string(int n, int len)
{
	char *s = calloc(sizeof(char), len + 1);
	for (int i = len - 1; i >= 0; i -= 1) {
		s[i] = (n % 10) + '0';
		n /= 10;
	}

	return s;
}

int get_string_length(int n)
{
	int len = 0;
	while (n > 0) {
		n /= 10;
		len += 1;
	}

	return len;
}

int to_int(char *s, int len)
{
	int n = 0;
	for (int i = 0; i < len; i += 1) {
		n *= 10;
		n += (int)(s[i] - '0');
	}

	return n;
}

int maximumSwap(int num)
{
	const int n = get_string_length(num);
	char *s = to_string(num, n);

	int max_digit_idx = -1;
	int swap_idx0 = -1;
	int swap_idx1 = -1;

	for (int i = n - 1; i >= 0; i -= 1) {
		if (max_digit_idx == -1 || s[i] > s[max_digit_idx]) {
			max_digit_idx = i;
		} else if (s[i] < s[max_digit_idx]) {
			swap_idx0 = i;
			swap_idx1 = max_digit_idx;
		}
	}

	if (swap_idx0 != -1 && swap_idx1 != -1) {
		const char temp = s[swap_idx0];
		s[swap_idx0] = s[swap_idx1];
		s[swap_idx1] = temp;
	}

	const int result = to_int(s, n);
	free(s);

	return result;
}
