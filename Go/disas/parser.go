package disas

import (
    "fmt"
    "strings"
)

func ShiftCode(code uint32, cond string, s_bit bool, imm_bit bool, Rm string, Rs string, Rd string) {

    var cmd string

    // mov (imm)
    if imm_bit {
        fmt.Printf("mov%v%v %v, %v\n", CheckBit(s_bit, "s", ""), cond, Rd, HexStr(code & 0xff, true))
        return
    }

    // mov (Rm)
    if ((code >> 4) & 0xff) == 0 {
        fmt.Printf("mov%v%v %v, %v\n", CheckBit(s_bit, "s", ""), cond, Rd, Rm)
        return
    }

    switch((code >> 5) & 2) {
		case 0: cmd = "lsl"
        case 1: cmd = "lsr"
        case 2: cmd = CheckBit(((code >> 7) & 0xff) == 0, "rrx", "ror")
    }

    if ((code >> 4) & 1) == 1 {
        fmt.Printf("%v%v%v %v, %v, %v\n", cmd, CheckBit(s_bit, "s", ""), cond, Rd, Rm, Rs)
    } else {
        fmt.Printf("%v%v%v %v, %v, %v\n", cmd, CheckBit(s_bit, "s", ""), cond, Rd, Rm, HexStr((code >> 7) & 0x1f, true))
    }
}

func DataProc(code uint32, cond string, Rm string, Rs string, Rd string, Rn string) {

    var flag string

	psr := ((code >> 22) & 1) == 1
    s_bit := ((code >> 20) & 1) == 1
    imm_bit := ((code >> 25) & 1) == 1
    bx_mode := ((code >> 20) & 0x1f) == 0x12
    cmp_mode := ((code >> 23) & 3) == 2
    mul_mode := !imm_bit && ((code >> 4) & 0xf) == 9

    cmdArray := [...] string {
		CheckBit(mul_mode, "mul", "and"),
        CheckBit(mul_mode, "mla", "eor"),

        "sub",
        "rsb",

        CheckBit(mul_mode, "umull", "add"),
        CheckBit(mul_mode, "umlal", "adc"),
        CheckBit(mul_mode, "smull", "sbc"),
        CheckBit(mul_mode, "smlal", "rsc"),

        CheckBit(s_bit, "tst", "mrs"),
        CheckBit(s_bit, "teq", "msr"),
        CheckBit(s_bit, "cmp", "mrs"),
        CheckBit(s_bit, "cmn", "msr"),

        "orr",
        "",     // Shift code
        "bic",
        "mvn",
    }

    flagArray := [...] string {
        "c",
        "x",
        "s",
        "f",
    }

    for i, v := range flagArray {
        if ((code >> (16 + i)) & 1) == 1 {
            flag += v
        }
    }

    // Shift code
    if ((code >> 21) & 0xf) == 0xd {
        ShiftCode(code, cond, s_bit, imm_bit, Rm, Rs, Rd)
        return
    }

    cmd := cmdArray[(code >> 21) & 0xf]

    // An immediate value exists
    if imm_bit {
        if cmd == "msr" {
            fmt.Printf("msr %v_%v, %v\n", CheckBit(psr, "spsr", "cpsr"), flag, HexStr(code & 0xff, true))
        } else if cmp_mode {
            fmt.Printf("%v%v %v, %v\n", cmd, cond, Rn, HexStr(code & 0xff, true))
        } else {
            fmt.Printf("%v%v%v %v, %v%v\n", cmd, CheckBit(s_bit, "s", ""), cond, Rd, CheckBit(cmd == "mvn", "", Rn + ", "), HexStr(code & 0xff, true))
        }
        return
    }

    // Branch and exchange instruction set
    if bx_mode {
        fmt.Printf("bx%v %v\n", cond, Rm)
        return
    }

    // Compare
    if cmp_mode {
        if cmd == "mrs" {
            fmt.Printf("mrs %v, %v\n", Rd, CheckBit(psr, "spsr", "cpsr"))
        } else if cmd == "msr" {
            fmt.Printf("msr %v_%v, %v\n", CheckBit(psr, "spsr", "cpsr"), flag, Rm)
        } else {
            fmt.Printf("%v%v %v, %v\n", cmd, cond, Rm, Rn)
        }
        return
    }

    // Multiply
    if mul_mode {
        if cmd == "mla" {
            Swap(&Rd, &Rn)
            fmt.Printf("%v%v%v %v, %v, %v, %v\n", cmd, CheckBit(s_bit, "s", ""), cond, Rd, Rm, Rs, Rn)
        } else if cmd == "mul" {
            fmt.Printf("%v%v%v %v, %v, %v\n", cmd, CheckBit(s_bit, "s", ""), cond, Rd, Rm, Rs)
        } else {
            fmt.Printf("%v%v%v %v, %v, %v, %v\n", cmd, CheckBit(s_bit, "s", ""), cond, Rd, Rn, Rm, Rs)
        }
        return
    }

    fmt.Printf("%v%v%v %v, %v%v\n", cmd, CheckBit(s_bit, "s", ""), cond, Rd, CheckBit(cmd == "mvn", "", Rn + ", "), Rm)
}

func SingleMemory(code uint32, cond string, Rm string, Rd string, Rn string) {

	l_bit := (code >> 20) & 1 == 1
    w_bit := (code >> 21) & 1 == 1
    b_bit := (code >> 22) & 1 == 1
    u_bit := ((code >> 23) & 1) == 1
    p_bit := ((code >> 24) & 1) == 1
    no_imm := ((code >> 25) & 1) == 1

    imm12 := code & 0xfff

    b := CheckBit(b_bit, "b", "")
    cmd := CheckBit(l_bit, "ldr", "str")
    write_back := CheckBit(w_bit, "!", "")
    imm := CheckBit(Int2Bool(imm12), ", " + HexStr(imm12, u_bit), "")

   if no_imm {
        if p_bit {
            fmt.Printf("%v%v%v %v, [%v, %v]%v\n", cmd, b, cond, Rd, Rn, Rm, write_back)
        } else {
            fmt.Printf("%v%v%v %v, [%v], %v\n", cmd, b, cond, Rd, Rn, Rm)
        }
    } else {
        if p_bit {
            fmt.Printf("%v%v%v %v, [%v%v]%v\n", cmd, b, cond, Rd, Rn, imm, write_back)
        } else {
            fmt.Printf("%v%v%v %v, [%v], %v\n", cmd, b, cond, Rd, Rn, HexStr(imm12, u_bit))
        }
    }
}

func MultiMemory(code uint32, cond string, Rn string) {

    load := ((code >> 20) & 1) == 1
    adr_bit := (code >> 23) & 3

    adr_mode := [...] string {
        "da",
        "ia",
        "db",
        "ib",
    }

    var reg_list []string

    if Rn == "sp" {
        switch(adr_bit) {
            case 0: adr_mode[0] = CheckBit(load, "fa", "ed")
            case 1: adr_mode[1] = CheckBit(load, "fd", "ea")
            case 2: adr_mode[2] = CheckBit(load, "ea", "fd")
            case 3: adr_mode[3] = CheckBit(load, "ed", "fa")
        }
    }

    for i := 0; i < 16; i++ {
        if ((code >> i) & 1) == 1 {
            reg_list = append(reg_list, CpuReg(uint32(i)))
        }
    }

    if load {
        fmt.Printf("ldm%v%v %v!, {%v}\n", adr_mode[adr_bit], cond, Rn, strings.Join(reg_list, ", "))
    } else {
        fmt.Printf("stm%v%v %v!, {%v}\n", adr_mode[adr_bit], cond, Rn, strings.Join(reg_list, ", "))
    }
}

func Branch(code uint32, cond string) {
    link := ((code >> 24) & 1) == 1
    imm := (code & 0xffffff) << 2 | (0xff << 24) + 8

    if link {
        fmt.Println("bl " + cond + HexStr(imm, true))
    } else {
        fmt.Println("b " + cond + HexStr(imm, true))
    }
}


func SwInterrupt(code uint32, cond string) {
    svc_num := code & 0xffffff
    fmt.Println("svc " + HexStr(svc_num, true))
}

func BlxMode(code uint32) {
    imm := (code & 0xffffff) << 2 | (0xff << 24) + 8
    fmt.Println("blx " + HexStr(imm, true))
}
