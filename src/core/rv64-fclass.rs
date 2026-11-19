use crate::*;

pub fn rv64FloatClass<P: Platform>(ctx: &Context<P>, mut raw: i128, mut bits: f64) -> i128 {
let exponentBits: i128 = (if (bits == 32.0f64) { 8i128 } else { 11i128 });
let fractionBits: i128 = (if (bits == 32.0f64) { 23i128 } else { 52i128 });
let sign: i128 = (numbers::big_shr(raw, ((exponentBits).checked_add(fractionBits).expect("BigInt arithmetic"))) & 1i128);
let exponent: i128 = (numbers::big_shr(raw, fractionBits) & (((numbers::big_shl(1i128, exponentBits))).checked_sub(1i128).expect("BigInt arithmetic")));
let fraction: i128 = (raw & (((numbers::big_shl(1i128, fractionBits))).checked_sub(1i128).expect("BigInt arithmetic")));
let all: i128 = ((numbers::big_shl(1i128, exponentBits))).checked_sub(1i128).expect("BigInt arithmetic");
if (exponent == all) {
{
if (fraction == 0i128) {
return numbers::big_shl(1i128, ((if (sign != 0i128) { 0i128 } else { 7i128 })));
}
return numbers::big_shl(1i128, ((if (((fraction & (numbers::big_shl(1i128, ((fractionBits).checked_sub(1i128).expect("BigInt arithmetic")))))) != 0i128) { 9i128 } else { 8i128 })));
}
}
if (exponent == 0i128) {
{
if (fraction == 0i128) {
return numbers::big_shl(1i128, ((if (sign != 0i128) { 3i128 } else { 4i128 })));
}
return numbers::big_shl(1i128, ((if (sign != 0i128) { 2i128 } else { 5i128 })));
}
}
return numbers::big_shl(1i128, ((if (sign != 0i128) { 1i128 } else { 6i128 })));
}

