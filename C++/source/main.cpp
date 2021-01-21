#include <iostream>

#include "disas.hpp"

std::string Rm, Rs, Rd, Rn;

int main(int argc, char *argv[]){

    int code;

    if(argc > 1){
        sscanf(argv[1], "%x", &code);
    }
    else{
        std::cout << "Error: No argument." << std::endl;
        return 1;
    }
    
    Rm = cpu_reg(code);
    Rs = cpu_reg(code >> 8);
    Rd = cpu_reg(code >> 12);
    Rn = cpu_reg(code >> 16);

    if(((code >> 28) & 0xf) == 0xf){
        blx_code(code);
    }
    else{
        int op = (code >> 26) & 3;
        bool branch_bit = (code >> 25) & 1;
        std::string cond = check_cond(code);

        switch(op){
            case 0: data_proc(code, cond); break;
            case 1: single_memory(code, cond); break;
            case 2: branch_bit ? branch(code, cond) : multi_memory(code, cond); break;
            case 3: sw_interrupt(code, cond); break;
        }
    }

    return 0;
}