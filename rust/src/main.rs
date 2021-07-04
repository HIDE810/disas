mod func;
mod parse;

use std::u32;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "INPUT")]
    input: String,
}

fn main() {
    let opt = Opt::from_args();
    let code = u32::from_str_radix(&opt.input, 16).unwrap();
    let reg = func::reg_list(code);

    if ((code >> 28) & 0xf) == 0xf {
        parse::blx_code(code);
    } else {
        let op = (code >> 26) & 3;
        let br_bit = ((code >> 25) & 1) == 1;
        let cond = func::check_cond(code);

        match op {
            0 => parse::data_proc(code, cond, reg),
            1 => parse::single_memory(code, cond, reg),
            2 => 
                if br_bit {
                    parse::branch(code, cond)
                } else {
                    parse::multi_memory(code, cond, reg)
                },
            3 => parse::sw_interrupt(code, cond),
            _ => ()
        }
    }
}
