use crate::*;

pub fn rv64MulHighUnsigned<P: Platform>(ctx: &Context<P>, mut left: i128, mut right: i128) -> i128 {
let a0: i128 = (left & ctx.globals.m_core__rv64_arithmetic__MASK32.expect("global initialisation"));
let a1: i128 = numbers::big_shr(left, 32i128);
let b0: i128 = (right & ctx.globals.m_core__rv64_arithmetic__MASK32.expect("global initialisation"));
let b1: i128 = numbers::big_shr(right, 32i128);
let mut product: i128 = (a0).checked_mul(b0).expect("BigInt arithmetic");
let carry: i128 = numbers::big_shr(product, 32i128);
let _ = { product = ((a1).checked_mul(b0).expect("BigInt arithmetic")).checked_add(carry).expect("BigInt arithmetic"); product };
let middle: i128 = (product & ctx.globals.m_core__rv64_arithmetic__MASK32.expect("global initialisation"));
let high: i128 = numbers::big_shr(product, 32i128);
let _ = { product = ((a0).checked_mul(b1).expect("BigInt arithmetic")).checked_add(middle).expect("BigInt arithmetic"); product };
return crate::m_core__rv64_numbers::ux(ctx, (((a1).checked_mul(b1).expect("BigInt arithmetic")).checked_add(high).expect("BigInt arithmetic")).checked_add((numbers::big_shr(product, 32i128))).expect("BigInt arithmetic"));
}


pub fn rv64MulHighSigned<P: Platform>(ctx: &Context<P>, mut left: i128, mut right: i128) -> i128 {
let mut high: i128 = crate::m_core__rv64_arithmetic::rv64MulHighUnsigned(ctx, crate::m_core__rv64_numbers::ux(ctx, left), crate::m_core__rv64_numbers::ux(ctx, right));
if (left < 0i128) {
let _ = { high = crate::m_core__rv64_numbers::ux(ctx, (high).checked_sub(crate::m_core__rv64_numbers::ux(ctx, right)).expect("BigInt arithmetic")); high };
}
if (right < 0i128) {
let _ = { high = crate::m_core__rv64_numbers::ux(ctx, (high).checked_sub(crate::m_core__rv64_numbers::ux(ctx, left)).expect("BigInt arithmetic")); high };
}
return crate::m_core__rv64_numbers::sx(ctx, high, 64.0f64);
}


pub fn rv64MulHighSignedUnsigned<P: Platform>(ctx: &Context<P>, mut left: i128, mut right: i128) -> i128 {
let mut high: i128 = crate::m_core__rv64_arithmetic::rv64MulHighUnsigned(ctx, crate::m_core__rv64_numbers::ux(ctx, left), right);
if (left < 0i128) {
let _ = { high = crate::m_core__rv64_numbers::ux(ctx, (high).checked_sub(right).expect("BigInt arithmetic")); high };
}
return crate::m_core__rv64_numbers::sx(ctx, high, 64.0f64);
}


pub fn rv64DivideSigned<P: Platform>(ctx: &Context<P>, mut left: i128, mut right: i128, mut bits: f64) -> i128 {
let a: i128 = crate::m_core__rv64_numbers::sx(ctx, left, bits);
let b: i128 = crate::m_core__rv64_numbers::sx(ctx, right, bits);
if (b == 0i128) {
return (1i128).checked_neg().expect("BigInt negation");
}
let minimum: i128 = (if (bits == 32.0f64) { (2147483648i128).checked_neg().expect("BigInt negation") } else { (9223372036854775808i128).checked_neg().expect("BigInt negation") });
if ((a == minimum) && (b == (1i128).checked_neg().expect("BigInt negation"))) {
return a;
}
return crate::m_core__rv64_numbers::sx(ctx, (a).checked_div(b).expect("BigInt arithmetic"), bits);
}


pub fn rv64DivideUnsigned<P: Platform>(ctx: &Context<P>, mut left: i128, mut right: i128, mut bits: f64) -> i128 {
let a: i128 = (if (bits == 32.0f64) { crate::m_core__rv64_numbers::u32word(ctx, left) } else { crate::m_core__rv64_numbers::ux(ctx, left) });
let b: i128 = (if (bits == 32.0f64) { crate::m_core__rv64_numbers::u32word(ctx, right) } else { crate::m_core__rv64_numbers::ux(ctx, right) });
if (b == 0i128) {
return (if (bits == 32.0f64) { ctx.globals.m_core__rv64_arithmetic__MASK32.expect("global initialisation") } else { 18446744073709551615i128 });
}
return (if (bits == 32.0f64) { crate::m_core__rv64_numbers::u32word(ctx, (a).checked_div(b).expect("BigInt arithmetic")) } else { crate::m_core__rv64_numbers::ux(ctx, (a).checked_div(b).expect("BigInt arithmetic")) });
}


pub fn rv64RemainderSigned<P: Platform>(ctx: &Context<P>, mut left: i128, mut right: i128, mut bits: f64) -> i128 {
let a: i128 = crate::m_core__rv64_numbers::sx(ctx, left, bits);
let b: i128 = crate::m_core__rv64_numbers::sx(ctx, right, bits);
if (b == 0i128) {
return a;
}
let minimum: i128 = (if (bits == 32.0f64) { (2147483648i128).checked_neg().expect("BigInt negation") } else { (9223372036854775808i128).checked_neg().expect("BigInt negation") });
if ((a == minimum) && (b == (1i128).checked_neg().expect("BigInt negation"))) {
return 0i128;
}
return crate::m_core__rv64_numbers::sx(ctx, (a).checked_rem(b).expect("BigInt arithmetic"), bits);
}


pub fn rv64RemainderUnsigned<P: Platform>(ctx: &Context<P>, mut left: i128, mut right: i128, mut bits: f64) -> i128 {
let a: i128 = (if (bits == 32.0f64) { crate::m_core__rv64_numbers::u32word(ctx, left) } else { crate::m_core__rv64_numbers::ux(ctx, left) });
let b: i128 = (if (bits == 32.0f64) { crate::m_core__rv64_numbers::u32word(ctx, right) } else { crate::m_core__rv64_numbers::ux(ctx, right) });
if (b == 0i128) {
return a;
}
return (if (bits == 32.0f64) { crate::m_core__rv64_numbers::u32word(ctx, (a).checked_rem(b).expect("BigInt arithmetic")) } else { crate::m_core__rv64_numbers::ux(ctx, (a).checked_rem(b).expect("BigInt arithmetic")) });
}

