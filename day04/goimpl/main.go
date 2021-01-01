package main

import (
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

	n, err = part2(f)
	log.Println("Part2:", n, err)
}

func part1(r io.Reader) (int, error) {
	return 0, fmt.Errorf("not implemented")
}

func part2(r io.Reader) (int, error) {
	return 0, fmt.Errorf("not implemented")
}
