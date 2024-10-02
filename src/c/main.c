#include "./main.h"

bool judgeCircle(char *moves)
{
	int x = 0;
	int y = 0;
	for (int i = 0; moves[i] != 0; i += 1) {
		const char ch = moves[i];
		if (ch == 'U') {
			y += 1;
		} else if (ch == 'D') {
			y -= 1;
		} else if (ch == 'R') {
			x += 1;
		} else if (ch == 'L') {
			x -= 1;
		}
	}

	return (x == 0) && (y == 0);
}
