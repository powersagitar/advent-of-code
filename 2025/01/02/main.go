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
			for i := 0; i < elem.magnitude; i++ {
				if dial == 99 {
					dial = 0
					password++
					continue
				}

				dial++
			}
		} else {
			for i := 0; i < elem.magnitude; i++ {
				if dial == 1 {
					dial = 0
					password++
					continue
				} else if dial == 0 {
					dial = 99
					continue
				}

				dial--
			}
		}
	}

	fmt.Println("Password", password)
}
