#include "./main.h"

int to_int(char *nums)
{
	int n = 0;
	int i = 0;
	int minus = 0;
	if (nums[i] == '-') {
		minus = 1;
		i += 1;
	}

	while (nums[i] != 0) {
		n *= 10;
		n += nums[i] - '0';
		i += 1;
	}

	return minus ? -n : n;
}

int calPoints(char **operations, int operations_size)
{
	int top_idx = -1;
	int stack[1000] = {
		0,
	};

	for (int i = 0; i < operations_size; i += 1) {
		char *oper = operations[i];
		char ch = oper[0];
		if (ch == '+') {
			int top = stack[top_idx];
			top_idx -= 1;
			int new_top = top + stack[top_idx];
			top_idx += 1;
			stack[top_idx] = top;
			top_idx += 1;
			stack[top_idx] = new_top;
		} else if (ch == 'D') {
			int top = stack[top_idx];
			top_idx += 1;
			stack[top_idx] = top * 2;
		} else if (ch == 'C') {
			top_idx -= 1;
		} else {
			top_idx += 1;
			stack[top_idx] = to_int(oper);
		}
	}

	int result = 0;
	for (int i = 0; i <= top_idx; i += 1) {
		result += stack[i];
	}
	return result;
}
