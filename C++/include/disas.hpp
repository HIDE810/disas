#pragma once

#include "func.hpp"

extern std::string Rm, Rs, Rd, Rn;

std::string check_cond(int code);
void data_proc(int code, std::string cond);
void single_memory(int code, std::string cond);
void multi_memory(int code, std::string cond);
void branch(int code, std::string cond);
void sw_interrupt(int code, std::string cond);
void blx_code(int code);