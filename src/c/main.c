#include "./main.h"

int *luckyNumbers(int **matrix, int row_count, int *col_count, int *return_size)
{
	const int N = row_count;
	const int M = *col_count;

	int row_min_max = -1;
	for (int i = 0; i < N; i += 1) {
		int row_min = 1000000;
		for (int j = 0; j < M; j += 1) {
			const int cell = matrix[i][j];
			if (cell < row_min) {
				row_min = cell;
			}
		}

		if (row_min > row_min_max) {
			row_min_max = row_min;
		}
	}

	int col_max_min = 1000000;
	for (int i = 0; i < M; i += 1) {
		int col_max = -1;
		for (int j = 0; j < N; j += 1) {
			const int cell = matrix[j][i];
			if (cell > col_max) {
				col_max = cell;
			}
		}

		if (col_max < col_max_min) {
			col_max_min = col_max;
		}
	}

	int *result = malloc(sizeof(int));
	if (row_min_max == col_max_min) {
		*result = row_min_max;
		*return_size = 1;
	} else {
		*return_size = 0;
	}

	return result;
}
