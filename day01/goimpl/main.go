package main

import (
	"bufio"
	"flag"
	"io"
	"log"
	"os"
	"strconv"
)

func main() {
	var target int
	flag.IntVar(&target, "t", 2020, "The target number to find")

	f, err := os.Open("../day_1_input")
	if err != nil {
		log.Fatalf("failed to open file, %v", err)
	}

	part1(target, f)

	f.Seek(0, 0)

	part2(target, f)
}

func part1(target int, r io.Reader) {
	scanner := bufio.NewScanner(r)

	items := map[int]struct{}{}
	for scanner.Scan() {
		v := scanner.Text()
		if len(v) == 0 {
			continue
		}
		i, err := strconv.ParseInt(v, 10, 64)
		if err != nil {
			log.Fatalf("failed to parse line, %v", err)
		}
		a := int(i)

		b := target - a
		if _, ok := items[b]; ok {
			log.Println(b * a)
			break
		}
		items[a] = struct{}{}
	}

	if err := scanner.Err(); err != nil {
		log.Fatalf("failed to scan input, %v", err)
	}
}

func part2(target int, r io.Reader) {
	scanner := bufio.NewScanner(r)

	is := map[int]struct{}{}
	bcs := map[int]int{}
	for scanner.Scan() {
		v := scanner.Text()
		if len(v) == 0 {
			continue
		}
		i, err := strconv.ParseInt(v, 10, 64)
		if err != nil {
			log.Fatalf("failed to parse line, %v", err)
		}
		a := int(i)

		bc := target - a
		bcs[bc] = a
		is[a] = struct{}{}
	}

	if err := scanner.Err(); err != nil {
		log.Fatalf("failed to scan input, %v", err)
	}

	for bc, a := range bcs {
		for b := range is {
			if b == a {
				continue
			}
			if c := bc - b; c > 0 && a+b+c == target {
				if _, ok := is[c]; !ok {
					continue
				}
				log.Println(a*b*c, a, b, c)
				return
			}
		}
	}
}

func part2_2(target int, r io.Reader) {
	scanner := bufio.NewScanner(r)

	is := map[int]struct{}{}
	for scanner.Scan() {
		v := scanner.Text()
		if len(v) == 0 {
			continue
		}
		i, err := strconv.ParseInt(v, 10, 64)
		if err != nil {
			log.Fatalf("failed to parse line, %v", err)
		}
		a := int(i)

		is[a] = struct{}{}
	}

	if err := scanner.Err(); err != nil {
		log.Fatalf("failed to scan input, %v", err)
	}

	for a := range is {
		for b := range is {
			c := target - a - b
			if _, ok := is[c]; ok {
				//log.Println(a*b*c, a, b, c)
				return
			}
		}
	}
}
