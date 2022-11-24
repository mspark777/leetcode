package main

import (
	"fmt"
)

func dfs(seens map[string]bool, board [][]byte, row, col, length int, word string) bool {
	if length == len(word) {
		return true
	}

	if (row < 0) || (col < 0) {
		return false
	} else if (row >= len(board)) || (col >= len(board[0])) {
		return false
	}

	seen := fmt.Sprint(row, '|', col)
	if _, ok := seens[seen]; ok {
		return false
	} else if board[row][col] != word[length] {
		return false
	}

	seens[seen] = true
	found := dfs(seens, board, row+1, col, length+1, word) ||
		dfs(seens, board, row-1, col, length+1, word) ||
		dfs(seens, board, row, col+1, length+1, word) ||
		dfs(seens, board, row, col-1, length+1, word)

	delete(seens, seen)

	return found
}

func exist(board [][]byte, word string) bool {
	for r, row := range board {
		for c, w := range row {
			if w == word[0] {
				seens := map[string]bool{}
				if dfs(seens, board, r, c, 0, word) {
					return true
				}
			}
		}
	}

	return false
}

type input struct {
	board [][]byte
	word  string
}

func main() {
	inputs := []input{
		{
			board: [][]byte{{'A', 'B', 'C', 'E'}, {'S', 'F', 'C', 'S'}, {'A', 'D', 'E', 'E'}},
			word:  "ABCCED",
		},
		{
			board: [][]byte{{'A', 'B', 'C', 'E'}, {'S', 'F', 'C', 'S'}, {'A', 'D', 'E', 'E'}},
			word:  "SEE",
		},
		{
			board: [][]byte{{'A', 'B', 'C', 'E'}, {'S', 'F', 'C', 'S'}, {'A', 'D', 'E', 'E'}},
			word:  "ABCB",
		},
	}

	for _, input := range inputs {
		board := input.board
		word := input.word
		result := exist(board, word)
		fmt.Println(result)
	}
}
