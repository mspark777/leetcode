#include <string.h>

int abs(int i)
{
	return i >= 0 ? i : -i;
}

int max(int left, int right)
{
	return left < right ? right : left;
}

int equalSubstring(char *s, char *t, int max_cost)
{
	const int n = strlen(s);
	int max_len = 0;
	int start = 0;
	int curr_cost = 0;

	for (int i = 0; i < n; i += 1) {
		curr_cost += abs(s[i] - t[i]);
		while (curr_cost > max_cost) {
			curr_cost -= abs(s[start] - t[start]);
			start += 1;
		}

		max_len = max(max_len, i - start + 1);
	}

	return max_len;
}
