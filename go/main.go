package main

import (
	"fmt"
	"io/fs"
	"log"
	"os"
	"path/filepath"
)

func main() {
	sourceDir := "../inputs/"

	inputs, err := readInputs(sourceDir)
	if err != nil {
		log.Fatalf("Failed to read inputs: %v", err)
	}

	registry := NewRegistry()

	results := registry.Solve(inputs)

	for i, result := range results {
		fmt.Printf("Day %d ===> First: %d; Second: %d\n", i+1, result.First, result.Second)
	}
}

func readInputs(sourceDir string) ([]string, error) {
	var inputs []string

	err := filepath.Walk(sourceDir, func(path string, info fs.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if !info.IsDir() {
			content, err := os.ReadFile(path)
			if err != nil {
				return err
			}
			inputs = append(inputs, string(content))
		}
		return nil
	})

	return inputs, err
}
