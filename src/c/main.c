#include <stdio.h>

int compareVersion(char *version1, char *version2)
{
	int i1 = 0;
	int i2 = 0;
	int v1 = 0;
	int v2 = 0;

	while (1) {
		while (1) {
			const char ch1 = version1[i1];
			if (ch1 == 0) {
				break;
			} else if (ch1 == '.') {
				i1 += 1;
				break;
			} else {
				v1 = v1 * 10 + (ch1 - '0');
				i1 += 1;
			}
		}

		while (1) {
			const char ch2 = version2[i2];
			if (ch2 == 0) {
				break;
			} else if (ch2 == '.') {
				i2 += 1;
				break;
			} else {
				v2 = v2 * 10 + (ch2 - '0');
				i2 += 1;
			}
		}

		if (v1 < v2) {
			break;
		} else if (v1 > v2) {
			break;
		} else if ((version1[i1] == 0) && (version2[i2] == 0)) {
			break;
		} else {
			v1 = 0;
			v2 = 0;
		}
	}

	if (v1 < v2) {
		return -1;
	} else if (v1 > v2) {
		return 1;
	}

	return 0;
}

int main()
{
	printf("%d\n", compareVersion("1.01", "1.001"));
	printf("%d\n", compareVersion("1.0", "1.0.0"));
	printf("%d\n", compareVersion("0.1", "1.1"));
}
