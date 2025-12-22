package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

type Window struct {
	start int
	end   int
}

func main() {
	windows, _ := getInput()

	for i := range len(windows) {
		start := unoverlapStart(slices.Concat(windows[:i], windows[i+1:]), windows[i].start)
		end := unoverlapEnd(slices.Concat(windows[:i], windows[i+1:]), windows[i].end)

		windows[i].start = start
		windows[i].end = end
	}

	freshCount := 0

	for _, window := range windows {
		delta := window.end - window.start + 1

		if delta < 1 {
			continue
		}

		freshCount += delta
	}

	fmt.Println("fresh count:", freshCount)
}

func unoverlapStart(windows []Window, start int) int {
	for _, window := range windows {
		if start >= window.start && start <= window.end {
			return window.end + 1
		}
	}

	return start
}

func unoverlapEnd(windows []Window, end int) int {
	for _, window := range windows {
		if end >= window.start && end <= window.end {
			return window.start - 1
		}
	}

	return end
}

func getInput() ([]Window, []int) {
	var windows []Window
	var ids []int

	scanner := bufio.NewScanner(os.Stdin)

	expectsWindows := true

	for scanner.Scan() {
		text := scanner.Text()

		if expectsWindows {
			if text == "" {
				expectsWindows = false
				continue
			}

			window := strings.Split(text, "-")

			start, err := strconv.Atoi(window[0])
			if err != nil {
				return make([]Window, 0), make([]int, 0)
			}

			end, err := strconv.Atoi(window[1])
			if err != nil {
				return make([]Window, 0), make([]int, 0)
			}

			windows = append(windows, Window{start, end})

		} else {
			if text == "" {
				break
			}

			id, err := strconv.Atoi(text)
			if err != nil {
				return make([]Window, 0), make([]int, 0)
			}

			ids = append(ids, id)
		}
	}

	return windows, ids
}
