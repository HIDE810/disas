package main

import (
    "./disas"
    "flag"
    "strconv"
)

func main() {
    flag.Parse()
    code64, _ := strconv.ParseInt(flag.Arg(0), 16, 64)
    code := uint32(code64)

    Rm := disas.CpuReg(code)
    Rs := disas.CpuReg(code >> 8)
    Rd := disas.CpuReg(code >> 12)
    Rn := disas.CpuReg(code >> 16)

    if ((code >> 28) & 0xf) == 0xf {
        disas.BlxMode(code)
    } else {
        op := (code >> 26) & 3
        branch_bit := (code >> 25) & 1 == 1
        cond := disas.CheckCond(code)

        switch(op) {
            case 0:
                disas.DataProc(code, cond, Rm, Rs, Rd, Rn)
            case 1:
                disas.SingleMemory(code, cond, Rm, Rd, Rn)
            case 2:
                if branch_bit {
                    disas.Branch(code, cond)
                } else {
                    disas.MultiMemory(code, cond, Rn)
                }
            case 3:
                disas.SwInterrupt(code, cond)
        }
    }
}
