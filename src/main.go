package main

import (
	"fmt"
)

func getMax(i, j int) int {
	if i > j {
		return i
	}
	return j
}

func minRefuelStops(target int, startFuel int, stations [][]int) int {
	stationCount := len(stations)
	dp := make([]int, stationCount+1)
	dp[0] = startFuel

	for i := 0; i < stationCount; i += 1 {
		station := stations[i]
		position := station[0]
		fuel := station[1]
		for t := i; t >= 0; t -= 1 {
			memo := dp[t]
			if memo >= position {
				next := t + 1
				dp[next] = getMax(dp[next], dp[t]+fuel)
			}
		}
	}

	for i, memo := range dp {
		if memo >= target {
			return i
		}
	}

	return -1
}

type input struct {
	target    int
	startFuel int
	stations  [][]int
}

func main() {
	inputs := []*input{
		{
			target: 1, startFuel: 1, stations: [][]int{},
		},
		{
			target: 100, startFuel: 1, stations: [][]int{{10, 100}},
		},
		{
			target: 100, startFuel: 10, stations: [][]int{{10, 60}, {20, 30}, {30, 30}, {60, 40}},
		},
		{
			target: 100, startFuel: 50, stations: [][]int{{50, 50}},
		},
	}

	for _, input := range inputs {
		target := input.target
		startFuel := input.startFuel
		stations := input.stations
		result := minRefuelStops(target, startFuel, stations)
		fmt.Println(result)
	}
}
