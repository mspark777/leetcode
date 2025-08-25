package main

import (
	"fmt"
)

func solve(releaseTimes []int, keysPressed []rune) rune {
	n := len(releaseTimes)
	longestPress := releaseTimes[0]
	slowestKey := keysPressed[0]

	for i := 1; i < n; i += 1 {
		currentDuration := releaseTimes[i] - releaseTimes[i-1]
		if currentDuration > longestPress ||
			(currentDuration == longestPress && keysPressed[i] > slowestKey) {
			longestPress = currentDuration
			slowestKey = keysPressed[i]
		}
	}

	return slowestKey
}

func slowestKey(releaseTimes []int, keysPressed string) byte {
	keysPressedRune := []rune(keysPressed)
	return byte(solve(releaseTimes, keysPressedRune))
}

type input struct {
	releaseTimes []int
	keysPressed  string
}

func main() {
	inputs := []input{
		{
			releaseTimes: []int{9, 29, 49, 50},
			keysPressed:  "cbcd",
		},
		{
			releaseTimes: []int{12, 23, 36, 46, 62},
			keysPressed:  "spuda",
		},
		{
			releaseTimes: []int{9, 29, 49, 50},
			keysPressed:  "cbcd",
		},
	}

	for _, input := range inputs {
		result := slowestKey(input.releaseTimes, input.keysPressed)
		fmt.Println(result)
	}
}
