use crate::*;

pub fn rv64RegisterWordValid<P: Platform>(ctx: &Context<P>, mut funct3: f64, mut top: f64) -> f64 {
if (top == 1.0f64) {
return (if (((((funct3 == 0.0f64) || (funct3 == 4.0f64)) || (funct3 == 5.0f64)) || (funct3 == 6.0f64)) || (funct3 == 7.0f64)) { 1.0f64 } else { 0.0f64 });
}
if (funct3 == 0.0f64) {
return (if ((top == 0.0f64) || (top == 32.0f64)) { 1.0f64 } else { 0.0f64 });
}
if (funct3 == 1.0f64) {
return (if (top == 0.0f64) { 1.0f64 } else { 0.0f64 });
}
if (funct3 == 5.0f64) {
return (if ((top == 0.0f64) || (top == 32.0f64)) { 1.0f64 } else { 0.0f64 });
}
return 0.0f64;
}


pub fn rv64RegisterWordValue<P: Platform>(ctx: &Context<P>, mut funct3: f64, mut top: f64, mut left: i128, mut right: i128) -> i128 {
let shift: i128 = (crate::m_core__rv64_numbers::ux(ctx, right) & 31i128);
if (top == 1.0f64) {
{
if (funct3 == 0.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, (crate::m_core__rv64_numbers::sx(ctx, left, 32.0f64)).checked_mul(crate::m_core__rv64_numbers::sx(ctx, right, 32.0f64)).expect("BigInt arithmetic"), 32.0f64);
}
if (funct3 == 4.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, crate::m_core__rv64_arithmetic::rv64DivideSigned(ctx, left, right, 32.0f64), 32.0f64);
}
if (funct3 == 5.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, crate::m_core__rv64_arithmetic::rv64DivideUnsigned(ctx, crate::m_core__rv64_numbers::u32word(ctx, left), crate::m_core__rv64_numbers::u32word(ctx, right), 32.0f64), 32.0f64);
}
if (funct3 == 6.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, crate::m_core__rv64_arithmetic::rv64RemainderSigned(ctx, left, right, 32.0f64), 32.0f64);
}
if (funct3 == 7.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, crate::m_core__rv64_arithmetic::rv64RemainderUnsigned(ctx, crate::m_core__rv64_numbers::u32word(ctx, left), crate::m_core__rv64_numbers::u32word(ctx, right), 32.0f64), 32.0f64);
}
return 0i128;
}
}
if (funct3 == 0.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, (if (top == 0.0f64) { (left).checked_add(right).expect("BigInt arithmetic") } else { (left).checked_sub(right).expect("BigInt arithmetic") }), 32.0f64);
}
if (funct3 == 1.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, numbers::big_shl(crate::m_core__rv64_numbers::sx(ctx, left, 32.0f64), shift), 32.0f64);
}
if (funct3 == 5.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, (if (top == 0.0f64) { numbers::big_shr(crate::m_core__rv64_numbers::u32word(ctx, left), shift) } else { numbers::big_shr(crate::m_core__rv64_numbers::sx(ctx, left, 32.0f64), shift) }), 32.0f64);
}
return 0i128;
}


pub fn rv64ImmediateWordValid<P: Platform>(ctx: &Context<P>, mut funct3: f64, mut instruction: f64) -> f64 {
let top: f64 = numbers::ushr(instruction, 25.0f64);
if (funct3 == 0.0f64) {
return 1.0f64;
}
if (funct3 == 1.0f64) {
return (if (top == 0.0f64) { 1.0f64 } else { 0.0f64 });
}
if (funct3 == 5.0f64) {
return (if ((top == 0.0f64) || (top == 32.0f64)) { 1.0f64 } else { 0.0f64 });
}
return 0.0f64;
}


pub fn rv64ImmediateWordValue<P: Platform>(ctx: &Context<P>, mut funct3: f64, mut instruction: f64, mut left: i128) -> i128 {
let shift: i128 = crate::m_core__rv64_numbers::word(ctx, numbers::and(numbers::ushr(instruction, 20.0f64), 31.0f64));
if (funct3 == 0.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, (left).checked_add(crate::m_core__rv64_numbers::word(ctx, crate::m_vm__rv64_immediate::rv64ImmediateI(ctx, instruction))).expect("BigInt arithmetic"), 32.0f64);
}
if (funct3 == 1.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, numbers::big_shl(crate::m_core__rv64_numbers::sx(ctx, left, 32.0f64), shift), 32.0f64);
}
if (funct3 == 5.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, (if (numbers::ushr(instruction, 25.0f64) == 0.0f64) { numbers::big_shr(crate::m_core__rv64_numbers::u32word(ctx, left), shift) } else { numbers::big_shr(crate::m_core__rv64_numbers::sx(ctx, left, 32.0f64), shift) }), 32.0f64);
}
return 0i128;
}

