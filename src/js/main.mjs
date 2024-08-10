/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

function isValidCell(grid, row, col) {
  const n = grid.length;
  return row >= 0 && col >= 0 && row < n && col < n && grid[row][col] === 0;
}

function floodFill(grid, row, col) {
  const directions = [
    [0, 1],
    [0, -1],
    [1, 0],
    [-1, 0],
  ];
  const queue = [[row, col]];
  grid[row][col] = 1;

  for (let cell = queue.shift(); cell != null; cell = queue.shift()) {
    const [r, c] = cell;
    for (const [dr, dc] of directions) {
      const newRow = r + dr;
      const newCol = c + dc;
      if (isValidCell(grid, newRow, newCol)) {
        grid[newRow][newCol] = 1;
        queue.push([newRow, newCol]);
      }
    }
  }
}

/**
 * @param {string[]} grid
 * @return {number}
 */
function regionsBySlashes(grid) {
  const gridSize = grid.length;
  const expandedSize = gridSize * 3;
  const expandedGrid = [];
  for (let i = 0; i < expandedSize; i += 1) {
    const g = [];
    for (let j = 0; j < expandedSize; j += 1) {
      g.push(0);
    }
    expandedGrid.push(g);
  }

  for (let i = 0; i < gridSize; i += 1) {
    const baseRow = i * 3;
    for (let j = 0; j < gridSize; j += 1) {
      const baseCol = j * 3;

      if (grid[i][j] === "\\") {
        expandedGrid[baseRow][baseCol] = 1;
        expandedGrid[baseRow + 1][baseCol + 1] = 1;
        expandedGrid[baseRow + 2][baseCol + 2] = 1;
      } else if (grid[i][j] === "/") {
        expandedGrid[baseRow][baseCol + 2] = 1;
        expandedGrid[baseRow + 1][baseCol + 1] = 1;
        expandedGrid[baseRow + 2][baseCol] = 1;
      }
    }
  }
  let result = 0;
  for (let i = 0; i < expandedSize; i += 1) {
    for (let j = 0; j < expandedSize; j += 1) {
      if (expandedGrid[i][j] === 0) {
        floodFill(expandedGrid, i, j);
        result += 1;
      }
    }
  }

  return result;
}

function main() {
  const inputs = [
    [" /", "/ "],
    [" /", "  "],
    ["/\\", "\\/"],
  ];

  for (const grid of inputs) {
    const result = regionsBySlashes(grid);
    console.log(result);
  }
}
main();
