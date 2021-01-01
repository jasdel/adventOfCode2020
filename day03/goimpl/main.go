package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
)

func main() {
	f, err := os.Open("../input")
	if err != nil {
		log.Fatalf("failed to open file, %v", err)
	}
	defer f.Close()

	n, err := part1(f)
	log.Println("Part1:", n, err)
	f.Seek(0, 0)

	ns, err := part2(f, []Check{
		{X: 1, Y: 1},
		{X: 3, Y: 1}, // same as part 1
		{X: 5, Y: 1},
		{X: 7, Y: 1},
		{X: 1, Y: 2},
	})
	p := ns[0]
	for _, n := range ns[1:] {
		p *= n
	}
	log.Println("Part2:", ns, p, err)
}

func part1(r io.Reader) (int, error) {
	var trees int

	scanner := bufio.NewScanner(r)

	scanner.Scan() // skip first line
	var offset int
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			continue
		}

		offset += 3

		o := offset % len(line)
		if line[o] == '#' {
			trees++
		}
	}

	if err := scanner.Err(); err != nil {
		return 0, fmt.Errorf("failed to scan lines, %w", err)
	}

	return trees, nil
}

type Check struct {
	X, Y int

	XOffset, YOffset int
}

func part2(r io.Reader, checks []Check) ([]int, error) {
	ns := make([]int, len(checks))

	scanner := bufio.NewScanner(r)
	scanner.Scan() // skip first line
	for scanner.Scan() {
		line := scanner.Text()

		for i := 0; i < len(checks); i++ {
			checks[i].YOffset++

			if (checks[i].YOffset % checks[i].Y) == 0 {
				checks[i].XOffset += checks[i].X

				o := checks[i].XOffset % len(line)
				if line[o] == '#' {
					ns[i]++
				}
			}
		}
	}

	if err := scanner.Err(); err != nil {
		return nil, fmt.Errorf("failed to scan lines, %w", err)
	}

	return ns, nil
}
