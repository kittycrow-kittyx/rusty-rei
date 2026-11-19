use crate::*;

pub fn sx<P: Platform>(ctx: &Context<P>, mut value: i128, mut bits: f64) -> i128 {
return numbers::as_int_n(bits, value);
}


pub fn ux<P: Platform>(ctx: &Context<P>, mut value: i128) -> i128 {
return numbers::as_uint_n(64.0f64, value);
}


pub fn u32word<P: Platform>(ctx: &Context<P>, mut value: i128) -> i128 {
return numbers::as_uint_n(32.0f64, value);
}


pub fn word<P: Platform>(ctx: &Context<P>, mut value: f64) -> i128 {
return numbers::bigint(Numeric::Number(value));
}

