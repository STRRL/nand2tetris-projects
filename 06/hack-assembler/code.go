package main

import (
	"fmt"
	"strconv"
	"strings"
)

type InstructionType string

// AInstruction example: @value
const AInstruction InstructionType = "A"

// CInstruction example: dest=comp;jump
const CInstruction InstructionType = "C"

type DestType struct {
	DestA bool
	DestD bool
	DestM bool
}

func (d DestType) BinaryCode() string {
	result := ""
	if d.DestA {
		result += "1"
	} else {
		result += "0"
	}
	if d.DestD {
		result += "1"
	} else {
		result += "0"
	}
	if d.DestM {
		result += "1"
	} else {
		result += "0"
	}
	return result
}

const CompZero CompType = "0"
const CompOne CompType = "1"
const CompNegOne CompType = "-1"
const CompD CompType = "D"
const CompA CompType = "A"
const CompM CompType = "M"
const CompNotD CompType = "!D"
const CompNotA CompType = "!A"
const CompNotM CompType = "!M"
const CompNegD CompType = "-D"
const CompNegA CompType = "-A"
const CompNegM CompType = "-M"
const CompDPlusOne CompType = "D+1"
const CompAPlusOne CompType = "A+1"
const CompMPlusOne CompType = "M+1"
const CompDMinusOne CompType = "D-1"
const CompAMinusOne CompType = "A-1"
const CompMMinusOne CompType = "M-1"
const CompDPlusA CompType = "D+A"
const CompDPlusM CompType = "D+M"
const CompDMinusA CompType = "D-A"
const CompDMinusM CompType = "D-M"
const CompAMinusD CompType = "A-D"
const CompMMinusD CompType = "M-D"
const CompDAndA CompType = "D&A"
const CompDAndM CompType = "D&M"
const CompDOrA CompType = "D|A"
const CompDOrM CompType = "D|M"

var AllComps = []CompType{
	CompZero,
	CompOne,
	CompNegOne,
	CompD,
	CompA,
	CompM,
	CompNotD,
	CompNotA,
	CompNotM,
	CompNegD,
	CompNegA,
	CompNegM,
	CompDPlusOne,
	CompAPlusOne,
	CompMPlusOne,
	CompDMinusOne,
	CompAMinusOne,
	CompMMinusOne,
	CompDPlusA,
	CompDPlusM,
	CompDMinusA,
	CompDMinusM,
	CompAMinusD,
	CompMMinusD,
	CompDAndA,
	CompDAndM,
	CompDOrA,
	CompDOrM,
}

type CompType string

func (c CompType) BinaryCode() string {
	switch c {
	case CompZero:
		return "0101010"
	case CompOne:
		return "0111111"
	case CompNegOne:
		return "0111010"
	case CompD:
		return "0001100"
	case CompA:
		return "0110000"
	case CompM:
		return "1110000"
	case CompNotD:
		return "0001101"
	case CompNotA:
		return "0110001"
	case CompNotM:
		return "1110001"
	case CompNegD:
		return "0001111"
	case CompNegA:
		return "0110011"
	case CompNegM:
		return "1110011"
	case CompDPlusOne:
		return "0011111"
	case CompAPlusOne:
		return "0110111"
	case CompMPlusOne:
		return "1110111"
	case CompDMinusOne:
		return "0001110"
	case CompAMinusOne:
		return "0110010"
	case CompMMinusOne:
		return "1110010"
	case CompDPlusA:
		return "0000010"
	case CompDPlusM:
		return "1000010"
	case CompDMinusA:
		return "0010011"
	case CompDMinusM:
		return "1010011"
	case CompAMinusD:
		return "0000111"
	case CompMMinusD:
		return "1000111"
	case CompDAndA:
		return "0000000"
	case CompDAndM:
		return "1000000"
	case CompDOrA:
		return "0010101"
	case CompDOrM:
		return "1010101"
	}
	return ""
}

type JumpType struct {
	JumpWheGreater bool
	JumpWheEqual   bool
	JumpWheLess    bool
}

func (j JumpType) BinaryCode() string {
	result := ""
	if j.JumpWheLess {
		result += "1"
	} else {
		result += "0"
	}
	if j.JumpWheEqual {
		result += "1"
	} else {
		result += "0"
	}
	if j.JumpWheGreater {
		result += "1"
	} else {
		result += "0"
	}
	return result
}

type Instruction struct {
	InstructionType InstructionType
	// Only available for AInstruction
	Value int
	// Only available for CInstruction
	Dest DestType
	// Only available for CInstruction
	Comp CompType
	// Only available for CInstruction
	Jump JumpType
}

func (p *Parser) ParseInstruction(line string) (*Instruction, error) {
	if strings.HasPrefix(line, "@") {
		return p.parseAInstruction(line)
	}

	return parseCInstruction(line)
}
func (p *Parser) parseAInstruction(line string) (*Instruction, error) {
	symbol := line[1:]
	// instant value
	value, err := strconv.Atoi(symbol)
	if err != nil {
		// or a symbol
		if p.symbolTable.Contains(symbol) {
			value = p.symbolTable.GetAddress(symbol)
		} else {
			panic("unknown symbol " + symbol)
		}
	}
	return &Instruction{
		InstructionType: AInstruction,
		Value:           value,
	}, nil
}

func parseCInstruction(line string) (*Instruction, error) {
	destPart, compPart, jumpPart := splitCInstruction(line)
	var dest DestType
	if strings.Contains(destPart, "A") {
		dest.DestA = true
	}
	if strings.Contains(destPart, "D") {
		dest.DestD = true
	}
	if strings.Contains(destPart, "M") {
		dest.DestM = true
	}
	comp := CompType(compPart)
	var jump JumpType
	if strings.Contains(jumpPart, "JGT") {
		jump.JumpWheGreater = true
	}
	if strings.Contains(jumpPart, "JEQ") {
		jump.JumpWheEqual = true
	}
	if strings.Contains(jumpPart, "JLT") {
		jump.JumpWheLess = true
	}
	if strings.Contains(jumpPart, "JGE") {
		jump.JumpWheGreater = true
		jump.JumpWheEqual = true
	}
	if strings.Contains(jumpPart, "JNE") {
		jump.JumpWheEqual = true
		jump.JumpWheLess = true
	}
	if strings.Contains(jumpPart, "JLE") {
		jump.JumpWheGreater = true
		jump.JumpWheLess = true
	}
	if strings.Contains(jumpPart, "JMP") {
		jump.JumpWheGreater = true
		jump.JumpWheEqual = true
		jump.JumpWheLess = true
	}

	return &Instruction{
		InstructionType: CInstruction,
		Dest:            dest,
		Comp:            comp,
		Jump:            jump,
	}, nil
}

func splitCInstruction(line string) (destPart, compPart, jumpPart string) {
	if strings.ContainsRune(line, '=') {
		destPart = line[:strings.IndexRune(line, '=')]
		compPart = line[strings.IndexRune(line, '=')+1:]
	}
	if strings.ContainsRune(line, ';') {
		jumpPart = line[strings.IndexRune(line, ';')+1:]
		compPart = line[strings.IndexRune(line, '=')+1 : strings.IndexRune(line, ';')]
	}
	return
}

func (i Instruction) BinaryCode() string {
	if i.InstructionType == AInstruction {
		return "0" + fmt.Sprintf("%015b", i.Value)
	}
	return "111" + i.Comp.BinaryCode() + i.Dest.BinaryCode() + i.Jump.BinaryCode()
}
