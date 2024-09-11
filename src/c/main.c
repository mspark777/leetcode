#include "./main.h"

int minBitFlips(int start, int goal)
{
	int xor_result = start ^ goal;
	int result = 0;

	while (xor_result != 0) {
		xor_result &= xor_result - 1;
		result += 1;
	}

	return result;
}
