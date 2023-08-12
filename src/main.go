package main

import (
	"fmt"
)

func uniquePathsWithObstacles(obstacleGrid [][]int) int {
	const OBSTACLE int = 1
	rowCount := len(obstacleGrid)
	colCount := len(obstacleGrid[0])
	countGrid := make([][]int, rowCount)
	for r := 0; r < rowCount; r += 1 {
		countGrid[r] = make([]int, colCount)
	}

	for r := 0; r < rowCount; r += 1 {
		for c := 0; c < colCount; c += 1 {
			if obstacleGrid[r][c] == OBSTACLE {
				continue
			}

			if (r + c) == 0 {
				countGrid[r][c] = 1
			} else if r == 0 {
				countGrid[r][c] = countGrid[r][c-1]
			} else if c == 0 {
				countGrid[r][c] = countGrid[r-1][c]
			} else {
				countGrid[r][c] = countGrid[r-1][c] + countGrid[r][c-1]
			}
		}
	}

	return countGrid[rowCount-1][colCount-1]
}

func main() {
	inputs := [][][]int{
		{{0, 0, 0}, {0, 1, 0}, {0, 0, 0}},
		{{0, 1}, {0, 0}},
	}

	for _, input := range inputs {
		result := uniquePathsWithObstacles(input)
		fmt.Println(result)
	}
}
