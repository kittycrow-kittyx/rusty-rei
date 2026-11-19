use crate::*;

pub fn word<P: Platform>(ctx: &Context<P>, mut value: f64) -> i128 {
return numbers::bigint(Numeric::Number(value));
}


pub fn sx<P: Platform>(ctx: &Context<P>, mut value: i128, mut bits: f64) -> i128 {
return numbers::as_int_n(bits, value);
}


pub fn ux<P: Platform>(ctx: &Context<P>, mut value: i128) -> i128 {
return numbers::as_uint_n(64.0f64, value);
}


pub fn u32word<P: Platform>(ctx: &Context<P>, mut value: Numeric) -> i128 {
return numbers::as_uint_n(32.0f64, numbers::bigint(value));
}


pub fn mulU32<P: Platform>(ctx: &Context<P>, mut left: f64, mut right: f64) -> f64 {
return numbers::ushr(numbers::imul(left, right), 0.0f64);
}


pub fn wordToI32<P: Platform>(ctx: &Context<P>, mut value: i128) -> f64 {
return numbers::number(Numeric::BigInt(numbers::as_int_n(32.0f64, value)));
}


pub fn wordToU32<P: Platform>(ctx: &Context<P>, mut value: i128) -> f64 {
return numbers::number(Numeric::BigInt(numbers::as_uint_n(32.0f64, value)));
}


pub fn wordToPtr<P: Platform>(ctx: &Context<P>, mut value: i128) -> f64 {
let unsigned: i128 = crate::m_native__word::ux(ctx, value);
if (unsigned > 2147483647i128) {
return 4294967295.0f64;
}
return numbers::number(Numeric::BigInt(unsigned));
}


pub fn bitsToF32<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut scratch: f64, mut value: i128) -> f64 {
let _ = ctx.platform.__bake_c_view_set_u32((memory).view, scratch, numbers::number(Numeric::BigInt((value & ctx.globals.m_native__word__MASK32.expect("global initialisation")))), Some(true));
return ctx.platform.__bake_c_view_get_f32((memory).view, scratch, Some(true));
}


pub fn bitsToF64<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut scratch: f64, mut value: i128) -> f64 {
let _ = ctx.platform.__bake_c_view_set_u64((memory).view, scratch, crate::m_native__word::ux(ctx, value), Some(true));
return ctx.platform.__bake_c_view_get_f64((memory).view, scratch, Some(true));
}


pub fn f32ToBits<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut scratch: f64, mut value: f64) -> i128 {
let _ = ctx.platform.__bake_c_view_set_f32((memory).view, scratch, ((value) as f32 as f64), Some(true));
return numbers::bigint(Numeric::Number(ctx.platform.__bake_c_view_get_u32((memory).view, scratch, Some(true))));
}


pub fn f64ToBits<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut scratch: f64, mut value: f64) -> i128 {
let _ = ctx.platform.__bake_c_view_set_f64((memory).view, scratch, value, Some(true));
return ctx.platform.__bake_c_view_get_u64((memory).view, scratch, Some(true));
}


pub fn roundF32<P: Platform>(ctx: &Context<P>, mut value: f64) -> f64 {
return ((value) as f32 as f64);
}


pub fn floatIsNaN<P: Platform>(ctx: &Context<P>, mut value: f64) -> bool {
return (value).is_nan();
}


pub fn floatSqrt<P: Platform>(ctx: &Context<P>, mut value: f64) -> f64 {
return ctx.platform.__bake_c_f64_sqrt(value);
}


pub fn floatMin<P: Platform>(ctx: &Context<P>, mut left: f64, mut right: f64) -> f64 {
return numbers::minimum(left, right);
}


pub fn floatMax<P: Platform>(ctx: &Context<P>, mut left: f64, mut right: f64) -> f64 {
return numbers::maximum(left, right);
}


pub fn floatTrunc<P: Platform>(ctx: &Context<P>, mut value: f64) -> f64 {
return numbers::trunc(value);
}


pub fn floatFloor<P: Platform>(ctx: &Context<P>, mut value: f64) -> f64 {
return numbers::floor(value);
}


pub fn floatCeil<P: Platform>(ctx: &Context<P>, mut value: f64) -> f64 {
return numbers::ceil(value);
}


pub fn floatToI64<P: Platform>(ctx: &Context<P>, mut value: f64) -> i128 {
return numbers::bigint(Numeric::Number(value));
}


pub fn floatToU64<P: Platform>(ctx: &Context<P>, mut value: f64) -> i128 {
return numbers::bigint(Numeric::Number(value));
}


pub fn wordToFloat<P: Platform>(ctx: &Context<P>, mut value: i128) -> f64 {
return numbers::number(Numeric::BigInt(value));
}


pub fn unsignedWordToFloat<P: Platform>(ctx: &Context<P>, mut value: i128) -> f64 {
return numbers::number(Numeric::BigInt(value));
}

