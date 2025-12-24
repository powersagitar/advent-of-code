package main

import (
	"bufio"
	"fmt"
	"os"
)

type block int

const (
	emptySpace block = iota
	splitter
	start
)

type manifold [][]block

func main() {
	manifold := getInput()
	startCol := findStart(&manifold)

	beam := make([]bool, len(manifold[0]))
	beam[startCol] = true

	splits := 0

	for _, row := range manifold[2:] {
		for col, block := range row {
			if block != splitter {
				continue
			}

			if !beam[col] {
				continue
			}

			beam[col] = false

			if col > 0 {
				beam[col-1] = true
			}

			if col < len(manifold[0])-1 {
				beam[col+1] = true
			}

			splits++
		}
	}

	fmt.Println("splits:", splits)
}

func findStart(manifold *manifold) int {
	for col := range (*manifold)[0] {
		if (*manifold)[0][col] == start {
			return col
		}
	}

	panic("unreachable")
}

func getInput() (manifold manifold) {
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		text := scanner.Text()

		if text == "" {
			break
		}

		var line []block

		for _, rune := range text {
			switch rune {
			case 'S':
				line = append(line, start)

			case '.':
				line = append(line, emptySpace)

			case '^':
				line = append(line, splitter)
			}
		}

		manifold = append(manifold, line)
	}

	return
}
