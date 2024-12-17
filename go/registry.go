package main

import (
	"github.com/TommyN987/advent_of_code/go/solutions"
)

type Solvable interface {
	First(input string) int
	Second(input string) int
}

type Registry struct {
	Solvers []Solvable
}

func NewRegistry() *Registry {
	return &Registry{
		Solvers: []Solvable{
			&solutions.Day01{},
		},
	}
}

func (r *Registry) Solve(inputs []string) []struct {
	First  int
	Second int
} {
	results := make([]struct {
		First  int
		Second int
	}, len(r.Solvers))

	for i, solver := range r.Solvers {
		results[i] = struct {
			First  int
			Second int
		}{
			First:  solver.First(inputs[i]),
			Second: solver.Second(inputs[i]),
		}
	}

	return results
}
