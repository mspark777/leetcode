package main

import (
	"fmt"
)

func hasKey(seens map[string]bool, s string) bool {
	_, ok := seens[s]
	return ok
}

func isValidSudoku(board [][]byte) bool {
	seens := map[string]bool{}
	for r := 0; r < 9; r += 1 {
		for c := 0; c < 9; c += 1 {
			n := board[r][c]
			if n == '.' {
				continue
			}

			ns := fmt.Sprintf("(%v)", n)
			row := fmt.Sprint(ns, r)
			col := fmt.Sprint(c, ns)
			cross := fmt.Sprint(r/3, ns, c/3)

			if hasKey(seens, row) || hasKey(seens, col) || hasKey(seens, cross) {
				return false
			}

			seens[row] = true
			seens[col] = true
			seens[cross] = true
		}
	}

	return true
}

func main() {
	inputs := [][][]byte{
		{
			{'5', '3', '.', '.', '7', '.', '.', '.', '.'},
			{'6', '.', '.', '1', '9', '5', '.', '.', '.'},
			{'.', '9', '8', '.', '.', '.', '.', '6', '.'},
			{'8', '.', '.', '.', '6', '.', '.', '.', '3'},
			{'4', '.', '.', '8', '.', '3', '.', '.', '1'},
			{'7', '.', '.', '.', '2', '.', '.', '.', '6'},
			{'.', '6', '.', '.', '.', '.', '2', '8', '.'},
			{'.', '.', '.', '4', '1', '9', '.', '.', '5'},
			{'.', '.', '.', '.', '8', '.', '.', '7', '9'},
		},
		{
			{'8', '3', '.', '.', '7', '.', '.', '.', '.'},
			{'6', '.', '.', '1', '9', '5', '.', '.', '.'},
			{'.', '9', '8', '.', '.', '.', '.', '6', '.'},
			{'8', '.', '.', '.', '6', '.', '.', '.', '3'},
			{'4', '.', '.', '8', '.', '3', '.', '.', '1'},
			{'7', '.', '.', '.', '2', '.', '.', '.', '6'},
			{'.', '6', '.', '.', '.', '.', '2', '8', '.'},
			{'.', '.', '.', '4', '1', '9', '.', '.', '5'},
			{'.', '.', '.', '.', '8', '.', '.', '7', '9'},
		},
	}

	for _, board := range inputs {
		result := isValidSudoku(board)
		fmt.Println(result)
	}
}
