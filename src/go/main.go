package main

import "fmt"

func numRookCaptures(board [][]byte) int {
	x0 := 0
	y0 := 0
	found := false

	for y := 0; y < 8; y += 1 {
		for x := 0; x < 8; x += 1 {
			if board[y][x] == 'R' {
				x0 = x
				y0 = y
				found = true
				break
			}
		}

		if found {
			break
		}
	}

	result := 0
	directions := [][]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	for _, dir := range directions {
		x := x0 + dir[0]
		y := y0 + dir[1]
		for (0 <= x) && (x < 8) && (0 <= y) && (y < 8) {
			if board[y][x] == 'p' {
				result += 1
			}

			if board[y][x] != '.' {
				break
			}

			x += dir[0]
			y += dir[1]
		}
	}

	return result
}

type input struct {
	board [][]byte
}

func main() {
	inputs := []input{
		{
			board: [][]byte{{'.', '.', '.', '.', '.', '.', '.', '.'}, {'.', '.', '.', 'p', '.', '.', '.', '.'}, {'.', '.', '.', 'R', '.', '.', '.', 'p'}, {'.', '.', '.', '.', '.', '.', '.', '.'}, {'.', '.', '.', '.', '.', '.', '.', '.'}, {'.', '.', '.', 'p', '.', '.', '.', '.'}, {'.', '.', '.', '.', '.', '.', '.', '.'}, {'.', '.', '.', '.', '.', '.', '.', '.'}},
		},
		{
			board: [][]byte{{'.', '.', '.', '.', '.', '.', '.', '.'}, {'.', 'p', 'p', 'p', 'p', 'p', '.', '.'}, {'.', 'p', 'p', 'B', 'p', 'p', '.', '.'}, {'.', 'p', 'B', 'R', 'B', 'p', '.', '.'}, {'.', 'p', 'p', 'B', 'p', 'p', '.', '.'}, {'.', 'p', 'p', 'p', 'p', 'p', '.', '.'}, {'.', '.', '.', '.', '.', '.', '.', '.'}, {'.', '.', '.', '.', '.', '.', '.', '.'}},
		},
		{
			board: [][]byte{{'.', '.', '.', '.', '.', '.', '.', '.'}, {'.', '.', '.', 'p', '.', '.', '.', '.'}, {'.', '.', '.', 'p', '.', '.', '.', '.'}, {'p', 'p', '.', 'R', '.', 'p', 'B', '.'}, {'.', '.', '.', '.', '.', '.', '.', '.'}, {'.', '.', '.', 'B', '.', '.', '.', '.'}, {'.', '.', '.', 'p', '.', '.', '.', '.'}, {'.', '.', '.', '.', '.', '.', '.', '.'}},
		},
	}

	for _, input := range inputs {
		result := numRookCaptures(input.board)
		fmt.Println(result)
	}
}
