package disas

import (
    "fmt"
)

func Swap(str1, str2 *string) {
    *str1, *str2 = *str2, *str1
}

func HexStr(imm uint32, sign bool) string {
    str := "#"

    if !sign {
        str += "-"
    }

    str += "0x" + fmt.Sprintf("%x", imm)

    return str
}

func CpuReg(pos uint32) string {
    reg := pos & 0xf
    reg_name := "r" + fmt.Sprint(reg)

    switch(reg) {
        case 11: reg_name = "fp"
        case 12: reg_name = "ip"
        case 13: reg_name = "sp"
        case 14: reg_name = "lr"
        case 15: reg_name = "pc"
    }

    return reg_name
}

func Int2Bool(i uint32) bool {

    if i != 0 {
        return true
    } else {
        return false
    }
}

func CheckBit(bit bool, set string, unset string) string {

    if bit {
        return set
    } else {
        return unset
    }
}

func CheckCond(code uint32) string {

    condArray := [...] string {
        "eq",
        "ne",
        "hs",
        "lo",
        "mi",
        "pl",
        "vs",
        "vc",
        "hi",
        "ls",
        "ge",
        "lt",
        "gt",
        "le",
        "",     // "al"
    }

    return condArray[(code >> 28) & 0xf]
}
