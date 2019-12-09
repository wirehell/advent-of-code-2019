
.count  equ 100 
.comp   equ 101
.size   equ 16

.loop   inc rb
        out [rb-1]
        add [count] 1 [count]       
        eq [tmp] size [comp]
        jif [comp] loop
        halt

.table  dw  1234,1235,4,127,88
.msg    dw  "hello world", 0

        jit 
        add
        mul
        lt


statements = list of statements

statement = [.(<label>)] memonic 

mnemonic = constant_expr | data_expr | instruction_expr

constant = equ CONST
data_expr = dw list of data, csv
data = <int> | "string"

value = CONST|INT

REF = IMM (value) | Rel(value) | Pos(value)

instruction_expression = 
  inc <register> |
  out <REF> |
  add <REF> <REF> <REF>




Executing: ProcessorState { ip: 0, relative_base: 0 } AdjustRelativeBase { op: Imm(1) }
Executing: ProcessorState { ip: 2, relative_base: 1 } Output { src: Rel(-1) }
Executing: ProcessorState { ip: 4, relative_base: 1 } Add { op1: Pos(100), op2: Imm(1), dst: Pos(100) }
Executing: ProcessorState { ip: 8, relative_base: 1 } Equals { op1: Pos(100), op2: Imm(16), dst: Pos(101) }
Executing: ProcessorState { ip: 12, relative_base: 1 } JumpIfFalse { cond: Pos(101), target: Imm(0) }
Executing: ProcessorState { ip: 0, relative_base: 1 } AdjustRelativeBase { op: Imm(1) }
Executing: ProcessorState { ip: 2, relative_base: 2 } Output { src: Rel(-1) }
Executing: ProcessorState { ip: 4, relative_base: 2 } Add { op1: Pos(100), op2: Imm(1), dst: Pos(100) }
Executing: ProcessorState { ip: 8, relative_base: 2 } Equals { op1: Pos(100), op2: Imm(16), dst: Pos(101) }
Executing: ProcessorState { ip: 12, relative_base: 2 } JumpIfFalse { cond: Pos(101), target: Imm(0) }
Executing: ProcessorState { ip: 0, relative_base: 2 } AdjustRelativeBase { op: Imm(1) }
Executing: ProcessorState { ip: 2, relative_base: 3 } Output { src: Rel(-1) }
Executing: ProcessorState { ip: 4, relative_base: 3 } Add { op1: Pos(100), op2: Imm(1), dst: Pos(100) }
Executing: ProcessorState { ip: 8, relative_base: 3 } Equals { op1: Pos(100), op2: Imm(16), dst: Pos(101) }
Executing: ProcessorState { ip: 12, relative_base: 3 } JumpIfFalse { cond: Pos(101), target: Imm(0) }
Executing: ProcessorState { ip: 0, relative_base: 3 } AdjustRelativeBase { op: Imm(1) }
Executing: ProcessorState { ip: 2, relative_base: 4 } Output { src: Rel(-1) }
Executing: ProcessorState { ip: 4, relative_base: 4 } Add { op1: Pos(100), op2: Imm(1), dst: Pos(100) }
Executing: ProcessorState { ip: 8, relative_base: 4 } Equals { op1: Pos(100), op2: Imm(16), dst: Pos(101) }
Executing: ProcessorState { ip: 12, relative_base: 4 } JumpIfFalse { cond: Pos(101), target: Imm(0) }
Executing: ProcessorState { ip: 0, relative_base: 4 } AdjustRelativeBase { op: Imm(1) }
Executing: ProcessorState { ip: 2, relative_base: 5 } Output { src: Rel(-1) }
Executing: ProcessorState { ip: 4, relative_base: 5 } Add { op1: Pos(100), op2: Imm(1), dst: Pos(100) }
Executing: ProcessorState { ip: 8, relative_base: 5 } Equals { op1: Pos(100), op2: Imm(16), dst: Pos(101) }
Executing: ProcessorState { ip: 12, relative_base: 5 } JumpIfFalse { cond: Pos(101), target: Imm(0) }
Executing: ProcessorState { ip: 0, relative_base: 5 } AdjustRelativeBase { op: Imm(1) }
Executing: ProcessorState { ip: 2, relative_base: 6 } Output { src: Rel(-1) }
Executing: ProcessorState { ip: 4, relative_base: 6 } Add { op1: Pos(100), op2: Imm(1), dst: Pos(100) }
Executing: ProcessorState { ip: 8, relative_base: 6 } Equals { op1: Pos(100), op2: Imm(16), dst: Pos(101) }
Executing: ProcessorState { ip: 12, relative_base: 6 } JumpIfFalse { cond: Pos(101), target: Imm(0) }
Executing: ProcessorState { ip: 0, relative_base: 6 } AdjustRelativeBase { op: Imm(1) }
Executing: ProcessorState { ip: 2, relative_base: 7 } Output { src: Rel(-1) }
Executing: ProcessorState { ip: 4, relative_base: 7 } Add { op1: Pos(100), op2: Imm(1), dst: Pos(100) }
Executing: ProcessorState { ip: 8, relative_base: 7 } Equals { op1: Pos(100), op2: Imm(16), dst: Pos(101) }
Executing: ProcessorState { ip: 12, relative_base: 7 } JumpIfFalse { cond: Pos(101), target: Imm(0) }
Executing: ProcessorState { ip: 0, relative_base: 7 } AdjustRelativeBase { op: Imm(1) }
Executing: ProcessorState { ip: 2, relative_base: 8 } Output { src: Rel(-1) }
Executing: ProcessorState { ip: 4, relative_base: 8 } Add { op1: Pos(100), op2: Imm(1), dst: Pos(100) }
Executing: ProcessorState { ip: 8, relative_base: 8 } Equals { op1: Pos(100), op2: Imm(16), dst: Pos(101) }
Executing: ProcessorState { ip: 12, relative_base: 8 } JumpIfFalse { cond: Pos(101), target: Imm(0) }
Executing: ProcessorState { ip: 0, relative_base: 8 } AdjustRelativeBase { op: Imm(1) }
Executing: ProcessorState { ip: 2, relative_base: 9 } Output { src: Rel(-1) }
Executing: ProcessorState { ip: 4, relative_base: 9 } Add { op1: Pos(100), op2: Imm(1), dst: Pos(100) }
Executing: ProcessorState { ip: 8, relative_base: 9 } Equals { op1: Pos(100), op2: Imm(16), dst: Pos(101) }
Executing: ProcessorState { ip: 12, relative_base: 9 } JumpIfFalse { cond: Pos(101), target: Imm(0) }
Executing: ProcessorState { ip: 0, relative_base: 9 } AdjustRelativeBase { op: Imm(1) }
Executing: ProcessorState { ip: 2, relative_base: 10 } Output { src: Rel(-1) }
Executing: ProcessorState { ip: 4, relative_base: 10 } Add { op1: Pos(100), op2: Imm(1), dst: Pos(100) }
Executing: ProcessorState { ip: 8, relative_base: 10 } Equals { op1: Pos(100), op2: Imm(16), dst: Pos(101) }
Executing: ProcessorState { ip: 12, relative_base: 10 } JumpIfFalse { cond: Pos(101), target: Imm(0) }
Executing: ProcessorState { ip: 0, relative_base: 10 } AdjustRelativeBase { op: Imm(1) }
Executing: ProcessorState { ip: 2, relative_base: 11 } Output { src: Rel(-1) }
Executing: ProcessorState { ip: 4, relative_base: 11 } Add { op1: Pos(100), op2: Imm(1), dst: Pos(100) }
Executing: ProcessorState { ip: 8, relative_base: 11 } Equals { op1: Pos(100), op2: Imm(16), dst: Pos(101) }
Executing: ProcessorState { ip: 12, relative_base: 11 } JumpIfFalse { cond: Pos(101), target: Imm(0) }
Executing: ProcessorState { ip: 0, relative_base: 11 } AdjustRelativeBase { op: Imm(1) }
Executing: ProcessorState { ip: 2, relative_base: 12 } Output { src: Rel(-1) }
Executing: ProcessorState { ip: 4, relative_base: 12 } Add { op1: Pos(100), op2: Imm(1), dst: Pos(100) }
Executing: ProcessorState { ip: 8, relative_base: 12 } Equals { op1: Pos(100), op2: Imm(16), dst: Pos(101) }
Executing: ProcessorState { ip: 12, relative_base: 12 } JumpIfFalse { cond: Pos(101), target: Imm(0) }
Executing: ProcessorState { ip: 0, relative_base: 12 } AdjustRelativeBase { op: Imm(1) }
Executing: ProcessorState { ip: 2, relative_base: 13 } Output { src: Rel(-1) }
Executing: ProcessorState { ip: 4, relative_base: 13 } Add { op1: Pos(100), op2: Imm(1), dst: Pos(100) }
Executing: ProcessorState { ip: 8, relative_base: 13 } Equals { op1: Pos(100), op2: Imm(16), dst: Pos(101) }
Executing: ProcessorState { ip: 12, relative_base: 13 } JumpIfFalse { cond: Pos(101), target: Imm(0) }
Executing: ProcessorState { ip: 0, relative_base: 13 } AdjustRelativeBase { op: Imm(1) }
Executing: ProcessorState { ip: 2, relative_base: 14 } Output { src: Rel(-1) }
Executing: ProcessorState { ip: 4, relative_base: 14 } Add { op1: Pos(100), op2: Imm(1), dst: Pos(100) }
Executing: ProcessorState { ip: 8, relative_base: 14 } Equals { op1: Pos(100), op2: Imm(16), dst: Pos(101) }
Executing: ProcessorState { ip: 12, relative_base: 14 } JumpIfFalse { cond: Pos(101), target: Imm(0) }
Executing: ProcessorState { ip: 0, relative_base: 14 } AdjustRelativeBase { op: Imm(1) }
Executing: ProcessorState { ip: 2, relative_base: 15 } Output { src: Rel(-1) }
Executing: ProcessorState { ip: 4, relative_base: 15 } Add { op1: Pos(100), op2: Imm(1), dst: Pos(100) }
Executing: ProcessorState { ip: 8, relative_base: 15 } Equals { op1: Pos(100), op2: Imm(16), dst: Pos(101) }
Executing: ProcessorState { ip: 12, relative_base: 15 } JumpIfFalse { cond: Pos(101), target: Imm(0) }
Executing: ProcessorState { ip: 0, relative_base: 15 } AdjustRelativeBase { op: Imm(1) }
Executing: ProcessorState { ip: 2, relative_base: 16 } Output { src: Rel(-1) }
Executing: ProcessorState { ip: 4, relative_base: 16 } Add { op1: Pos(100), op2: Imm(1), dst: Pos(100) }
Executing: ProcessorState { ip: 8, relative_base: 16 } Equals { op1: Pos(100), op2: Imm(16), dst: Pos(101) }
Executing: ProcessorState { ip: 12, relative_base: 16 } JumpIfFalse { cond: Pos(101), target: Imm(0) }
Executing: ProcessorState { ip: 15, relative_base: 16 } Halt


