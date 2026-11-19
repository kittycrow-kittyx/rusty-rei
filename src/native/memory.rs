use crate::*;

#[derive(Clone, Copy)]
pub struct TetoMemory {
    pub buffer: __bake_c_Buffer,
    pub bytes: __bake_c_U8Array,
    pub view: __bake_c_View,
    pub shared: bool,
}

pub fn directMemory<P: Platform>(ctx: &Context<P>, mut bytes: f64, mut shared: bool) -> crate::m_native__memory::TetoMemory {
if (((!{ let __crust_v0 = bytes; __crust_v0.is_finite() && numbers::trunc(__crust_v0) == __crust_v0 }) || (bytes < 65536.0f64)) || (bytes > 2147483647.0f64)) {
{
let _ = ctx.platform.__bake_c_abort_range();
}
}
let buffer: __bake_c_Buffer = (if shared { ctx.platform.__bake_c_new_shared_buffer(bytes) } else { ctx.platform.__bake_c_new_buffer(bytes) });
return crate::m_native__memory::TetoMemory { buffer: buffer, bytes: ctx.platform.__bake_c_new_u8_array(buffer), view: ctx.platform.__bake_c_new_view(buffer), shared: shared };
}


pub fn wasmMemory<P: Platform>(ctx: &Context<P>, mut memory: __bake_c_ExternalMemory, mut shared: bool) -> crate::m_native__memory::TetoMemory {
let buffer: __bake_c_Buffer = (memory).buffer;
return crate::m_native__memory::TetoMemory { buffer: buffer, bytes: ctx.platform.__bake_c_new_u8_array(buffer), view: ctx.platform.__bake_c_new_view(buffer), shared: shared };
}


pub fn memorySize<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory) -> f64 {
return ((memory).bytes).length;
}


pub fn range<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64, mut size: f64) -> () {
if (((((!{ let __crust_v1 = at; __crust_v1.is_finite() && numbers::trunc(__crust_v1) == __crust_v1 }) || (!{ let __crust_v2 = size; __crust_v2.is_finite() && numbers::trunc(__crust_v2) == __crust_v2 })) || (at < 0.0f64)) || (size < 0.0f64)) || ((at + size) > ((memory).bytes).length)) {
{
let _ = ctx.platform.__bake_c_abort_range();
}
}
}


pub fn fill<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64, mut size: f64, mut value: f64) -> () {
let _ = crate::m_native__memory::range(ctx, memory, at, size);
let _ = ctx.platform.__bake_c_u8_fill((memory).bytes, numbers::and(value, 255.0f64), Some(at), Some((at + size)));
}


pub fn copyMemory<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut to: f64, mut from: f64, mut size: f64) -> () {
let _ = crate::m_native__memory::range(ctx, memory, to, size);
let _ = crate::m_native__memory::range(ctx, memory, from, size);
let _ = ctx.platform.__bake_c_u8_copy_within((memory).bytes, to, from, Some((from + size)));
}


pub fn loadU8<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64) -> f64 {
let _ = crate::m_native__memory::range(ctx, memory, at, 1.0f64);
return ctx.platform.__bake_c_u8_get((memory).bytes, at);
}


pub fn loadI8<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64) -> i128 {
return numbers::as_int_n(8.0f64, numbers::bigint(Numeric::Number(crate::m_native__memory::loadU8(ctx, memory, at))));
}


pub fn loadU16<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64) -> f64 {
let _ = crate::m_native__memory::range(ctx, memory, at, 2.0f64);
return ctx.platform.__bake_c_view_get_u16((memory).view, at, Some(true));
}


pub fn loadI16<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64) -> i128 {
let _ = crate::m_native__memory::range(ctx, memory, at, 2.0f64);
return numbers::bigint(Numeric::Number(ctx.platform.__bake_c_view_get_i16((memory).view, at, Some(true))));
}


pub fn loadU32<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64) -> f64 {
let _ = crate::m_native__memory::range(ctx, memory, at, 4.0f64);
return ctx.platform.__bake_c_view_get_u32((memory).view, at, Some(true));
}


pub fn loadI32<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64) -> f64 {
let _ = crate::m_native__memory::range(ctx, memory, at, 4.0f64);
return ctx.platform.__bake_c_view_get_i32((memory).view, at, Some(true));
}


pub fn loadU64<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64) -> i128 {
let _ = crate::m_native__memory::range(ctx, memory, at, 8.0f64);
return ctx.platform.__bake_c_view_get_u64((memory).view, at, Some(true));
}


pub fn loadI64<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64) -> i128 {
let _ = crate::m_native__memory::range(ctx, memory, at, 8.0f64);
return ctx.platform.__bake_c_view_get_i64((memory).view, at, Some(true));
}


pub fn storeU8<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64, mut value: Numeric) -> () {
let _ = crate::m_native__memory::range(ctx, memory, at, 1.0f64);
let _ = ctx.platform.__bake_c_u8_set((memory).bytes, at, numbers::and(numbers::number(value), 255.0f64));
}


pub fn storeU16<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64, mut value: Numeric) -> () {
let _ = crate::m_native__memory::range(ctx, memory, at, 2.0f64);
let _ = ctx.platform.__bake_c_view_set_u16((memory).view, at, numbers::number(value), Some(true));
}


pub fn storeU32<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64, mut value: Numeric) -> () {
let _ = crate::m_native__memory::range(ctx, memory, at, 4.0f64);
let _ = ctx.platform.__bake_c_view_set_u32((memory).view, at, numbers::number(value), Some(true));
}


pub fn storeI32<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64, mut value: f64) -> () {
let _ = crate::m_native__memory::range(ctx, memory, at, 4.0f64);
let _ = ctx.platform.__bake_c_view_set_i32((memory).view, at, value, Some(true));
}


pub fn storeU64<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64, mut value: i128) -> () {
let _ = crate::m_native__memory::range(ctx, memory, at, 8.0f64);
let _ = ctx.platform.__bake_c_view_set_u64((memory).view, at, numbers::as_uint_n(64.0f64, value), Some(true));
}


pub fn storeI64<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64, mut value: i128) -> () {
let _ = crate::m_native__memory::range(ctx, memory, at, 8.0f64);
let _ = ctx.platform.__bake_c_view_set_i64((memory).view, at, numbers::as_int_n(64.0f64, value), Some(true));
}


pub fn words<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory) -> __bake_c_I32Array {
return ctx.platform.__bake_c_new_i32_array((memory).buffer);
}


pub fn compareExchangeI32<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64, mut expected: f64, mut replacement: f64) -> f64 {
let _ = crate::m_native__memory::range(ctx, memory, at, 4.0f64);
if numbers::truth_number(numbers::and(at, 3.0f64)) {
let _ = ctx.platform.__bake_c_abort_range();
}
let view: __bake_c_I32Array = crate::m_native__memory::words(ctx, memory);
let index: f64 = numbers::ushr(at, 2.0f64);
if (memory).shared {
return ctx.platform.__bake_c_atomic_cmpxchg_i32(view, index, expected, replacement);
}
let old: f64 = ctx.platform.__bake_c_i32_get(view, index);
if (old == expected) {
let _ = ctx.platform.__bake_c_i32_set(view, index, replacement);
}
return old;
}


pub fn atomicAddI32<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64, mut value: f64) -> f64 {
let _ = crate::m_native__memory::range(ctx, memory, at, 4.0f64);
if numbers::truth_number(numbers::and(at, 3.0f64)) {
let _ = ctx.platform.__bake_c_abort_range();
}
let view: __bake_c_I32Array = crate::m_native__memory::words(ctx, memory);
let index: f64 = numbers::ushr(at, 2.0f64);
if (memory).shared {
return ctx.platform.__bake_c_atomic_add_i32(view, index, value);
}
let old: f64 = ctx.platform.__bake_c_i32_get(view, index);
let _ = ctx.platform.__bake_c_i32_set(view, index, (old + value));
return old;
}


pub fn atomicStoreI32<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64, mut value: f64) -> () {
let _ = crate::m_native__memory::range(ctx, memory, at, 4.0f64);
if numbers::truth_number(numbers::and(at, 3.0f64)) {
let _ = ctx.platform.__bake_c_abort_range();
}
let view: __bake_c_I32Array = crate::m_native__memory::words(ctx, memory);
let index: f64 = numbers::ushr(at, 2.0f64);
if (memory).shared {
let _ = ctx.platform.__bake_c_atomic_store_i32(view, index, value);
}
else {
let _ = ctx.platform.__bake_c_i32_set(view, index, value);
}
}


pub fn atomicLoadI32<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64) -> f64 {
let _ = crate::m_native__memory::range(ctx, memory, at, 4.0f64);
if numbers::truth_number(numbers::and(at, 3.0f64)) {
let _ = ctx.platform.__bake_c_abort_range();
}
let view: __bake_c_I32Array = crate::m_native__memory::words(ctx, memory);
let index: f64 = numbers::ushr(at, 2.0f64);
return (if (memory).shared { ctx.platform.__bake_c_atomic_load_i32(view, index) } else { ctx.platform.__bake_c_i32_get(view, index) });
}


pub fn atomicLoadU64<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64) -> i128 {
let _ = crate::m_native__memory::range(ctx, memory, at, 8.0f64);
if numbers::truth_number(numbers::and(at, 7.0f64)) {
let _ = ctx.platform.__bake_c_abort_range();
}
if (memory).shared {
return ctx.platform.__bake_c_atomic_load_u64(ctx.platform.__bake_c_new_u64_array((memory).buffer), numbers::ushr(at, 3.0f64));
}
return ctx.platform.__bake_c_view_get_u64((memory).view, at, Some(true));
}


pub fn atomicStoreU64<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64, mut value: i128) -> () {
let _ = crate::m_native__memory::range(ctx, memory, at, 8.0f64);
if numbers::truth_number(numbers::and(at, 7.0f64)) {
let _ = ctx.platform.__bake_c_abort_range();
}
if (memory).shared {
let _ = ctx.platform.__bake_c_atomic_store_u64(ctx.platform.__bake_c_new_u64_array((memory).buffer), numbers::ushr(at, 3.0f64), crate::m_native__memory::ux64(ctx, value));
}
else {
let _ = ctx.platform.__bake_c_view_set_u64((memory).view, at, crate::m_native__memory::ux64(ctx, value), Some(true));
}
}


pub fn atomicAddU64<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64, mut value: i128) -> i128 {
let _ = crate::m_native__memory::range(ctx, memory, at, 8.0f64);
if numbers::truth_number(numbers::and(at, 7.0f64)) {
let _ = ctx.platform.__bake_c_abort_range();
}
if (memory).shared {
return ctx.platform.__bake_c_atomic_add_u64(ctx.platform.__bake_c_new_u64_array((memory).buffer), numbers::ushr(at, 3.0f64), crate::m_native__memory::ux64(ctx, value));
}
let old: i128 = ctx.platform.__bake_c_view_get_u64((memory).view, at, Some(true));
let _ = ctx.platform.__bake_c_view_set_u64((memory).view, at, crate::m_native__memory::ux64(ctx, (old).checked_add(value).expect("BigInt arithmetic")), Some(true));
return old;
}


pub fn ux64<P: Platform>(ctx: &Context<P>, mut value: i128) -> i128 {
return numbers::as_uint_n(64.0f64, value);
}

