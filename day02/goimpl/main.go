package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"strings"
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

	n, err = part2(f)
	log.Println("Part2:", n, err)
}

func part1(r io.Reader) (int64, error) {
	var valid int64

	scanner := bufio.NewScanner(r)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if len(line) == 0 {
			continue
		}

		var min, max int
		var char rune
		var pass string
		if np, err := fmt.Sscanf(line, "%d-%d %c: %s", &min, &max, &char, &pass); err != nil || np != 4 {
			return 0, fmt.Errorf("failed to parse line, [%s], %w", line, err)
		}

		var cs int
		for _, v := range pass {
			if v == char {
				cs++
			}
		}

		if cs >= min && cs <= max {
			valid++
		}

	}
	if err := scanner.Err(); err != nil {
		return 0, fmt.Errorf("failed to scan lines, %w", err)
	}

	return valid, nil
}

func part2(r io.Reader) (int64, error) {
	var valid int64

	scanner := bufio.NewScanner(r)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if len(line) == 0 {
			continue
		}

		var ai, bi int
		var char rune
		var pass string
		if np, err := fmt.Sscanf(line, "%d-%d %c: %s", &ai, &bi, &char, &pass); err != nil || np != 4 {
			return 0, fmt.Errorf("failed to parse line, [%s], %w", line, err)
		}

		var haveA, haveB bool
		for i, v := range pass {
			if i+1 == ai && v == char {
				haveA = true
			}
			if i+1 == bi && v == char {
				haveB = true
			}
		}

		if !(haveA == haveB) && (haveA || haveB) {
			valid++
		}

	}
	if err := scanner.Err(); err != nil {
		return 0, fmt.Errorf("failed to scan lines, %w", err)
	}

	return valid, nil
}
