package main

import (
	"bufio"
	"errors"
	"os"
	"strconv"
	"strings"
)

type Parser struct {
	file        *os.File
	scanner     *bufio.Scanner
	symbolTable *SymbolTable
	lastLine    string
}

func NewParser(file *os.File) *Parser {
	return &Parser{
		file:        file,
		scanner:     bufio.NewScanner(file),
		symbolTable: NewSymbolTable(),
	}
}

func (p *Parser) PrepareSymbolTable() {
	defer func() {
		p.file.Seek(0, 0)
		p.scanner = bufio.NewScanner(p.file)
	}()

	for pc := 0; p.scanner.Scan(); {
		trimmedLine := strings.TrimSpace(p.scanner.Text())
		if strings.HasPrefix(trimmedLine, "(") {
			// this line is label
			label := strings.Trim(trimmedLine, "()")
			p.symbolTable.AddEntry(label, pc)
		}
		if strings.HasPrefix(trimmedLine, "@") {
			// this line is A instruction
			symbol := strings.Trim(trimmedLine, "@")
			if _, err := strconv.Atoi(symbol); err != nil {
				// a symbol
				if !p.symbolTable.Contains(symbol) {
					p.symbolTable.AddEntry(symbol, p.symbolTable.NextAvailableAddress())
				}
			}
		}
		if isValidInstruction(trimmedLine) {
			pc++
		}
	}
}

func (p *Parser) HasMoreCommands() bool {
	for p.scanner.Scan() {
		line := trim(p.scanner.Text())
		if isValidInstruction(line) {
			p.lastLine = line
			return true
		}
	}
	p.lastLine = ""
	return false
}
func trim(line string) string {
	trimSpace := strings.TrimSpace(line)
	if !strings.HasPrefix(trimSpace, "//") && strings.Contains(trimSpace, "//") {
		return strings.TrimSpace(strings.Split(trimSpace, "//")[0])
	}
	return trimSpace
}

var ErrNoInstruction = errors.New("no instruction")

func (p *Parser) Instruction() (*Instruction, error) {
	if p.lastLine == "" {
		return nil, ErrNoInstruction
	}
	return p.ParseInstruction(p.lastLine)
}

func isValidInstruction(trimmedLine string) bool {
	if len(trimmedLine) == 0 {
		// empty line
		return false
	}
	if strings.HasPrefix(trimmedLine, "//") {
		// this line is commend
		return false
	}
	if strings.HasPrefix(trimmedLine, "(") {
		// this line is label
		return false
	}

	return true
}
