#include <iostream>
#include <string>
#include <vector>

#include "disas.hpp"

std::string check_cond(int code){
    
    std::vector<std::string> cond_name = {
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
    };
    
    std::string cond = cond_name[(code >> 28) & 0xf];
    
    return cond;
}

void shift_code(int code, std::string cond, bool s_bit, bool imm_bit){

    std::string cmd;

    if(imm_bit){
        std::cout << "mov" << (s_bit ? "s" : "") << cond << " " << Rd << ", " << hex_str(code & 0xff, true) << std::endl;
        return;
    }
    
    if(((code >> 4) & 0xff) == 0){
        std::cout << "mov" << (s_bit ? "s" : "") << cond << " " << Rd << ", " << Rm << std::endl;
    }
    else{
        switch((code >> 5) & 2){
            case 0: cmd = "lsl"; break;
            case 1: cmd = "lsr"; break;
            case 2: cmd = ((code >> 7) & 0xff ? "ror" : "rrx"); break;
        }
        
        (code >> 4) & 1 ?
            std::cout << cmd << (s_bit ? "s" : "") << cond << " " << Rd << ", " << Rm << ", " << Rs << std::endl:
            std::cout << cmd << (s_bit ? "s" : "") << cond << " " << Rd << ", " << Rm << ", " << hex_str((code >> 7) & 0x1f, true) << std::endl;
    }
}

void data_proc(int code, std::string cond){

    std::string cmd, flag;

    bool psr = (code >> 22) & 1;
    bool s_bit = (code >> 20) & 1;
    bool imm_bit = (code >> 25) & 1;
    bool bx_mode = ((code >> 20) & 0x1f) == 0x12;
    bool cmp_mode = ((code >> 23) & 3) == 2;
    bool mul_mode = !imm_bit && ((code >> 4) & 0xf) == 9;

    std::vector<std::string> cmd_name = {
        
        mul_mode ? "mul" : "and",
        mul_mode ? "mla" : "eor",
        
        "sub",
        "rsb",
        
        mul_mode ? "umull" : "add",
        mul_mode ? "umlal" : "adc",
        mul_mode ? "smull" : "sbc",
        mul_mode ? "smlal" : "rsc",
        
        s_bit ? "tst" : "mrs",
        s_bit ? "teq" : "msr",
        s_bit ? "cmp" : "mrs",
        s_bit ? "cmn" : "msr",
        
        "orr",
        "",     // shift code
        "bic",
        "mvn"
    };

    std::vector<std::string> flag_list = {
        "c",
        "x",
        "s",
        "f"
    };

    for(int i = 0; i < 4; i++){
        if((code >> (16 + i)) & 1){
            flag += flag_list[i];
        }
    } 

    // nop
    if(code == 0xe320f000){
        std::cout << "nop" << std::endl;
        return;
    }

    // Opcode is shift code
    if(((code >> 21) & 0xf) == 0xd){
        shift_code(code, cond, s_bit, imm_bit);
        return;
    }

    cmd = cmd_name[(code >> 21) & 0xf];

    // An immediate value exists
    if(imm_bit){
        if(cmd == "msr"){
            std::cout << cmd << " " << (psr ? "spsr" : "cpsr") << "_" << flag << ", " << hex_str(code & 0xff, true) << std::endl;
        }
        else if(cmp_mode){
            std::cout << cmd << cond << " " << Rn << ", " << hex_str(code & 0xff, true) << std::endl;
        }
        else{
            std::cout << cmd << (s_bit ? "s" : "") << cond << " " << Rd << ", " << (cmd == "mvn" ? "" : Rn + ", ") \
            << hex_str(code & 0xff, true) << std::endl;
        }
        return;
    }

    if(bx_mode){
        std::cout << "bx" << cond << " " << Rm << std::endl;
    }
    else if(cmp_mode){
        if(cmd == "mrs")
            std::cout << cmd << " " << Rd << ", " << (psr ? "spsr" : "cpsr") << std::endl;
        else if(cmd == "msr")
            std::cout << cmd << " " << (psr ? "spsr" : "cpsr") << "_" << flag << ", " << Rm << std::endl;
        else
            std::cout << cmd << cond << " " << Rn << ", " << Rm << std::endl;
    }
    else if(mul_mode){
        if(cmd == "mla")
            Rd.swap(Rn);

        std::cout << cmd << (s_bit ? "s" : "") << cond << " " << Rd << ", " \
        << ((code >> 23) & 1 ? (Rn + ", ") : "") << Rm << ", " << Rs \
        << (cmd == "mla" ? (", " + Rn) : "") << std::endl;
    }
    else{
        std::cout << cmd << (s_bit ? "s" : "") << cond << " " << Rd << ", " \
        << (cmd == "mvn" ? "" : Rn + ", ") << Rm << std::endl;
    }
}

void single_memory(int code, std::string cond){

    bool l_bit = (code >> 20) & 1;
    bool w_bit = (code >> 21) & 1;
    bool b_bit = (code >> 22) & 1;
    bool u_bit = (code >> 23) & 1;
    bool p_bit = (code >> 24) & 1;
    bool imm_bit = !((code >> 25) & 1);

    int imm12 = code & 0xfff;

    std::string cmd = l_bit ? "ldr" : "str";
    std::string byte = b_bit ? "b" : "";
    std::string write_back = w_bit ? "!" : "";
    std::string imm = imm12 ? (", " + hex_str(imm12, u_bit)) : "";

    if(imm_bit){
        p_bit?
            std::cout << cmd << byte << cond << " " << Rd << ", [" << Rn << imm << "]" << write_back << std::endl:
            std::cout << cmd << byte << cond << " " << Rd << ", [" << Rn << "], " << hex_str(imm12, u_bit) << std::endl;
    }
    else{
        p_bit?
            std::cout << cmd << byte << cond << " " << Rd << ", [" << Rn << ", " << Rm << "]" << write_back << std::endl:
            std::cout << cmd << byte << cond << " " << Rd << ", [" << Rn << "], " << Rm << std::endl;
    }
}

void multi_memory(int code, std::string cond){

    bool cmd = (code >> 20) & 1;
    std::string reg_list;

    std::vector<std::string> adr_mode = {
        Rn == "sp" ? (cmd ? "fa" : "ed") : "da",
        Rn == "sp" ? (cmd ? "fd" : "ea") : "ia",
        Rn == "sp" ? (cmd ? "ea" : "fd") : "db",
        Rn == "sp" ? (cmd ? "ed" : "fa") : "ib"
    };

    for(int i = 0; i < 16; i++){
        if((code >> i) & 1)
            reg_list += (reg_list.empty() ? "{" : ", ") + cpu_reg(i);
        
        if(i == 15)
            reg_list += "}";
    }

    std::cout << (cmd ? "ldm" : "stm") << adr_mode[(code >> 23) & 3] << cond << " " << Rn << "!, " << reg_list << std::endl;
}

void branch(int code, std::string cond){

    bool link = (code >> 24) & 1;
    int imm = (((code & 0xffffff) | (0xff << 24)) << 2) + 8;

    std::cout << (link ? "bl" : "b") << cond << " " << hex_str(imm, true) << std::endl;
}

void sw_interrupt(int code, std::string cond){
    int svc_num = code & 0xffffff;
    std::cout << "svc" << cond << " " << hex_str(svc_num, true) << std::endl;
}

void blx_code(int code){
    int imm = (((code & 0xffffff) | (0xff << 24)) << 2) + 8;
    std::cout << "blx " << hex_str(imm, true) << std::endl;
}
