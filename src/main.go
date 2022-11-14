package main

import "fmt"

type helper struct {
	unimemo map[int]int
	islands int
}

func (helper *helper) find(x int) int {
	if _, ok := helper.unimemo[x]; !ok {
		helper.unimemo[x] = x
		helper.islands += 1
	}

	p := helper.unimemo[x]
	if x != p {
		helper.unimemo[x] = helper.find(p)
	}

	return helper.unimemo[x]
}

func (helper *helper) uni(x, y int) {
	x = helper.find(x)
	y = helper.find(y)
	if x != y {
		helper.unimemo[x] = y
		helper.islands -= 1
	}
}

func removeStones(stones [][]int) int {
	helper := &helper{
		unimemo: map[int]int{},
		islands: 0,
	}

	for _, stone := range stones {
		helper.uni(stone[0], ^stone[1])
	}

	return len(stones) - helper.islands
}

func main() {
	inputs := [][][]int{
		{{0, 0}, {0, 1}, {1, 0}, {1, 2}, {2, 1}, {2, 2}},
		{{0, 0}, {0, 2}, {1, 1}, {2, 0}, {2, 2}},
		{{0, 0}, {0, 1}, {1, 0}, {1, 1}, {2, 1}, {2, 2}, {3, 2}, {3, 3}, {3, 4}, {4, 3}, {4, 4}},
	}

	for _, stones := range inputs {
		result := removeStones(stones)
		fmt.Println(result)
	}
}
