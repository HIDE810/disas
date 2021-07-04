pub struct Reg {
    pub rm: String,
    pub rs: String,
    pub rd: String,
    pub rn: String,
}

pub fn check_reg(pos: u32) -> String {
    let reg = pos & 0xf;
    let reg_name = format!("r{}", reg);

    match reg {
        11 => "fp",
        12 => "ip",
        13 => "sp",
        14 => "lr",
        15 => "pc",
         _ => &reg_name
    }.to_string()
}

pub fn reg_list(code: u32) -> Reg {
    Reg {
        rm: check_reg(code),        // Rm
        rs: check_reg(code >> 8),   // Rs
        rd: check_reg(code >> 12),  // Rd
        rn: check_reg(code >> 16),  // Rn
    }
}

pub fn check_cond(code: u32) -> String {

    let cond: [&str; 15] = [
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
        ""      // "al" 
    ];

    cond[((code >> 28) & 0xf) as usize].to_string()
}

pub fn is_bit<'a>(bit: bool, set: &'a str, unset: &'a str) -> &'a str {
    if bit {
        set
    } else {
        unset
    }
}

pub fn hex_str(imm: u32, sign: bool) -> String {
    let mut str = "#".to_string();

    if !sign {
        str.push('-');
    }

    format!("{}0x{:x}", str, imm)
}