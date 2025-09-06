package main

import (
	"KdnLang/src/lexer"
	"os"
)

func main() {
	bytes, _ := os.ReadFile("test.lang")
	tokens := lexer.Tokenize(string(bytes))

	for _, token := range tokens {
		token.Debug()
	}
}
