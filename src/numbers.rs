#[derive(Clone, Copy)]
pub enum Numeric { Number(f64), BigInt(i128) }

pub fn truth_number(value: f64) -> bool { value != 0.0 && !value.is_nan() }
pub fn truth_bigint(value: i128) -> bool { value != 0 }

pub fn trunc(value: f64) -> f64 {
    let bits = value.to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as i32 - 1023;
    if exponent < 0 { return f64::from_bits(bits & (1u64 << 63)); }
    if exponent >= 52 { return value; }
    f64::from_bits(bits & !((1u64 << (52 - exponent)) - 1))
}
pub fn floor(value: f64) -> f64 {
    let integral = trunc(value);
    if value < integral { integral - 1.0 } else { integral }
}
pub fn ceil(value: f64) -> f64 {
    let integral = trunc(value);
    if value > integral { integral + 1.0 } else { integral }
}
pub fn minimum(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() { return f64::NAN; }
    if a < b || (a == b && a.is_sign_negative()) { a } else { b }
}
pub fn maximum(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() { return f64::NAN; }
    if a > b || (a == b && a.is_sign_positive()) { a } else { b }
}
pub fn to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 { return 0; }
    let reduced = trunc(value) % 4294967296.0;
    (if reduced < 0.0 { reduced + 4294967296.0 } else { reduced }) as u32
}
pub fn to_i32(value: f64) -> i32 { to_u32(value) as i32 }
pub fn and(a: f64, b: f64) -> f64 { (to_i32(a) & to_i32(b)) as f64 }
pub fn or(a: f64, b: f64) -> f64 { (to_i32(a) | to_i32(b)) as f64 }
pub fn xor(a: f64, b: f64) -> f64 { (to_i32(a) ^ to_i32(b)) as f64 }
pub fn not(a: f64) -> f64 { (!to_i32(a)) as f64 }
pub fn shl(a: f64, b: f64) -> f64 { to_i32(a).wrapping_shl(to_u32(b) & 31) as f64 }
pub fn shr(a: f64, b: f64) -> f64 { (to_i32(a) >> (to_u32(b) & 31)) as f64 }
pub fn ushr(a: f64, b: f64) -> f64 { (to_u32(a) >> (to_u32(b) & 31)) as f64 }
pub fn imul(a: f64, b: f64) -> f64 { to_i32(a).wrapping_mul(to_i32(b)) as f64 }

pub fn compare_number_bigint(a: f64, b: i128) -> Option<core::cmp::Ordering> {
    use core::cmp::Ordering;
    if a.is_nan() { return None; }
    if a >= -(i128::MIN as f64) { return Some(Ordering::Greater); }
    if a < i128::MIN as f64 { return Some(Ordering::Less); }
    let integral = trunc(a);
    match (integral as i128).cmp(&b) {
        Ordering::Equal if a < integral => Some(Ordering::Less),
        Ordering::Equal if a > integral => Some(Ordering::Greater),
        other => Some(other),
    }
}
pub fn bigint(value: Numeric) -> i128 {
    match value {
        Numeric::BigInt(value) => value,
        Numeric::Number(value) => {
            assert!(value.is_finite() && trunc(value) == value);
            assert!(value >= i128::MIN as f64 && value < -(i128::MIN as f64));
            value as i128
        }
    }
}
pub fn number(value: Numeric) -> f64 {
    match value { Numeric::BigInt(v) => v as f64, Numeric::Number(v) => v }
}
fn width(bits: f64) -> u32 {
    assert!(bits >= 0.0 && bits <= 127.0 && trunc(bits) == bits);
    bits as u32
}
pub fn as_uint_n(bits: f64, value: i128) -> i128 {
    let bits = width(bits);
    if bits == 0 { 0 } else { value & (i128::MAX >> (127 - bits)) }
}
pub fn as_int_n(bits: f64, value: i128) -> i128 {
    let bits = width(bits);
    if bits == 0 { 0 } else {
        let shift = 128 - bits;
        (value << shift) >> shift
    }
}
pub fn big_shl(value: i128, amount: i128) -> i128 {
    if amount < 0 { return big_shr(value, amount.checked_neg().expect("BigInt shift")); }
    if value == 0 { return 0; }
    assert!(amount < 128, "BigInt exceeds Crust's bounded representation");
    let result = value << (amount as u32);
    assert!(result >> (amount as u32) == value, "BigInt overflow");
    result
}
pub fn big_shr(value: i128, amount: i128) -> i128 {
    if amount < 0 { return big_shl(value, amount.checked_neg().expect("BigInt shift")); }
    if amount >= 128 { if value < 0 { -1 } else { 0 } } else { value >> (amount as u32) }
}
