use std;

enum ArithmeticTarget
{
    A, B, C, D, E, H, L,
}

enum Instruction
{
    // ADD can target all of the 8-bit registers except F
    ADD(ArithmeticTarget),
}
