#include "./main.h"

int strangePrinter(char *s)
{
	const int n = strlen(s);
	int *dp = calloc(n * n, sizeof(int));
	for (int l = 1; l <= n; l += 1) {
		for (int left = 0; left <= n - l; left += 1) {
			const int right = left + l - 1;
			int j = -1;
			dp[left * n + right] = n;
			for (int i = left; i < right; i += 1) {
				if (s[i] != s[right] && j == -1) {
					j = i;
				}

				if (j != -1) {
					const int lmin = dp[left * n + right];
					const int rmin =
						1 + dp[j * n + i] +
						dp[(i + 1) * n + right];
					dp[left * n + right] =
						lmin < rmin ? lmin : rmin;
				}
			}

			if (j == -1) {
				dp[left * n + right] = 0;
			}
		}
	}

	const int result = dp[n - 1] + 1;
	free(dp);
	dp = NULL;

	return result;
}
