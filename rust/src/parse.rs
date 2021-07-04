use super::func;

fn shift_code(code: u32, cond: String, s_bit: bool, imm_bit: bool, reg: func::Reg) {
    
    // mov (imm)
    if imm_bit {
        println!("mov{}{} {}, {}", func::is_bit(s_bit, "s", ""), cond, reg.rd, func::hex_str(code & 0xff, true));
        return;
    }

    // mov (Rm)
    if ((code >> 4) & 0xff) == 0 {
        println!("mov{}{} {}, {}", func::is_bit(s_bit, "s", ""), cond, reg.rd, reg.rm);
        return;
    }
    
    let cmd = match (code >> 5) & 2 {
        0 => "lsl",
        1 => "lsr",
        2 => func::is_bit(((code >> 7) & 0xff) == 0, "rrx", "ror"),
        _ => ""
    };

    if ((code >> 4) & 1) == 1 {
        println!("{}{}{} {}, {}, {}", cmd, func::is_bit(s_bit, "s", ""), cond, reg.rd, reg.rm, reg.rs);
    } else {
        println!("{}{}{} {}, {}, {}", cmd, func::is_bit(s_bit, "s", ""), cond, reg.rd, reg.rm, func::hex_str((code >> 7) & 0x1f, true));
    }
}

pub fn data_proc(code: u32, cond: String, mut reg: func::Reg) {
    let mut flag = "".to_string();

    let psr = ((code >> 22) & 1) != 0;
    let s_bit = ((code >> 20) & 1) != 0;
    let imm_bit = ((code >> 25) & 1) != 0;
    let bx_mode = ((code >> 4) & 0xffffff) == 0x12fff1;
    let cmp_mode = ((code >> 23) & 3) == 2;
    let mul_mode = !imm_bit && ((code >> 4) & 0xf) == 9;

    let cmd_list = [
        func::is_bit(mul_mode, "mul", "and"),
        func::is_bit(mul_mode, "mla", "eor"),

        "sub",
        "rsb",

        func::is_bit(mul_mode, "umull", "add"),
        func::is_bit(mul_mode, "umlal", "adc"),
        func::is_bit(mul_mode, "smull", "sbc"),
        func::is_bit(mul_mode, "smlal", "rsc"),

        func::is_bit(s_bit, "tst", "mrs"),
        func::is_bit(s_bit, "teq", "msr"),
        func::is_bit(s_bit, "cmp", "mrs"),
        func::is_bit(s_bit, "cmn", "msr"),

        "orr",
        "",     // shift code
        "bic",
        "mvn"
    ];

    let flag_list = [
        'c',
        'x',
        's',
        'f'
    ];

    for i in 0..3 {
        if ((code >> (16 + i)) & 1) == 1 {
            flag.push(flag_list[i]);
        }
    }

    // Shift code
    if ((code >> 21) & 0xf) == 0xd {
        shift_code(code, cond, s_bit, imm_bit, reg);
        return;
    }

    if ((code >> 20) & 1) == 0 {
        if ((code >> 4) & 0xf) == 0xd {
            println!("Not supported: ldrd");
            return;
        } else if ((code >> 4) & 0xf) == 0xf {
            println!("Not supported: strd");
            return;
        }
    }

    let cmd = cmd_list[((code >> 21) & 0xf) as usize];

    // An immediate value exists
    if imm_bit {
        if cmd == "msr" {
            println!("msr {}_{}, {}", func::is_bit(psr, "spsr", "cpsr"), flag, func::hex_str(code & 0xff, true));
        } else if cmp_mode {
            println!("{}{} {}, {}", cmd, cond, reg.rn, func::hex_str(code & 0xff, true));
        } else {
            println!("{}{}{} {}, {}{}", cmd, func::is_bit(s_bit, "s", ""), cond, reg.rd, func::is_bit(cmd == "mvn", "", &(reg.rn + ", ")), func::hex_str(code & 0xff, true));
        }
        return;
    }

    // Branch and exchange instruction set
    if bx_mode {
        println!("bx{} {}", cond, reg.rm);
        return;
    }

    // Compare
    if cmp_mode {
        if cmd == "mrs" {
            println!("mrs {}, {}", reg.rd, func::is_bit(psr, "spsr", "cpsr"));
        } else if cmd == "msr" {
            println!("msr {}_{}, {}", func::is_bit(psr, "spsr", "cpsr"), flag, reg.rm);
        } else {
            println!("{}{} {}, {}", cmd, cond, reg.rn, reg.rm);
        }
        return;
    }

    // Multiply
    if mul_mode {
        if cmd == "mla" {
            std::mem::swap(&mut reg.rd, &mut reg.rn);
            println!("{}{}{} {}, {}, {}, {}", cmd, func::is_bit(s_bit, "s", ""), cond, reg.rd, reg.rm, reg.rs, reg.rn);
        } else if cmd == "mul" {
            reg.rd = reg.rn;
            println!("{}{}{} {}, {}, {}", cmd, func::is_bit(s_bit, "s", ""), cond, reg.rd, reg.rm, reg.rs);
        } else {
            println!("{}{}{} {}, {}, {}, {}", cmd, func::is_bit(s_bit, "s", ""), cond, reg.rd, reg.rn, reg.rm, reg.rs);
        }
        return;
    }

    println!("{}{}{} {}, {}{}", cmd, func::is_bit(s_bit, "s", ""), cond, reg.rd, func::is_bit(cmd == "mvn", "", &(reg.rn + ", ")), reg.rm);
}

pub fn single_memory(code: u32, cond: String, reg: func::Reg) {
    let l_bit = ((code >> 20) & 1) != 0;
    let w_bit = ((code >> 21) & 1) != 0;
    let b_bit = ((code >> 22) & 1) != 0;
    let u_bit = ((code >> 23) & 1) != 0;
    let p_bit = ((code >> 24) & 1) != 0;
    let no_imm = ((code >> 25) & 1) != 0;

    let imm12 = code & 0xfff;
    let immstr = func::hex_str(imm12, u_bit);

    let b = func::is_bit(b_bit, "b", "");
    let cmd = func::is_bit(l_bit, "ldr", "str");
    let wbk = func::is_bit(w_bit, "!", "");
    let imm = func::is_bit(imm12 != 0, &immstr, "");

    if no_imm {
        if p_bit {
            println!("{}{}{} {}, [{}, {}]{}", cmd, b, cond, reg.rd, reg.rn, reg.rm, wbk);
        } else {
            println!("{}{}{} {}, [{}], {}", cmd, b, cond, reg.rd, reg.rn, reg.rm);
        }
    } else {
        if p_bit {
            println!("{}{}{} {}, [{}, {}]{}", cmd, b, cond, reg.rd, reg.rn, imm, wbk);
        } else {
            println!("{}{}{} {}, [{}], {}", cmd, b, cond, reg.rd, reg.rn, func::hex_str(imm12, u_bit));
        }
    }
}

pub fn multi_memory(code: u32, cond: String, reg: func::Reg) {
    let load = ((code >> 20) & 1) != 0;
    let adr_bit = (code >> 23) & 3;
    let mut reglist = "".to_string();

    let mut adr_mode = [
        "da",
        "ia",
        "db",
        "ib"
    ];

    if reg.rn == "sp" {
        adr_mode[adr_bit as usize] = match adr_bit {
            0 => func::is_bit(load, "fa", "ed"),
            1 => func::is_bit(load, "fd", "ea"),
            2 => func::is_bit(load, "ea", "fd"),
            3 => func::is_bit(load, "ed", "fa"),
            _ => ""
        };
    }

    for i in 0..15 {
        if ((code >> i) & 1) == 1 {
            let newreg = func::check_reg(i as u32);

            if reglist.is_empty() {
                reglist.push('{');
            } else {
                reglist.push_str(", ");
            }

            reglist.push_str(&newreg);
        }
    }

    if reglist.is_empty() {
        reglist.push('{');
    }

    reglist.push('}');

    if load {
        println!("ldm{}{} {}!, {}", adr_mode[adr_bit as usize], cond, reg.rn, reglist);
    } else {
        println!("stm{}{} {}!, {}", adr_mode[adr_bit as usize], cond, reg.rn, reglist);
    }
}

pub fn branch(code: u32, cond: String) {
    let link = ((code >> 24) & 1) != 0;
    let imm = (((code & 0xffffff) | (0xff << 24)) << 2) + 8;

    if link {
        println!("bl{} {}", cond, func::hex_str(imm, true));
    } else {
        println!("b{} {}", cond, func::hex_str(imm, true));
    }
}

pub fn sw_interrupt(code: u32, cond: String) {
    let svc_num = code & 0xffffff;
    println!("svc{} {}", cond, func::hex_str(svc_num, true));
}

pub fn blx_code(code: u32) {
    let imm = (((code & 0xffffff) | (0xff << 24)) << 2) + 8;
    println!("blx {}", func::hex_str(imm, true));
}