package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var paperRolls [][]bool

	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		text := scanner.Text()

		if text == "" {
			break
		}

		var row []bool

		for _, rune := range text {
			if rune == '@' {
				row = append(row, true)
			} else {
				row = append(row, false)
			}
		}

		paperRolls = append(paperRolls, row)
	}

	liftables := 0

	for row := 0; row < len(paperRolls); row++ {
		for col := 0; col < len(paperRolls[row]); col++ {
			if !paperRolls[row][col] {
				continue
			}

			if countAdjacentRolls(paperRolls, row, col) < 4 {
				liftables++
			}
		}
	}

	fmt.Println("liftables:", liftables)
}

func countAdjacentRolls(paperRolls [][]bool, row int, col int) int {
	count := 0

	if row > 0 && col > 0 && paperRolls[row-1][col-1] {
		count++
	}

	if row > 0 && paperRolls[row-1][col] {
		count++
	}

	if row > 0 && col < len(paperRolls[row])-1 && paperRolls[row-1][col+1] {
		count++
	}

	if col > 0 && paperRolls[row][col-1] {
		count++
	}

	if col < len(paperRolls[row])-1 && paperRolls[row][col+1] {
		count++
	}

	if row < len(paperRolls)-1 && col > 0 && paperRolls[row+1][col-1] {
		count++
	}

	if row < len(paperRolls)-1 && paperRolls[row+1][col] {
		count++
	}

	if row < len(paperRolls)-1 && col < len(paperRolls[row])-1 && paperRolls[row+1][col+1] {
		count++
	}

	return count
}
