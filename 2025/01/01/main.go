package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type Combination struct {
	direction byte
	magnitude int
}

func main() {
	var combinations []Combination

	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			break
		}

		direction := line[0]
		magnitude, err := strconv.Atoi(line[1:])
		if err != nil {
			return
		}

		combinations = append(combinations, Combination{
			direction,
			magnitude,
		})
	}

	dial := 50
	password := 0

	for _, elem := range combinations {
		if elem.direction == 'R' {
			dial = (dial + elem.magnitude) % 100
		} else {
			dial = ((dial - elem.magnitude) + 100) % 100
		}

		if dial == 0 {
			password++
		}
	}

	fmt.Println("Password", password)
}
