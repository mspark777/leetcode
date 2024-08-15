#include "./main.h"

bool lemonadeChange(int *bills, int bills_size)
{
	int five_dollar = 0;
	int ten_dollar = 0;
	for (int i = 0; i < bills_size; i += 1) {
		const int bill = bills[i];
		if (bill == 5) {
			five_dollar += 1;
		} else if (bill == 10) {
			if (five_dollar > 0) {
				five_dollar -= 1;
				ten_dollar += 1;
			} else {
				return 0;
			}
		} else {
			if (ten_dollar > 0 && five_dollar > 0) {
				ten_dollar -= 1;
				five_dollar -= 1;
			} else if (five_dollar > 2) {
				five_dollar -= 3;
			} else {
				return 0;
			}
		}
	}

	return 1;
}
