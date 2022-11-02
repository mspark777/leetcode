package main

import (
	"fmt"
)

func minMutation(start string, end string, bank []string) int {
	bankSet := make(map[string]bool)
	seens := map[string]bool{start: true}
	queue := []string{start}

	for _, gene := range bank {
		bankSet[gene] = true
	}

	result := 0

	for len(queue) > 0 {
		queueLen := len(queue)
		for i := 0; i < queueLen; i += 1 {
			gene := queue[i]

			if gene == end {
				return result
			}

			for _, g := range "ACGT" {
				for j := 0; j < len(gene); j += 1 {
					genes := []rune(gene)
					genes[j] = g

					neighbor := string(genes)

					if _, ok := seens[neighbor]; ok {
						continue
					}

					if _, ok := bankSet[neighbor]; ok {
						queue = append(queue, neighbor)
						seens[neighbor] = true
					}
				}
			}
		}

		result += 1
		queue = queue[queueLen:]
	}

	return -1
}

type input struct {
	start string
	end   string
	bank  []string
}

func main() {
	inputs := []input{
		{start: "AACCGGTT", end: "AACCGGTA", bank: []string{"AACCGGTA"}},
		{start: "AACCGGTT", end: "AAACGGTA", bank: []string{"AACCGGTA", "AACCGCTA", "AAACGGTA"}},
		{start: "AAAAACCC", end: "AACCCCCC", bank: []string{"AAAACCCC", "AAACCCCC", "AACCCCCC"}},
		{start: "AACCGGTT", end: "AACCGCTA", bank: []string{"AACCGGTA", "AACCGCTA", "AAACGGTA"}},
	}

	for _, input := range inputs {
		start := input.start
		end := input.end
		bank := input.bank
		result := minMutation(start, end, bank)
		fmt.Println(result)
	}
}
