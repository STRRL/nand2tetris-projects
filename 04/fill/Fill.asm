// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

(LOOP)

@KBD
D=M
@SET_WHITE
D;JEQ
@SET_BLACK
0;JMP

(SET_BLACK)
// color=-1
@color
M=-1
@PAINT
0;JMP

(SET_WHITE)
// color=0
@color
M=0
@PAINT
0;JMP

(PAINT)
// i=0
@i
M=0

@SCREEN
D=A
@seek
M=D

(PAINTLOOP)
// if i>=8192; exit paint loop
@8192
D=A
@i
D=M-D // D=i-8192
@LOOP
D;JGE

// else paint pixel

@color
D=M
@seek
A=M
M=D

// i=i+1
@i
M=M+1
@seek
M=M+1

// paint loop
@PAINTLOOP
0;JMP
