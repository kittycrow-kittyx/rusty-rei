use crate::*;

pub fn rv64ImmediateOpValid<P: Platform>(ctx: &Context<P>, mut funct3: f64, mut instruction: f64) -> f64 {
if (funct3 > 7.0f64) {
return 0.0f64;
}
let top: f64 = numbers::ushr(instruction, 26.0f64);
if (funct3 == 1.0f64) {
return (if (top == 0.0f64) { 1.0f64 } else { 0.0f64 });
}
if (funct3 == 5.0f64) {
return (if ((top == 0.0f64) || (top == 16.0f64)) { 1.0f64 } else { 0.0f64 });
}
return 1.0f64;
}


pub fn rv64ImmediateOpValue<P: Platform>(ctx: &Context<P>, mut funct3: f64, mut instruction: f64, mut left: i128) -> i128 {
let immediate: i128 = crate::m_core__rv64_numbers::word(ctx, crate::m_vm__rv64_immediate::rv64ImmediateI(ctx, instruction));
let shift: i128 = crate::m_core__rv64_numbers::word(ctx, numbers::and(numbers::ushr(instruction, 20.0f64), 63.0f64));
if (funct3 == 0.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, (left).checked_add(immediate).expect("BigInt arithmetic"), 64.0f64);
}
if (funct3 == 1.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, numbers::big_shl(crate::m_core__rv64_numbers::ux(ctx, left), shift), 64.0f64);
}
if (funct3 == 2.0f64) {
return (if (left < immediate) { 1i128 } else { 0i128 });
}
if (funct3 == 3.0f64) {
return (if (crate::m_core__rv64_numbers::ux(ctx, left) < crate::m_core__rv64_numbers::ux(ctx, immediate)) { 1i128 } else { 0i128 });
}
if (funct3 == 4.0f64) {
return (left ^ immediate);
}
if (funct3 == 5.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, (if (numbers::ushr(instruction, 26.0f64) == 0.0f64) { numbers::big_shr(crate::m_core__rv64_numbers::ux(ctx, left), shift) } else { numbers::big_shr(left, shift) }), 64.0f64);
}
if (funct3 == 6.0f64) {
return (left | immediate);
}
if (funct3 == 7.0f64) {
return (left & immediate);
}
return 0i128;
}

