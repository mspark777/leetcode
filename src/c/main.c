#include "./main.h"

int countSeniors(char **details, int details_size)
{
	int result = 0;
	for (int i = 0; i < details_size; i += 1) {
		const char *d = details[i];
		const int left = (int)(d[11] - '0');
		const int right = (int)(d[12] - '0');
		const int age = left * 10 + right;
		result += age > 60;
	}

	return result;
}
