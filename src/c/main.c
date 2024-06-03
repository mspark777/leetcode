int appendCharacters(char *s, char *t)
{
	while (*s) {
		if (*t == *s) {
			t += 1;
		}
		s += 1;
		if (*t == 0) {
			return 0;
		}
	}

	char *st = t;
	while (*t) {
		t += 1;
	}

	return t - st;
}
