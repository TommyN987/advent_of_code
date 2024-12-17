package solutions

import (
	"sort"
	"strconv"
	"strings"
)

type Day01 struct{}

func (d *Day01) First(input string) int {
	var left []string
	var right []string

	for _, line := range strings.Split(input, "\n") {
		parts := strings.Fields(line)
		if len(parts) >= 2 {
			left = append(left, strings.TrimSpace(parts[0]))
			right = append(right, strings.TrimSpace(parts[1]))
		}
	}

	sort.Strings(left)
	sort.Strings(right)

	sum := 0
	for i := 0; i < len(left) && i < len(right); i++ {
		leftNum, err1 := strconv.Atoi(left[i])
		rightNum, err2 := strconv.Atoi(right[i])

		if err1 == nil && err2 == nil {
			sum += abs(leftNum - rightNum)
		}
	}

	return sum
}

func (d *Day01) Second(input string) int {
	leftSet := make(map[string]struct{})
	rightCounts := make(map[string]int)

	for _, line := range strings.Split(input, "\n") {
		parts := strings.Fields(line)
		if len(parts) >= 2 {
			left := strings.TrimSpace(parts[0])
			right := strings.TrimSpace(parts[1])

			leftSet[left] = struct{}{}

			rightCounts[right]++
		}
	}

	sum := 0
	for left := range leftSet {
		if count, exists := rightCounts[left]; exists {
			leftValue, err := strconv.Atoi(left)
			if err == nil {
				sum += leftValue * count
			}
		}
	}

	return sum
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}
