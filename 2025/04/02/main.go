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

	rollsRemoved := 0

	removePaperRolls(paperRolls, &rollsRemoved)

	fmt.Println("rolls removed:", rollsRemoved)
}

type Coord struct {
	row int
	col int
}

func removePaperRolls(paperRolls [][]bool, totalRollsRemoved *int) {
	var toRemove []Coord

	for row := range len(paperRolls) {
		for col := range len(paperRolls[row]) {
			if !paperRolls[row][col] {
				continue
			}

			if countAdjacentRolls(paperRolls, row, col) < 4 {
				toRemove = append(toRemove, Coord{row, col})
			}
		}
	}

	if len(toRemove) < 1 {
		return
	}

	*totalRollsRemoved += len(toRemove)

	for _, coord := range toRemove {
		paperRolls[coord.row][coord.col] = false
	}

	removePaperRolls(paperRolls, totalRollsRemoved)
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
