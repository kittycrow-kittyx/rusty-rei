use crate::*;

pub fn inputContains<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut size: f64) -> bool {
if (crate::m_native__memory::atomicLoadI32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_IMAGE_LOCK.expect("global initialisation"))) != 1.0f64) {
return false;
}
let start: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_IMAGE_TOP.expect("global initialisation")));
let length: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_IMAGE_SIZE.expect("global initialisation")));
return (((at >= start) && ((at - start) <= length)) && (size <= (length - ((at - start)))));
}

