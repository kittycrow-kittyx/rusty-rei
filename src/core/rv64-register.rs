use crate::*;

pub fn rv64RegisterValid<P: Platform>(ctx: &Context<P>, mut funct3: f64, mut top: f64) -> f64 {
if (funct3 > 7.0f64) {
return 0.0f64;
}
if ((top == 0.0f64) || (top == 1.0f64)) {
return 1.0f64;
}
return (if ((top == 32.0f64) && (((funct3 == 0.0f64) || (funct3 == 5.0f64)))) { 1.0f64 } else { 0.0f64 });
}


pub fn rv64RegisterValue<P: Platform>(ctx: &Context<P>, mut funct3: f64, mut top: f64, mut left: i128, mut right: i128) -> i128 {
let shift: i128 = (crate::m_core__rv64_numbers::ux(ctx, right) & 63i128);
if (top == 1.0f64) {
{
if (funct3 == 0.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, (left).checked_mul(right).expect("BigInt arithmetic"), 64.0f64);
}
if (funct3 == 1.0f64) {
return crate::m_core__rv64_arithmetic::rv64MulHighSigned(ctx, left, right);
}
if (funct3 == 2.0f64) {
return crate::m_core__rv64_arithmetic::rv64MulHighSignedUnsigned(ctx, left, crate::m_core__rv64_numbers::ux(ctx, right));
}
if (funct3 == 3.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, crate::m_core__rv64_arithmetic::rv64MulHighUnsigned(ctx, crate::m_core__rv64_numbers::ux(ctx, left), crate::m_core__rv64_numbers::ux(ctx, right)), 64.0f64);
}
if (funct3 == 4.0f64) {
return crate::m_core__rv64_arithmetic::rv64DivideSigned(ctx, left, right, 64.0f64);
}
if (funct3 == 5.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, crate::m_core__rv64_arithmetic::rv64DivideUnsigned(ctx, crate::m_core__rv64_numbers::ux(ctx, left), crate::m_core__rv64_numbers::ux(ctx, right), 64.0f64), 64.0f64);
}
if (funct3 == 6.0f64) {
return crate::m_core__rv64_arithmetic::rv64RemainderSigned(ctx, left, right, 64.0f64);
}
if (funct3 == 7.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, crate::m_core__rv64_arithmetic::rv64RemainderUnsigned(ctx, crate::m_core__rv64_numbers::ux(ctx, left), crate::m_core__rv64_numbers::ux(ctx, right), 64.0f64), 64.0f64);
}
return 0i128;
}
}
if (funct3 == 0.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, (if (top == 0.0f64) { (left).checked_add(right).expect("BigInt arithmetic") } else { (left).checked_sub(right).expect("BigInt arithmetic") }), 64.0f64);
}
if (funct3 == 1.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, numbers::big_shl(crate::m_core__rv64_numbers::ux(ctx, left), shift), 64.0f64);
}
if (funct3 == 2.0f64) {
return (if (left < right) { 1i128 } else { 0i128 });
}
if (funct3 == 3.0f64) {
return (if (crate::m_core__rv64_numbers::ux(ctx, left) < crate::m_core__rv64_numbers::ux(ctx, right)) { 1i128 } else { 0i128 });
}
if (funct3 == 4.0f64) {
return (left ^ right);
}
if (funct3 == 5.0f64) {
return crate::m_core__rv64_numbers::sx(ctx, (if (top == 0.0f64) { numbers::big_shr(crate::m_core__rv64_numbers::ux(ctx, left), shift) } else { numbers::big_shr(left, shift) }), 64.0f64);
}
if (funct3 == 6.0f64) {
return (left | right);
}
if (funct3 == 7.0f64) {
return (left & right);
}
return 0i128;
}

