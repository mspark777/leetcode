#include "./main.h"

char *longestDiverseString(int a, int b, int c)
{
	int curr_a = 0;
	int curr_b = 0;
	int curr_c = 0;
	int pos = 0;
	const int result_length = a + b + c;
	char *result = calloc(sizeof(char), result_length + 1);

	for (int i = 0; i < result_length; i += 1) {
		if ((a >= b && a >= c && curr_a < 2) ||
		    (a > 0 && (curr_b == 2 || curr_c == 2))) {
			result[pos] = 'a';
			pos += 1;
			a -= 1;
			curr_a += 1;
			curr_b = 0;
			curr_c = 0;
		} else if ((b >= a && b >= c && curr_b < 2) ||
			   (b > 0 && (curr_c == 2 || curr_a == 2))) {
			result[pos] = 'b';
			pos += 1;
			b -= 1;
			curr_b += 1;
			curr_a = 0;
			curr_c = 0;
		} else if ((c >= a && c >= b && curr_c < 2) ||
			   (c > 0 && (curr_a == 2 || curr_b == 2))) {
			result[pos] = 'c';
			pos += 1;
			c -= 1;
			curr_c += 1;
			curr_a = 0;
			curr_b = 0;
		}
	}

	return result;
}
