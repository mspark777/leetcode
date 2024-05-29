int len(const char *s)
{
	int l = 0;
	while (s[l] != 0) {
		l += 1;
	}

	return l;
}
int numSteps(char *s)
{
	int N = len(s);
	int operations = 0;
	int carry = 0;

	for (int i = N - 1; i > 0; i -= 1) {
		const int n = (s[i] - '0') + carry;

		if (n & 1) {
			operations += 2;
			carry = 1;
		} else {
			operations += 1;
		}
	}

	return operations + carry;
}
