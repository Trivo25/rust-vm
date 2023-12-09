# Little Rust VM

_Thanks to https://www.jmeiners.com/lc3-vm/#s0:0_

This project implements a basic virtual machine, simulating the fictional computer called LC-3 (Little Computer 3) with a simplified instruction set.

The LC-3 has a total of 2^16 or 65536 memory locations, each of which can store a 16bit value. As a result, LC-3 can store a total of 128kb.

A register is a basic container for storing a single value on the CPU. In order for the CPU to work with a piece of data, it needs to be accessible to the CPU - we do that by providing it via a register to it. Programs load data from memory into registers, each piece of data at a time.

Our example will have a total of 10 registers, each of which is 16bits. Most of them are general purpose except a few that handle the program counter (PC) and the conditional flags (COND).

The program counter (PC) is an unsigned integer which holds the address of the next instruction that needs to be executed in memory.

Instructions are commands that tell the CPU what to do. Instructions consist of both an opcode which indicates the type of task to perform and a bunch of parameters which provide inputs to the tasks.

Each opcode represents one task that the CPU can process. In LC-3, there are a total of 16 opcodes. Everything the computer can process is some combination of these 16 simple instructions. Each instruction is 16 bit in length, with the left first 4 bits storing the opcode and the rest is used to store the parameters.

The COND register stores conditional flags that provide information about most recent executed tasks. This allows programs to check logical conditions such as `if x > 0 {}`.

Each CPU has a variety of condition flags to signal different situations. The LC-3 uses 3 conditional flags which indicate the sign of the previous calculation.
