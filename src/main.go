package main

import (
	"fmt"
)

func nearestExit(maze [][]byte, entrance []int) int {
	const WALL = byte('+')
	rowCount := len(maze)
	colCount := len(maze[0])
	lastRow := rowCount - 1
	lastCol := colCount - 1
	dirs := [][]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}

	queue := [][]int{{entrance[0], entrance[1], 0}}
	maze[entrance[0]][entrance[1]] = WALL
	for len(queue) > 0 {
		front := queue[0]
		queue = queue[1:]
		row := front[0]
		col := front[1]
		steps := front[2]

		nextSteps := steps + 1
		for _, dir := range dirs {
			nextRow := row + dir[0]
			nextCol := col + dir[1]

			if nextRow < 0 {
				continue
			} else if nextRow >= rowCount {
				continue
			} else if nextCol < 0 {
				continue
			} else if nextCol >= colCount {
				continue
			} else if maze[nextRow][nextCol] == WALL {
				continue
			}

			if nextRow == 0 {
				return nextSteps
			} else if nextRow == lastRow {
				return nextSteps
			} else if nextCol == 0 {
				return nextSteps
			} else if nextCol == lastCol {
				return nextSteps
			}

			maze[nextRow][nextCol] = WALL
			queue = append(queue, []int{nextRow, nextCol, nextSteps})
		}
	}

	return -1
}

type input struct {
	maze     [][]byte
	entrance []int
}

func main() {
	inputs := []input{
		{
			maze: [][]byte{
				{
					'+',
					'+',
					'.',
					'+',
				},
				{
					'.',
					'.',
					'.',
					'+',
				},
				{
					'+',
					'+',
					'+',
					'.',
				},
			},
			entrance: []int{1, 2},
		},
		{
			maze:     [][]byte{{'+', '+', '+'}, {'.', '.', '.'}, {'+', '+', '+'}},
			entrance: []int{1, 0},
		},
		{
			maze:     [][]byte{{'.', '+'}},
			entrance: []int{0, 0},
		},
	}

	for _, input := range inputs {
		maze := input.maze
		entrance := input.entrance
		result := nearestExit(maze, entrance)
		fmt.Println(result)
	}
}
