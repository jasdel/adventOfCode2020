package main

import (
	"bytes"
	"io"
	"os"
	"testing"
)

func BenchmarkPart2(b *testing.B) {
	cases := map[string]struct {
		fn func(int, io.Reader)
	}{
		"first": {
			fn: part2,
		},
		"second": {
			fn: part2_2,
		},
	}

	f, err := os.Open("../day_1_input")
	if err != nil {
		b.Fatalf("failed to open file, %v", err)
	}
	defer f.Close()

	var buf bytes.Buffer
	if _, err := io.Copy(&buf, f); err != nil {
		b.Fatalf("failed to get file, %v", err)
	}
	r := bytes.NewReader(buf.Bytes())

	for name, c := range cases {
		b.Run(name, func(b *testing.B) {
			for i := 0; i < b.N; i++ {
				r.Seek(0, 0)
				c.fn(2020, r)
			}
		})
	}
}
