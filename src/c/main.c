#include "./main.h"

bool is_palindrome(char *s, int left, int right)
{
	while (left < right) {
		if (s[left] != s[right]) {
			return 0;
		}
		left += 1;
		right -= 1;
	}

	return 1;
}

bool validPalindrome(char *s)
{
	int left = 0;
	int right = strlen(s) - 1;
	while (left < right) {
		if (s[left] != s[right]) {
			return is_palindrome(s, left + 1, right) ||
			       is_palindrome(s, left, right - 1);
		}

		left += 1;
		right -= 1;
	}

	return 1;
}
