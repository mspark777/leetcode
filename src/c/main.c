#include "./main.h"

long long minimumSteps(char *s)
{
	long long result = 0;
	long long black_ball_count = 0;

	for (int i = 0; s[i] != 0; i += 1) {
		const char ch = s[i];
		if (ch == '0') {
			result += black_ball_count;
		} else {
			black_ball_count += 1;
		}
	}

	return result;
}
