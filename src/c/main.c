int max(const int left, const int right)
{
	return left > right ? left : right;
}

int dfs_backtrack(int **grid, const int rows, const int cols, const int row,
		  const int col)
{
	if (row < 0 || col < 0 || row == rows || col == cols ||
	    grid[row][col] == 0) {
		return 0;
	}

	int amount = 0;
	int current_cell_gold = grid[row][col];
	grid[row][col] = 0;
	amount = max(amount, dfs_backtrack(grid, rows, cols, row, col - 1));
	amount = max(amount, dfs_backtrack(grid, rows, cols, row, col + 1));
	amount = max(amount, dfs_backtrack(grid, rows, cols, row - 1, col));
	amount = max(amount, dfs_backtrack(grid, rows, cols, row + 1, col));
	grid[row][col] = current_cell_gold;

	return amount + current_cell_gold;
}

int getMaximumGold(int **grid, int grid_size, int *grid_col_size)
{
	const int m = grid_size;
	const int n = *grid_col_size;
	int result = 0;
	for (int r = 0; r < m; r += 1) {
		for (int c = 0; c < n; c += 1) {
			result = max(result, dfs_backtrack(grid, m, n, r, c));
		}
	}
	return result;
}
