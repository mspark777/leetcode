package main

import (
	"fmt"
)

func maxProbability(n int, edges [][]int, succProb []float64, start int, end int) float64 {
	maxProps := make([]float64, n)
	maxProps[start] = 1.0

	for i := 0; i < n-1; i += 1 {
		breakable := true
		for j, edge := range edges {
			u := edge[0]
			v := edge[1]
			prob := succProb[j]
			umax := maxProps[u] * prob
			if umax > maxProps[v] {
				maxProps[v] = umax
				breakable = false
			}

			vmax := maxProps[v] * prob
			if vmax > maxProps[u] {
				maxProps[u] = vmax
				breakable = false
			}
		}

		if breakable {
			break
		}
	}

	return maxProps[end]
}

type input struct {
	n        int
	edges    [][]int
	succProb []float64
	start    int
	end      int
}

func main() {
	inputs := []input{
		{n: 3, edges: [][]int{{0, 1}, {1, 2}, {0, 2}}, succProb: []float64{0.5, 0.5, 0.2}, start: 0, end: 2},
		{n: 3, edges: [][]int{{0, 1}, {1, 2}, {0, 2}}, succProb: []float64{0.5, 0.5, 0.3}, start: 0, end: 2},
		{n: 3, edges: [][]int{{0, 1}}, succProb: []float64{0.5}, start: 0, end: 2},
	}

	for _, input := range inputs {
		result := maxProbability(input.n, input.edges, input.succProb, input.start, input.end)
		fmt.Println(result)
	}
}
