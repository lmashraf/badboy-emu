use std;

enum ArithmeticTarget
{
    A, B, C, D, E, H, L,
}

enum Instruction
{
    ADD(ArithmeticTarget),
}
