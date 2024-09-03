#include "./main.h"

int getLucky(char *s, int k)
{
	int result = 0;
	for (int i = 0; s[i] != 0; i += 1) {
		int pos = ((int)(s[i] - 'a')) + 1;
		while (pos > 0) {
			result += pos % 10;
			pos /= 10;
		}
	}

	for (int i = 1; i < k; i += 1) {
		int temp = 0;
		while (result > 0) {
			temp += result % 10;
			result /= 10;
		}
		result = temp;
	}

	return result;
}
