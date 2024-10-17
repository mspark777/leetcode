#include "./main.h"

bool isOneBitCharacter(int *bits, int bits_size)
{
	const int last = bits_size - 1;
	int i = 0;
	while (i < last) {
		i += bits[i] + 1;
	}

	return i == last;
}
