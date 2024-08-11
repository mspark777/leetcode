/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

function isValidLandCell(grid, row, col) {
  const rows = grid.length;
  const cols = grid[0].length;
  return (
    row >= 0 && col >= 0 && row < rows && col < cols && grid[row][col] == 1
  );
}

function findArticulationPoints(
  grid,
  row,
  col,
  discoveryTime,
  lowestReachable,
  parentCell,
  apInfo,
) {
  const cols = grid[0].length;
  discoveryTime[row][col] = apInfo.time;
  apInfo.time++;
  lowestReachable[row][col] = discoveryTime[row][col];
  let children = 0;
  const directions = [
    [0, 1],
    [1, 0],
    [0, -1],
    [-1, 0],
  ];

  for (const [dr, dc] of directions) {
    const newRow = row + dr;
    const newCol = col + dc;

    if (!isValidLandCell(grid, newRow, newCol)) {
      continue;
    }

    if (discoveryTime[newRow][newCol] === -1) {
      children += 1;
      parentCell[newRow][newCol] = row * cols + col;
      findArticulationPoints(
        grid,
        newRow,
        newCol,
        discoveryTime,
        lowestReachable,
        parentCell,
        apInfo,
      );

      lowestReachable[row][col] = Math.min(
        lowestReachable[row][col],
        lowestReachable[newRow][newCol],
      );

      if (
        lowestReachable[newRow][newCol] >= discoveryTime[row][col] &&
        parentCell[row][col] !== -1
      ) {
        apInfo.hasArticulationPoint = true;
      }
    } else if (newRow * cols + newCol != parentCell[row][col]) {
      lowestReachable[row][col] = Math.min(
        lowestReachable[row][col],
        discoveryTime[newRow][newCol],
      );
    }
  }

  if (parentCell[row][col] == -1 && children > 1) {
    apInfo.hasArticulationPoint = true;
  }
}

/**
 * @param {number[][]} grid
 * @return {number}
 */
function minDays(grid) {
  const rows = grid.length;
  const cols = grid[0].length;
  const apInfo = { hasArticulationPoint: false, time: 0 };
  let landCells = 0;
  let islandCount = 0;

  const discoveryTime = [];
  const lowestReachable = [];
  const parentCell = [];

  for (let i = 0; i < rows; i += 1) {
    const discoveryTimeRow = [];
    const lowestReachableRow = [];
    const islandCountRow = [];
    for (let j = 0; j < cols; j += 1) {
      discoveryTimeRow.push(-1);
      lowestReachableRow.push(-1);
      islandCountRow.push(-1);
    }

    discoveryTime.push(discoveryTimeRow);
    lowestReachable.push(lowestReachableRow);
    parentCell.push(islandCountRow);
  }

  for (let i = 0; i < rows; i++) {
    for (let j = 0; j < cols; j++) {
      if (grid[i][j] == 1) {
        landCells++;
        if (discoveryTime[i][j] === -1) {
          findArticulationPoints(
            grid,
            i,
            j,
            discoveryTime,
            lowestReachable,
            parentCell,
            apInfo,
          );
          islandCount++;
        }
      }
    }
  }

  if (islandCount === 0 || islandCount >= 2) {
    return 0;
  } else if (landCells === 1) {
    return 1;
  } else if (apInfo.hasArticulationPoint) {
    return 1;
  }

  return 2;
}

function main() {
  const inputs = [
    [
      [0, 1, 1, 0],
      [0, 1, 1, 0],
      [0, 0, 0, 0],
    ],
    [[1, 1]],
  ];

  for (const input of inputs) {
    const result = minDays(input);
    console.log(result);
  }
}
main();
