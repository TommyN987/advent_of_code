package solutions

import (
	"strconv"
	"strings"
)

type Day02 struct{}

func isSafe(report []int) bool {
	isIncreeasing := true
	isDecreasing := true

	for i := 0; i < len(report)-1; i++ {
		diff := report[i+1] - report[i]
		absDiff := abs(diff)

		if absDiff < 1 || absDiff > 3 {
			return false
		}
		if diff > 0 {
			isDecreasing = false
		}
		if diff < 0 {
			isIncreeasing = false
		}
	}

	return isIncreeasing || isDecreasing
}

func canBeMadeSafe(report []int) bool {
	if isSafe(report) {
		return true
	}

	for i := 0; i < len(report); i++ {
		modified := append([]int{}, report[:i]...)
		modified = append(modified, report[i+1:]...)

		if isSafe(modified) {
			return true
		}
	}

	return false
}

func parseLine(line string) []int {
	parts := strings.Fields(line)
	report := make([]int, len(parts))

	for i, part := range parts {
		num, _ := strconv.Atoi(part)
		report[i] = num
	}
	return report
}

func (d *Day02) First(input string) int {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	safeCount := 0

	for _, line := range lines {
		report := parseLine(line)
		if isSafe(report) {
			safeCount++
		}
	}

	return safeCount
}

func (d *Day02) Second(input string) int {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	safeCount := 0

	for _, line := range lines {
		report := parseLine(line)
		if canBeMadeSafe(report) {
			safeCount++
		}
	}

	return safeCount
}
