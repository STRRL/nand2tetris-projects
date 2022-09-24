package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	if len(os.Args) < 2 {
		println("usage: hack-assembler <file>")
		return
	}
	file := os.Args[1]
	f, err := os.Open(file)
	if err != nil {
		panic(err)
	}
	parser := NewParser(f)
	output, err := os.OpenFile(strings.Replace(file, ".asm", ".hack", -1), os.O_CREATE|os.O_WRONLY, 0644)
	output.Truncate(0)
	if err != nil {
		panic(err)
	}
	parser.PrepareSymbolTable()

	for parser.HasMoreCommands() {
		instruction, err := parser.Instruction()
		if err != nil {
			panic(err)
		}
		fmt.Fprintln(output, instruction.BinaryCode())
	}
}
