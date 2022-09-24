package main

import "testing"

func Test_splitCInstruction(t *testing.T) {
	type args struct {
		line string
	}
	tests := []struct {
		name         string
		args         args
		wantDestPart string
		wantCompPart string
		wantJumpPart string
	}{
		{
			name:         "test1",
			args:         args{line: "D=D+A"},
			wantDestPart: "D",
			wantCompPart: "D+A",
			wantJumpPart: "",
		},
		{
			name:         "test2",
			args:         args{line: "D=D+A;JMP"},
			wantDestPart: "D",
			wantCompPart: "D+A",
			wantJumpPart: "JMP",
		},
		{
			name:         "test3",
			args:         args{line: "D+A;JMP"},
			wantDestPart: "",
			wantCompPart: "D+A",
			wantJumpPart: "JMP",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gotDestPart, gotCompPart, gotJumpPart := splitCInstruction(tt.args.line)
			if gotDestPart != tt.wantDestPart {
				t.Errorf("splitCInstruction() gotDestPart = %v, want %v", gotDestPart, tt.wantDestPart)
			}
			if gotCompPart != tt.wantCompPart {
				t.Errorf("splitCInstruction() gotCompPart = %v, want %v", gotCompPart, tt.wantCompPart)
			}
			if gotJumpPart != tt.wantJumpPart {
				t.Errorf("splitCInstruction() gotJumpPart = %v, want %v", gotJumpPart, tt.wantJumpPart)
			}
		})
	}
}
