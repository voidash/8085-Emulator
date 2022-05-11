interface Questions {
    question: string,
    code: string
}

interface QuestionList {
    questions: Array<Questions>
}

export default function getQuestionList() {
    return {
        questions: [
            {
                question: "Store the data byte 32H into memory location 4000H",
                code: `; move immediate 32h into A
MVI A, 32H
; store accumulator content to 4000h location
STA 4000H`
            },
            {
                question: "Write a program to shift an eight bit data four bits right. Assume data is in register C",
                code: `; load data into C first
MVI C, 0b01011111
; move data to accumulator for shifting
MOV A, C
; 10111110 or BE
RAR 
; 01111100 or 7c
RAR
; 11111000 or f8
RAR
; 11110000 or f0
RAR
HLT`
            },
            {
                question: "Add the contents of memory locations 4000H and 4001H and place the result in memory location 4002H. Also write program description.",
                code: `; initial preperation
MVI A, 45H
STA 4000H
MVI A, 25H
STA 4001H 

; program starts from here 
; load 4000H on HL pair
LXI H, 4000H
; move contents present in address defined by HL i.e 4000h to B
MOV B, M
; Make 4000H into 4001H
INR L
; move contents on 4000h into C
MOV C, M
; move C to A
MOV A, C
ADD B
; store contents on 4002H
STA 4002H
HLT
`
            },
            {
                question: `Find the square of the given numbers from memory location 6800H and store the result from memory location 8000H.`,
                code: `; store 5 on 6800h = 5 * 5 is 25 choose any number between 0 and 255
MVI A, 12
STA 6800H

; program starts from here
; load contents of 6800H into Accumulator
LDA 6800H
;move accumulator content to B
MOV B, A
; move B contents to C
MOV C, B
; create a label whileCnotZero
whileCnotZero:MOV A, D
ADD B 
; decrement C
DCR C
MOV D, A
; if C is 0 zero flag will be set 
MOV A, C
; jump to label whileCnotZero until Zero flag is False
JNZ whileCnotZero
MOV A, D
STA 8000H
HLT`
            },
            {
                question: `Search the given byte in the list of 50 numbers stored in the consecutive memory locations and store the address of memory location in the memory locations 2800H and 2801H. Assume byte is in the C register and starting address of the list is 2500H. If byte is not found store 00 at 2800H and 2801H.`,
                code: `LXI D, 0000H 
; assume number is stored from 0000H to 0032H

MVI C, 02H
; assume number to search is 02H

MVI B, 50
;50 memory location so B is our counter

; when B reaches 0 we go to notfound lable at bottom
find: MOV B, A
; move contents of counter to A

JZ notfound 
; if counter is 0 then go to notfound

LDAX D 
; load content on address that is hold by D register

CMP C
; compare with C

INX D
; increment D

DCR B
; decrease counter

JNZ find
; if Zero flag is not enable then number is notfound so go to top loop

MOV D, H
; move D to H

MOV E, L
; move E to L

SHLD 2800H
; store contents of H and L on 2800H and 2801H

notfound: nop`
            },
            {
                question: `Write a program to find smallest of two 8 bit numbers`,
                code: `; smaller one will be placed on accumulator
; store 45 on A
MVI A, 45
; store 55 on B
MVI B, 55

;compare A and B
CMP B

;if A > B sign flag will be set
JP smaller
;if A < B do nothing 
HLT

smaller: MOV A, B
HLT`
            },
            {
                question: `Write an assembly language program to separate even numbers from the given list of 50 numbers and store them in the another list starting from 2200H. Assume starting address of 50 number list is 2100H`,
                code: `MVI A, 3
STA 2100H
MVI A, 5
STA 2101H
MVI A, 5
STA 2102H
MVI A, 6
STA 2103H
; to separate even numbers
; find last bit is 1 or 0
; example : 0x33 & 0x1 = 1 so odd
; 0x44 & 0x1 = 0 so even
;starting address is 2100h
; stopping address will be 2100h + 50 = 2132
LXI D, 2100h
; for storing starting will be 2200h
; stopping address will be 2232h
LXI B, 2200h
; label for jump
; load from 2100 to 2150 to accumulator
inrLabel: LDAX D
; AND with 0x1
ANI 0x1
; if zero then even
JZ even
; increment E
INR E
MOV A, E
CPI 32h
JNZ inrLabel
HLT
; store that same content in memory address pointed by B
even: LDAX D
STAX B
; increment C
INR C
; increment E
INR E
JMP inrLabel`
            },
            {
                question: `Write a  program to subtract two 16 bit numbers with borrow`,
                code: `; load 16 bit number to D
LXI D, 6666h
; load 16 bit number to B
LXI B, 9999h
; Subtract D from B
; Move lower address C to accumulator
MOV A, C
; subtract with E
SUB E
; store output on L
MOV L, A
; now borrow will be present on CY Flag so we have to use SBB
; move higher address B to accumulator
MOV A, B
; subtract with borrow with D 
SBB D
; store output on H
MOV H, A
; output will be on H and L pair 
HLT `
            },
            {
                question: `Write a program to add two 16 bit numbers with carry`,
                code: `; load 16 bit number to D
LXI D, 6666H
; load 16 bit number to B
LXI B, 9999H
; add D and B
; Move lower address C to accumulator
MOV A, C
; add with E
ADD E
; store output on L
MOV L, A
; now borrow will be present on CY Flag so we have to use SBB
; move higher address B to accumulator
MOV A, B
; add with carry with D 
ADC D
; store output on H
MOV H, A
; output will be on H and L pair 
HLT`
            }
        ]
    } as QuestionList;
}
