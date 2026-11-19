use crate::*;

pub fn startupEntry<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64, mut end: f64) -> f64 {
if ((end - at) < 4.0f64) {
return ctx.globals.m_native__start__START_BAD.expect("global initialisation");
}
let length: f64 = crate::m_native__memory::loadU32(ctx, memory, at);
if (length > ((end - at) - 4.0f64)) {
return ctx.globals.m_native__start__START_BAD.expect("global initialisation");
}
let mut index: f64 = 0.0f64;
'__crust_v56: while (index < length) {
{
if (crate::m_native__memory::loadU8(ctx, memory, ((at + 4.0f64) + index)) == 0.0f64) {
return ctx.globals.m_native__start__START_BAD.expect("global initialisation");
}
let _ = { index = (index + 1.0f64); index };
}
}
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__TETO_PARSER_NUMBER.expect("global initialisation")), Numeric::Number(length));
return ((at + 4.0f64) + length);
}


pub fn tetoBuildInitialStack<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut startup: f64, mut startupLength: f64, mut stackBytes: f64) -> f64 {
if (!crate::m_native__kernel::tetoImageContains(ctx, memory, startup, startupLength)) {
return ctx.globals.m_native__abi__TETO_START_RANGE.expect("global initialisation");
}
if ((startupLength < 32.0f64) || (crate::m_native__memory::loadU32(ctx, memory, startup) != ctx.globals.m_native__abi__TETO_STARTUP_MAGIC.expect("global initialisation"))) {
return ctx.globals.m_native__abi__TETO_START_FORMAT.expect("global initialisation");
}
let argc: f64 = crate::m_native__memory::loadU32(ctx, memory, (startup + 4.0f64));
let envc: f64 = crate::m_native__memory::loadU32(ctx, memory, (startup + 8.0f64));
if ((((argc < 1.0f64) || (argc > 4096.0f64)) || (envc > 8192.0f64)) || (argc > (4294967295.0f64 - envc))) {
return ctx.globals.m_native__abi__TETO_START_FORMAT.expect("global initialisation");
}
let entries: f64 = (argc + envc);
let end: f64 = (startup + startupLength);
let mut cursor: f64 = (startup + 32.0f64);
let mut totalStrings: i128 = 0i128;
let mut index: f64 = 0.0f64;
'__crust_v57: while (index < entries) {
{
let next: f64 = crate::m_native__start::startupEntry(ctx, memory, cursor, end);
if (next == ctx.globals.m_native__start__START_BAD.expect("global initialisation")) {
return ctx.globals.m_native__abi__TETO_START_FORMAT.expect("global initialisation");
}
let _ = { totalStrings = (totalStrings).checked_add((crate::m_native__word::u32word(ctx, Numeric::Number(crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__TETO_PARSER_NUMBER.expect("global initialisation")))))).checked_add(1i128).expect("BigInt arithmetic")).expect("BigInt arithmetic"); totalStrings };
if (totalStrings > 2147483647i128) {
return ctx.globals.m_native__abi__TETO_START_RANGE.expect("global initialisation");
}
let _ = { cursor = next; cursor };
let _ = { index = (index + 1.0f64); index };
}
}
if (cursor != end) {
return ctx.globals.m_native__abi__TETO_START_FORMAT.expect("global initialisation");
}
let top: i128 = crate::m_native__kernel::tetoHartVirtualTop(ctx, memory, hart);
let limit: f64 = (if (stackBytes < (1024.0f64 * 1024.0f64)) { (1024.0f64 * 1024.0f64) } else { stackBytes });
if ((top <= crate::m_native__word::u32word(ctx, Numeric::Number(limit))) || (top < (32i128).checked_add(totalStrings).expect("BigInt arithmetic"))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let bottom: i128 = (top).checked_sub(crate::m_native__word::u32word(ctx, Numeric::Number(limit))).expect("BigInt arithmetic");
let mut stringPointer: i128 = ((top).checked_sub(16i128).expect("BigInt arithmetic")).checked_sub(totalStrings).expect("BigInt arithmetic");
if (stringPointer < 16i128) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let random: i128 = (stringPointer).checked_sub(16i128).expect("BigInt arithmetic");
let aligned: i128 = (random & (16i128).checked_neg().expect("BigInt negation"));
let words: f64 = ((argc + envc) + 33.0f64);
if (words > 268435455.0f64) {
return ctx.globals.m_native__abi__TETO_START_RANGE.expect("global initialisation");
}
let wordBytes: i128 = crate::m_native__word::u32word(ctx, Numeric::Number((words * 8.0f64)));
if (aligned < wordBytes) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let stackPointer: i128 = ((aligned).checked_sub(wordBytes).expect("BigInt arithmetic") & (16i128).checked_neg().expect("BigInt negation"));
let floor: i128 = crate::m_native__kernel::tetoHartImageFloor(ctx, memory, hart);
let heapGuard: i128 = ((((floor).checked_add(4095i128).expect("BigInt arithmetic") & (4096i128).checked_neg().expect("BigInt negation")))).checked_add((1024i128).checked_mul(1024i128).expect("BigInt arithmetic")).expect("BigInt arithmetic");
if ((stackPointer < bottom) || (stackPointer < heapGuard)) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let mut result: f64 = crate::m_native__kernel::tetoStackPrepare(ctx, memory, hart, bottom, stackPointer);
if (result != ctx.globals.m_native__abi__TETO_START_OK.expect("global initialisation")) {
return result;
}
let argvTable: i128 = (stackPointer).checked_add(8i128).expect("BigInt arithmetic");
let envTable: i128 = ((argvTable).checked_add((crate::m_native__word::u32word(ctx, Numeric::Number(argc))).checked_mul(8i128).expect("BigInt arithmetic")).expect("BigInt arithmetic")).checked_add(8i128).expect("BigInt arithmetic");
let auxTable: i128 = ((envTable).checked_add((crate::m_native__word::u32word(ctx, Numeric::Number(envc))).checked_mul(8i128).expect("BigInt arithmetic")).expect("BigInt arithmetic")).checked_add(8i128).expect("BigInt arithmetic");
let _ = { result = crate::m_native__kernel::tetoStackStoreU64(ctx, memory, hart, stackPointer, crate::m_native__word::u32word(ctx, Numeric::Number(argc))); result };
if (result != ctx.globals.m_native__abi__TETO_START_OK.expect("global initialisation")) {
return result;
}
let _ = { result = crate::m_native__kernel::tetoStackStoreU64(ctx, memory, hart, (argvTable).checked_add((crate::m_native__word::u32word(ctx, Numeric::Number(argc))).checked_mul(8i128).expect("BigInt arithmetic")).expect("BigInt arithmetic"), 0i128); result };
if (result != ctx.globals.m_native__abi__TETO_START_OK.expect("global initialisation")) {
return result;
}
let _ = { result = crate::m_native__kernel::tetoStackStoreU64(ctx, memory, hart, (envTable).checked_add((crate::m_native__word::u32word(ctx, Numeric::Number(envc))).checked_mul(8i128).expect("BigInt arithmetic")).expect("BigInt arithmetic"), 0i128); result };
if (result != ctx.globals.m_native__abi__TETO_START_OK.expect("global initialisation")) {
return result;
}
let _ = { cursor = (startup + 32.0f64); cursor };
let _ = { stringPointer = (top).checked_sub(16i128).expect("BigInt arithmetic"); stringPointer };
let _ = { index = 0.0f64; index };
'__crust_v58: while (index < envc) {
{
let next: f64 = crate::m_native__start::startupEntry(ctx, memory, cursor, end);
if (next == ctx.globals.m_native__start__START_BAD.expect("global initialisation")) {
return ctx.globals.m_native__abi__TETO_START_FORMAT.expect("global initialisation");
}
let length: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__TETO_PARSER_NUMBER.expect("global initialisation")));
let _ = { stringPointer = (stringPointer).checked_sub((crate::m_native__word::u32word(ctx, Numeric::Number(length))).checked_add(1i128).expect("BigInt arithmetic")).expect("BigInt arithmetic"); stringPointer };
let _ = { result = crate::m_native__kernel::tetoStackCopy(ctx, memory, hart, stringPointer, (cursor + 4.0f64), length); result };
if (result != ctx.globals.m_native__abi__TETO_START_OK.expect("global initialisation")) {
return result;
}
let _ = { result = crate::m_native__kernel::tetoStackStoreU64(ctx, memory, hart, (envTable).checked_add((crate::m_native__word::u32word(ctx, Numeric::Number(index))).checked_mul(8i128).expect("BigInt arithmetic")).expect("BigInt arithmetic"), stringPointer); result };
if (result != ctx.globals.m_native__abi__TETO_START_OK.expect("global initialisation")) {
return result;
}
let _ = { cursor = next; cursor };
let _ = { index = (index + 1.0f64); index };
}
}
let mut exec: i128 = 0i128;
let _ = { index = 0.0f64; index };
'__crust_v59: while (index < argc) {
{
let next: f64 = crate::m_native__start::startupEntry(ctx, memory, cursor, end);
if (next == ctx.globals.m_native__start__START_BAD.expect("global initialisation")) {
return ctx.globals.m_native__abi__TETO_START_FORMAT.expect("global initialisation");
}
let length: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__TETO_PARSER_NUMBER.expect("global initialisation")));
let _ = { stringPointer = (stringPointer).checked_sub((crate::m_native__word::u32word(ctx, Numeric::Number(length))).checked_add(1i128).expect("BigInt arithmetic")).expect("BigInt arithmetic"); stringPointer };
let _ = { result = crate::m_native__kernel::tetoStackCopy(ctx, memory, hart, stringPointer, (cursor + 4.0f64), length); result };
if (result != ctx.globals.m_native__abi__TETO_START_OK.expect("global initialisation")) {
return result;
}
let _ = { result = crate::m_native__kernel::tetoStackStoreU64(ctx, memory, hart, (argvTable).checked_add((crate::m_native__word::u32word(ctx, Numeric::Number(index))).checked_mul(8i128).expect("BigInt arithmetic")).expect("BigInt arithmetic"), stringPointer); result };
if (result != ctx.globals.m_native__abi__TETO_START_OK.expect("global initialisation")) {
return result;
}
if (index == 0.0f64) {
let _ = { exec = stringPointer; exec };
}
let _ = { cursor = next; cursor };
let _ = { index = (index + 1.0f64); index };
}
}
if ((stringPointer != (random).checked_add(16i128).expect("BigInt arithmetic")) || (exec == 0i128)) {
return ctx.globals.m_native__abi__TETO_START_FORMAT.expect("global initialisation");
}
let _ = { result = crate::m_native__kernel::tetoStackCopy(ctx, memory, hart, random, (startup + 16.0f64), 16.0f64); result };
if (result != ctx.globals.m_native__abi__TETO_START_OK.expect("global initialisation")) {
return result;
}
let _ = { result = crate::m_native__kernel::tetoStackAux(ctx, memory, hart, auxTable, random, exec); result };
if (result != ctx.globals.m_native__abi__TETO_START_OK.expect("global initialisation")) {
return result;
}
return crate::m_native__kernel::tetoStackFinish(ctx, memory, hart, stackPointer);
}

