use crate::*;

pub fn whitespace<P: Platform>(ctx: &Context<P>, mut byte: f64) -> bool {
return ((((byte == 32.0f64) || (byte == 9.0f64)) || (byte == 10.0f64)) || (byte == 13.0f64));
}


pub fn digit<P: Platform>(ctx: &Context<P>, mut byte: f64) -> bool {
return ((byte >= 48.0f64) && (byte <= 57.0f64));
}


pub fn hex<P: Platform>(ctx: &Context<P>, mut byte: f64) -> bool {
return ((crate::m_native__thx::digit(ctx, byte) || ((byte >= 65.0f64) && (byte <= 70.0f64))) || ((byte >= 97.0f64) && (byte <= 102.0f64)));
}


pub fn space<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut end: f64) -> f64 {
let mut cursor: f64 = at;
'__crust_v63: while ((cursor < end) && crate::m_native__thx::whitespace(ctx, crate::m_native__memory::loadU8(ctx, memory, cursor))) {
let _ = { cursor = (cursor + 1.0f64); cursor };
}
return cursor;
}


pub fn skipEscape<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut end: f64) -> f64 {
if (at >= end) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
let code: f64 = crate::m_native__memory::loadU8(ctx, memory, at);
if ((code == 117.0f64) && (((((((end - at) < 5.0f64) || (!crate::m_native__thx::hex(ctx, crate::m_native__memory::loadU8(ctx, memory, (at + 1.0f64))))) || (!crate::m_native__thx::hex(ctx, crate::m_native__memory::loadU8(ctx, memory, (at + 2.0f64))))) || (!crate::m_native__thx::hex(ctx, crate::m_native__memory::loadU8(ctx, memory, (at + 3.0f64))))) || (!crate::m_native__thx::hex(ctx, crate::m_native__memory::loadU8(ctx, memory, (at + 4.0f64))))))) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
if (code == 117.0f64) {
return (at + 5.0f64);
}
if ((((((((code != 34.0f64) && (code != 92.0f64)) && (code != 47.0f64)) && (code != 98.0f64)) && (code != 102.0f64)) && (code != 110.0f64)) && (code != 114.0f64)) && (code != 116.0f64)) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
return (at + 1.0f64);
}


pub fn parseString<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut end: f64) -> f64 {
if ((at >= end) || (crate::m_native__memory::loadU8(ctx, memory, at) != 34.0f64)) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
let start: f64 = (at + 1.0f64);
let mut cursor: f64 = start;
let mut escaped: f64 = 0.0f64;
'__crust_v64: while (cursor < end) {
{
let byte: f64 = crate::m_native__memory::loadU8(ctx, memory, cursor);
if (byte == 34.0f64) {
{
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING.expect("global initialisation")), Numeric::Number(start));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING_LENGTH.expect("global initialisation")), Numeric::Number((cursor - start)));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING_ESCAPED.expect("global initialisation")), Numeric::Number(escaped));
return (cursor + 1.0f64);
}
}
if (byte < 32.0f64) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
if (byte == 92.0f64) {
{
let _ = { escaped = 1.0f64; escaped };
let _ = { cursor = crate::m_native__thx::skipEscape(ctx, memory, (cursor + 1.0f64), end); cursor };
continue '__crust_v64;
}
}
let _ = { cursor = (cursor + 1.0f64); cursor };
}
}
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}


pub fn byte<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut index: f64, mut expected: f64) -> bool {
return (crate::m_native__memory::loadU8(ctx, memory, (at + index)) == expected);
}


pub fn keyLength3<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64) -> f64 {
if ((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 118.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 101.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 114.0f64)) {
return ctx.globals.m_native__thx__K_VER.expect("global initialisation");
}
if ((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 115.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 101.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 99.0f64)) {
return ctx.globals.m_native__thx__K_SEC.expect("global initialisation");
}
if ((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 115.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 121.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 109.0f64)) {
return ctx.globals.m_native__thx__K_SYM.expect("global initialisation");
}
if ((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 114.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 101.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 108.0f64)) {
return ctx.globals.m_native__thx__K_REL.expect("global initialisation");
}
if ((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 100.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 98.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 103.0f64)) {
return ctx.globals.m_native__thx__K_DBG.expect("global initialisation");
}
if ((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 109.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 101.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 109.0f64)) {
return ctx.globals.m_native__thx__K_MEM.expect("global initialisation");
}
if ((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 105.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 115.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 97.0f64)) {
return ctx.globals.m_native__thx__K_ISA.expect("global initialisation");
}
if ((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 102.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 108.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 103.0f64)) {
return ctx.globals.m_native__thx__K_FLG.expect("global initialisation");
}
if ((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 108.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 101.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 110.0f64)) {
return ctx.globals.m_native__thx__K_LEN.expect("global initialisation");
}
return 0.0f64;
}


pub fn keyLength4<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64) -> f64 {
if (((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 110.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 97.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 109.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 3.0f64, 101.0f64)) {
return ctx.globals.m_native__thx__K_NAME.expect("global initialisation");
}
if (((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 115.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 105.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 122.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 3.0f64, 101.0f64)) {
return ctx.globals.m_native__thx__K_SIZE.expect("global initialisation");
}
if (((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 97.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 100.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 100.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 3.0f64, 114.0f64)) {
return ctx.globals.m_native__thx__K_ADDR.expect("global initialisation");
}
if (((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 112.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 104.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 100.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 3.0f64, 114.0f64)) {
return ctx.globals.m_native__thx__K_PHDR.expect("global initialisation");
}
return 0.0f64;
}


pub fn keyLength5<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64) -> f64 {
if ((((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 105.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 100.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 101.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 3.0f64, 110.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 4.0f64, 116.0f64)) {
return ctx.globals.m_native__thx__K_IDENT.expect("global initialisation");
}
if ((((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 101.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 110.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 116.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 3.0f64, 114.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 4.0f64, 121.0f64)) {
return ctx.globals.m_native__thx__K_ENTRY.expect("global initialisation");
}
if ((((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 112.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 104.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 101.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 3.0f64, 110.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 4.0f64, 116.0f64)) {
return ctx.globals.m_native__thx__K_PHENT.expect("global initialisation");
}
if ((((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 112.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 104.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 110.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 3.0f64, 117.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 4.0f64, 109.0f64)) {
return ctx.globals.m_native__thx__K_PHNUM.expect("global initialisation");
}
if ((((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 97.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 108.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 105.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 3.0f64, 103.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 4.0f64, 110.0f64)) {
return ctx.globals.m_native__thx__K_ALIGN.expect("global initialisation");
}
return 0.0f64;
}


pub fn keyId<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory) -> f64 {
if (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING_ESCAPED.expect("global initialisation"))) != 0.0f64) {
return 0.0f64;
}
let at: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING.expect("global initialisation")));
let length: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING_LENGTH.expect("global initialisation")));
if (((length == 2.0f64) && crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 97.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 116.0f64)) {
return ctx.globals.m_native__thx__K_AT.expect("global initialisation");
}
if (length == 3.0f64) {
return crate::m_native__thx::keyLength3(ctx, memory, at);
}
if (length == 4.0f64) {
return crate::m_native__thx::keyLength4(ctx, memory, at);
}
if (length == 5.0f64) {
return crate::m_native__thx::keyLength5(ctx, memory, at);
}
if ((((((((length == 7.0f64) && crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 109.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 97.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 99.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 3.0f64, 104.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 4.0f64, 105.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 5.0f64, 110.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 6.0f64, 101.0f64)) {
return ctx.globals.m_native__thx__K_MACHINE.expect("global initialisation");
}
return 0.0f64;
}


pub fn exactMachine<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory) -> bool {
if ((crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING_ESCAPED.expect("global initialisation"))) != 0.0f64) || (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING_LENGTH.expect("global initialisation"))) != 9.0f64)) {
return false;
}
let at: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING.expect("global initialisation")));
return ((((((((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 116.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 104.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 105.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 3.0f64, 115.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 4.0f64, 116.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 5.0f64, 108.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 6.0f64, 101.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 7.0f64, 54.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 8.0f64, 52.0f64));
}


pub fn exactIsa<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory) -> bool {
if ((crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING_ESCAPED.expect("global initialisation"))) != 0.0f64) || (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING_LENGTH.expect("global initialisation"))) != 6.0f64)) {
return false;
}
let at: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING.expect("global initialisation")));
return (((((crate::m_native__thx::byte(ctx, memory, at, 0.0f64, 114.0f64) && crate::m_native__thx::byte(ctx, memory, at, 1.0f64, 118.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 2.0f64, 54.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 3.0f64, 52.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 4.0f64, 103.0f64)) && crate::m_native__thx::byte(ctx, memory, at, 5.0f64, 99.0f64));
}


pub fn parseUnsigned<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut end: f64) -> f64 {
if ((at >= end) || (!crate::m_native__thx::digit(ctx, crate::m_native__memory::loadU8(ctx, memory, at)))) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
let first: f64 = crate::m_native__memory::loadU8(ctx, memory, at);
let mut cursor: f64 = at;
let mut value: i128 = 0i128;
'__crust_v65: while ((cursor < end) && crate::m_native__thx::digit(ctx, crate::m_native__memory::loadU8(ctx, memory, cursor))) {
{
if ((first == 48.0f64) && (cursor != at)) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
let _ = { value = ((value).checked_mul(10i128).expect("BigInt arithmetic")).checked_add(crate::m_native__word::u32word(ctx, Numeric::Number((crate::m_native__memory::loadU8(ctx, memory, cursor) - 48.0f64)))).expect("BigInt arithmetic"); value };
if (value > ctx.globals.m_native__thx__MAX_SAFE.expect("global initialisation")) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
let _ = { cursor = (cursor + 1.0f64); cursor };
}
}
let _ = crate::m_native__memory::storeU64(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_NUMBER.expect("global initialisation")), value);
return cursor;
}


pub fn literal<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut end: f64, mut value: f64) -> f64 {
if (((end - at) < 4.0f64) || (crate::m_native__memory::loadU32(ctx, memory, at) != value)) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
return (at + 4.0f64);
}


pub fn skipDigits<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut end: f64) -> f64 {
let mut cursor: f64 = at;
if ((cursor >= end) || (!crate::m_native__thx::digit(ctx, crate::m_native__memory::loadU8(ctx, memory, cursor)))) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
'__crust_v66: while ((cursor < end) && crate::m_native__thx::digit(ctx, crate::m_native__memory::loadU8(ctx, memory, cursor))) {
let _ = { cursor = (cursor + 1.0f64); cursor };
}
return cursor;
}


pub fn skipExponent<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut end: f64) -> f64 {
let mut cursor: f64 = at;
if ((cursor < end) && (((crate::m_native__memory::loadU8(ctx, memory, cursor) == 43.0f64) || (crate::m_native__memory::loadU8(ctx, memory, cursor) == 45.0f64)))) {
let _ = { cursor = (cursor + 1.0f64); cursor };
}
return crate::m_native__thx::skipDigits(ctx, memory, cursor, end);
}


pub fn skipNumber<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut end: f64) -> f64 {
let mut cursor: f64 = at;
if ((cursor < end) && (crate::m_native__memory::loadU8(ctx, memory, cursor) == 45.0f64)) {
let _ = { cursor = (cursor + 1.0f64); cursor };
}
if ((cursor >= end) || (!crate::m_native__thx::digit(ctx, crate::m_native__memory::loadU8(ctx, memory, cursor)))) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
if (crate::m_native__memory::loadU8(ctx, memory, cursor) == 48.0f64) {
let _ = { cursor = (cursor + 1.0f64); cursor };
}
else {
let _ = { cursor = crate::m_native__thx::skipDigits(ctx, memory, cursor, end); cursor };
}
if ((cursor < end) && (crate::m_native__memory::loadU8(ctx, memory, cursor) == 46.0f64)) {
let _ = { cursor = crate::m_native__thx::skipDigits(ctx, memory, (cursor + 1.0f64), end); cursor };
}
if ((cursor < end) && (((crate::m_native__memory::loadU8(ctx, memory, cursor) == 101.0f64) || (crate::m_native__memory::loadU8(ctx, memory, cursor) == 69.0f64)))) {
let _ = { cursor = crate::m_native__thx::skipExponent(ctx, memory, (cursor + 1.0f64), end); cursor };
}
return cursor;
}


pub fn skipArray<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut end: f64, mut depth: f64) -> f64 {
let mut cursor: f64 = crate::m_native__thx::space(ctx, memory, (at + 1.0f64), end);
if ((cursor < end) && (crate::m_native__memory::loadU8(ctx, memory, cursor) == 93.0f64)) {
return (cursor + 1.0f64);
}
'__crust_v67: while (cursor < end) {
{
let _ = { cursor = crate::m_native__thx::skipValue(ctx, memory, cursor, end, (depth + 1.0f64)); cursor };
if (cursor == ctx.globals.m_native__thx__BAD.expect("global initialisation")) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
let _ = { cursor = crate::m_native__thx::space(ctx, memory, cursor, end); cursor };
if ((cursor < end) && (crate::m_native__memory::loadU8(ctx, memory, cursor) == 93.0f64)) {
return (cursor + 1.0f64);
}
if ((cursor >= end) || (crate::m_native__memory::loadU8(ctx, memory, cursor) != 44.0f64)) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
let _ = { cursor = crate::m_native__thx::space(ctx, memory, (cursor + 1.0f64), end); cursor };
}
}
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}


pub fn skipObject<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut end: f64, mut depth: f64) -> f64 {
let mut cursor: f64 = crate::m_native__thx::space(ctx, memory, (at + 1.0f64), end);
if ((cursor < end) && (crate::m_native__memory::loadU8(ctx, memory, cursor) == 125.0f64)) {
return (cursor + 1.0f64);
}
'__crust_v68: while (cursor < end) {
{
let _ = { cursor = crate::m_native__thx::parseString(ctx, memory, cursor, end); cursor };
if (cursor == ctx.globals.m_native__thx__BAD.expect("global initialisation")) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
let _ = { cursor = crate::m_native__thx::space(ctx, memory, cursor, end); cursor };
if ((cursor >= end) || (crate::m_native__memory::loadU8(ctx, memory, cursor) != 58.0f64)) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
let _ = { cursor = crate::m_native__thx::skipValue(ctx, memory, (cursor + 1.0f64), end, (depth + 1.0f64)); cursor };
if (cursor == ctx.globals.m_native__thx__BAD.expect("global initialisation")) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
let _ = { cursor = crate::m_native__thx::space(ctx, memory, cursor, end); cursor };
if ((cursor < end) && (crate::m_native__memory::loadU8(ctx, memory, cursor) == 125.0f64)) {
return (cursor + 1.0f64);
}
if ((cursor >= end) || (crate::m_native__memory::loadU8(ctx, memory, cursor) != 44.0f64)) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
let _ = { cursor = crate::m_native__thx::space(ctx, memory, (cursor + 1.0f64), end); cursor };
}
}
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}


pub fn skipFalse<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut end: f64) -> f64 {
if ((((end - at) < 5.0f64) || (crate::m_native__memory::loadU32(ctx, memory, at) != 1936482662.0f64)) || (crate::m_native__memory::loadU8(ctx, memory, (at + 4.0f64)) != 101.0f64)) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
return (at + 5.0f64);
}


pub fn skipValue<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut end: f64, mut depth: f64) -> f64 {
if (depth > 32.0f64) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
let cursor: f64 = crate::m_native__thx::space(ctx, memory, at, end);
if (cursor >= end) {
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}
let first: f64 = crate::m_native__memory::loadU8(ctx, memory, cursor);
if (first == 34.0f64) {
return crate::m_native__thx::parseString(ctx, memory, cursor, end);
}
if (first == 116.0f64) {
return crate::m_native__thx::literal(ctx, memory, cursor, end, 1702195828.0f64);
}
if (first == 102.0f64) {
return crate::m_native__thx::skipFalse(ctx, memory, cursor, end);
}
if (first == 110.0f64) {
return crate::m_native__thx::literal(ctx, memory, cursor, end, 1819047278.0f64);
}
if ((first == 45.0f64) || crate::m_native__thx::digit(ctx, first)) {
return crate::m_native__thx::skipNumber(ctx, memory, cursor, end);
}
if (first == 91.0f64) {
return crate::m_native__thx::skipArray(ctx, memory, cursor, end, depth);
}
if (first == 123.0f64) {
return crate::m_native__thx::skipObject(ctx, memory, cursor, end, depth);
}
return ctx.globals.m_native__thx__BAD.expect("global initialisation");
}


pub fn nameHash<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory) -> f64 {
let at: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING.expect("global initialisation")));
let length: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING_LENGTH.expect("global initialisation")));
let mut hash: f64 = 2166136261.0f64;
let mut index: f64 = 0.0f64;
'__crust_v69: while (index < length) {
{
let _ = { hash = crate::m_native__word::mulU32(ctx, numbers::xor(hash, crate::m_native__memory::loadU8(ctx, memory, (at + index))), 16777619.0f64); hash };
let _ = { index = (index + 1.0f64); index };
}
}
return hash;
}


pub fn sectionFlags<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory) -> f64 {
if (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING_ESCAPED.expect("global initialisation"))) != 0.0f64) {
return 4294967295.0f64;
}
let at: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING.expect("global initialisation")));
let length: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING_LENGTH.expect("global initialisation")));
let mut flags: f64 = 0.0f64;
let mut index: f64 = 0.0f64;
'__crust_v70: while (index < length) {
{
let value: f64 = crate::m_native__memory::loadU8(ctx, memory, (at + index));
let flag: f64 = (if (value == 114.0f64) { ctx.globals.m_native__abi__THISTLE_SEGMENT_READ.expect("global initialisation") } else { (if (value == 119.0f64) { ctx.globals.m_native__abi__THISTLE_SEGMENT_WRITE.expect("global initialisation") } else { (if (value == 120.0f64) { ctx.globals.m_native__abi__THISTLE_SEGMENT_EXECUTE.expect("global initialisation") } else { 0.0f64 }) }) });
if (flag == 0.0f64) {
return 4294967295.0f64;
}
if ((numbers::and(flags, flag)) != 0.0f64) {
return 4294967295.0f64;
}
let _ = { flags = numbers::or(flags, flag); flags };
let _ = { index = (index + 1.0f64); index };
}
}
return flags;
}


pub fn parseSections<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut hart: f64, mut at: f64, mut end: f64, mut payload: f64, mut payloadLength: f64) -> f64 {
let mut cursor: f64 = crate::m_native__thx::space(ctx, memory, at, end);
if ((cursor >= end) || (crate::m_native__memory::loadU8(ctx, memory, cursor) != 91.0f64)) {
return ctx.globals.m_native__abi__THISTLE_THX_SECTION.expect("global initialisation");
}
let _ = { cursor = crate::m_native__thx::space(ctx, memory, (cursor + 1.0f64), end); cursor };
if ((cursor < end) && (crate::m_native__memory::loadU8(ctx, memory, cursor) == 93.0f64)) {
return ctx.globals.m_native__abi__THISTLE_THX_SECTION.expect("global initialisation");
}
'__crust_v71: while (cursor < end) {
{
if (crate::m_native__memory::loadU8(ctx, memory, cursor) != 123.0f64) {
return ctx.globals.m_native__abi__THISTLE_THX_SECTION.expect("global initialisation");
}
let _ = { cursor = crate::m_native__thx::space(ctx, memory, (cursor + 1.0f64), end); cursor };
let mut seen: f64 = 0.0f64;
let mut hash: f64 = 0.0f64;
let mut nameLength: f64 = 0.0f64;
let mut flags: f64 = 0.0f64;
let mut align: i128 = 0i128;
let mut size: i128 = 0i128;
let mut address: i128 = 0i128;
let mut offset: i128 = 0i128;
let mut length: i128 = 0i128;
'__crust_v72: while ((cursor < end) && (crate::m_native__memory::loadU8(ctx, memory, cursor) != 125.0f64)) {
{
let _ = { cursor = crate::m_native__thx::parseString(ctx, memory, cursor, end); cursor };
if (cursor == ctx.globals.m_native__thx__BAD.expect("global initialisation")) {
return ctx.globals.m_native__abi__THISTLE_THX_SECTION.expect("global initialisation");
}
let key: f64 = crate::m_native__thx::keyId(ctx, memory);
let _ = { cursor = crate::m_native__thx::space(ctx, memory, cursor, end); cursor };
if ((cursor >= end) || (crate::m_native__memory::loadU8(ctx, memory, cursor) != 58.0f64)) {
return ctx.globals.m_native__abi__THISTLE_THX_SECTION.expect("global initialisation");
}
let _ = { cursor = crate::m_native__thx::space(ctx, memory, (cursor + 1.0f64), end); cursor };
let bit: f64 = (if ((key >= ctx.globals.m_native__thx__K_NAME.expect("global initialisation")) && (key <= ctx.globals.m_native__thx__K_LEN.expect("global initialisation"))) { numbers::shl(1.0f64, ((key - ctx.globals.m_native__thx__K_NAME.expect("global initialisation")))) } else { 0.0f64 });
if ((bit != 0.0f64) && ((numbers::and(seen, bit)) != 0.0f64)) {
return ctx.globals.m_native__abi__THISTLE_THX_SECTION.expect("global initialisation");
}
let stringField: bool = ((key == ctx.globals.m_native__thx__K_NAME.expect("global initialisation")) || (key == ctx.globals.m_native__thx__K_FLG.expect("global initialisation")));
let _ = { cursor = (if stringField { crate::m_native__thx::parseString(ctx, memory, cursor, end) } else { (if (bit != 0.0f64) { crate::m_native__thx::parseUnsigned(ctx, memory, cursor, end) } else { crate::m_native__thx::skipValue(ctx, memory, cursor, end, 0.0f64) }) }); cursor };
if (cursor == ctx.globals.m_native__thx__BAD.expect("global initialisation")) {
return ctx.globals.m_native__abi__THISTLE_THX_SECTION.expect("global initialisation");
}
if ((key == ctx.globals.m_native__thx__K_NAME.expect("global initialisation")) && (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING_ESCAPED.expect("global initialisation"))) != 0.0f64)) {
return ctx.globals.m_native__abi__THISTLE_THX_SECTION.expect("global initialisation");
}
if (key == ctx.globals.m_native__thx__K_NAME.expect("global initialisation")) {
let _ = { hash = crate::m_native__thx::nameHash(ctx, memory); hash };
}
if (key == ctx.globals.m_native__thx__K_NAME.expect("global initialisation")) {
let _ = { nameLength = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_STRING_LENGTH.expect("global initialisation"))); nameLength };
}
if (key == ctx.globals.m_native__thx__K_FLG.expect("global initialisation")) {
let _ = { flags = crate::m_native__thx::sectionFlags(ctx, memory); flags };
}
if ((key == ctx.globals.m_native__thx__K_FLG.expect("global initialisation")) && (flags == 4294967295.0f64)) {
return ctx.globals.m_native__abi__THISTLE_THX_SECTION.expect("global initialisation");
}
let value: i128 = crate::m_native__memory::loadU64(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_NUMBER.expect("global initialisation")));
if (key == ctx.globals.m_native__thx__K_ALIGN.expect("global initialisation")) {
let _ = { align = value; align };
}
if (key == ctx.globals.m_native__thx__K_SIZE.expect("global initialisation")) {
let _ = { size = value; size };
}
if (key == ctx.globals.m_native__thx__K_ADDR.expect("global initialisation")) {
let _ = { address = value; address };
}
if (key == ctx.globals.m_native__thx__K_AT.expect("global initialisation")) {
let _ = { offset = value; offset };
}
if (key == ctx.globals.m_native__thx__K_LEN.expect("global initialisation")) {
let _ = { length = value; length };
}
let _ = { seen = numbers::or(seen, bit); seen };
let _ = { cursor = crate::m_native__thx::space(ctx, memory, cursor, end); cursor };
if ((cursor < end) && (crate::m_native__memory::loadU8(ctx, memory, cursor) == 125.0f64)) {
break '__crust_v72;
}
if ((cursor >= end) || (crate::m_native__memory::loadU8(ctx, memory, cursor) != 44.0f64)) {
return ctx.globals.m_native__abi__THISTLE_THX_SECTION.expect("global initialisation");
}
let _ = { cursor = crate::m_native__thx::space(ctx, memory, (cursor + 1.0f64), end); cursor };
}
}
if (((((((((((cursor >= end) || (crate::m_native__memory::loadU8(ctx, memory, cursor) != 125.0f64)) || (seen != 127.0f64)) || (align == 0i128)) || (((align & (align).checked_sub(1i128).expect("BigInt arithmetic"))) != 0i128)) || ((address).checked_rem(align).expect("BigInt arithmetic") != 0i128)) || (length > size)) || (offset > crate::m_native__word::u32word(ctx, Numeric::Number(payloadLength)))) || (length > (crate::m_native__word::u32word(ctx, Numeric::Number(payloadLength))).checked_sub(offset).expect("BigInt arithmetic"))) || (offset > 4294967295i128)) || (length > 4294967295i128)) {
return ctx.globals.m_native__abi__THISTLE_THX_SECTION.expect("global initialisation");
}
let result: f64 = crate::m_native__kernel::thistleImageSegment(ctx, memory, hart, hash, nameLength, address, size, flags, (payload + crate::m_native__word::wordToU32(ctx, offset)), crate::m_native__word::wordToU32(ctx, length));
if (result != ctx.globals.m_native__abi__THISTLE_THX_OK.expect("global initialisation")) {
return result;
}
let _ = { cursor = crate::m_native__thx::space(ctx, memory, (cursor + 1.0f64), end); cursor };
if ((cursor < end) && (crate::m_native__memory::loadU8(ctx, memory, cursor) == 93.0f64)) {
return ctx.globals.m_native__abi__THISTLE_THX_OK.expect("global initialisation");
}
if ((cursor >= end) || (crate::m_native__memory::loadU8(ctx, memory, cursor) != 44.0f64)) {
return ctx.globals.m_native__abi__THISTLE_THX_SECTION.expect("global initialisation");
}
let _ = { cursor = crate::m_native__thx::space(ctx, memory, (cursor + 1.0f64), end); cursor };
}
}
return ctx.globals.m_native__abi__THISTLE_THX_SECTION.expect("global initialisation");
}


pub fn validateEnvelope<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut image: f64, mut imageLength: f64) -> f64 {
if (!crate::m_native__kernel::thistleImageContains(ctx, memory, image, imageLength)) {
return ctx.globals.m_native__abi__THISTLE_THX_RANGE.expect("global initialisation");
}
if (imageLength < 16.0f64) {
return ctx.globals.m_native__abi__THISTLE_THX_LENGTH.expect("global initialisation");
}
if ((((crate::m_native__memory::loadU8(ctx, memory, image) != 84.0f64) || (crate::m_native__memory::loadU8(ctx, memory, (image + 1.0f64)) != 72.0f64)) || (crate::m_native__memory::loadU8(ctx, memory, (image + 2.0f64)) != 88.0f64)) || (crate::m_native__memory::loadU8(ctx, memory, (image + 3.0f64)) != 50.0f64)) {
return ctx.globals.m_native__abi__THISTLE_THX_MAGIC.expect("global initialisation");
}
let headerLength: f64 = crate::m_native__memory::loadU32(ctx, memory, (image + 4.0f64));
let payloadLength: f64 = crate::m_native__memory::loadU32(ctx, memory, (image + 8.0f64));
if ((headerLength > (imageLength - 16.0f64)) || (payloadLength != ((imageLength - 16.0f64) - headerLength))) {
return ctx.globals.m_native__abi__THISTLE_THX_LENGTH.expect("global initialisation");
}
let mut checksum: f64 = 2166136261.0f64;
let mut index: f64 = 16.0f64;
'__crust_v73: while (index < imageLength) {
{
let _ = { checksum = crate::m_native__word::mulU32(ctx, numbers::xor(checksum, crate::m_native__memory::loadU8(ctx, memory, (image + index))), 16777619.0f64); checksum };
let _ = { index = (index + 1.0f64); index };
}
}
if (checksum != crate::m_native__memory::loadU32(ctx, memory, (image + 12.0f64))) {
return ctx.globals.m_native__abi__THISTLE_THX_CHECKSUM.expect("global initialisation");
}
return ctx.globals.m_native__abi__THISTLE_THX_OK.expect("global initialisation");
}


pub fn parseHeaderValue<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut key: f64, mut at: f64, mut end: f64) -> f64 {
let stringField: bool = ((key == ctx.globals.m_native__thx__K_MACHINE.expect("global initialisation")) || (key == ctx.globals.m_native__thx__K_ISA.expect("global initialisation")));
let numericField: bool = ((((((key == ctx.globals.m_native__thx__K_VER.expect("global initialisation")) || (key == ctx.globals.m_native__thx__K_ENTRY.expect("global initialisation"))) || (key == ctx.globals.m_native__thx__K_MEM.expect("global initialisation"))) || (key == ctx.globals.m_native__thx__K_PHDR.expect("global initialisation"))) || (key == ctx.globals.m_native__thx__K_PHENT.expect("global initialisation"))) || (key == ctx.globals.m_native__thx__K_PHNUM.expect("global initialisation")));
let arrayField: bool = ((((key == ctx.globals.m_native__thx__K_SYM.expect("global initialisation")) || (key == ctx.globals.m_native__thx__K_REL.expect("global initialisation"))) || (key == ctx.globals.m_native__thx__K_DBG.expect("global initialisation"))) || (key == ctx.globals.m_native__thx__K_IDENT.expect("global initialisation")));
if (arrayField && (((at >= end) || (crate::m_native__memory::loadU8(ctx, memory, at) != 91.0f64)))) {
return ctx.globals.m_native__thx__BAD_METADATA.expect("global initialisation");
}
let cursor: f64 = (if stringField { crate::m_native__thx::parseString(ctx, memory, at, end) } else { (if numericField { crate::m_native__thx::parseUnsigned(ctx, memory, at, end) } else { crate::m_native__thx::skipValue(ctx, memory, at, end, 0.0f64) }) });
if (cursor == ctx.globals.m_native__thx__BAD.expect("global initialisation")) {
return (if (stringField || numericField) { ctx.globals.m_native__thx__BAD_METADATA.expect("global initialisation") } else { ctx.globals.m_native__thx__BAD.expect("global initialisation") });
}
if (((key == ctx.globals.m_native__thx__K_MACHINE.expect("global initialisation")) && (!crate::m_native__thx::exactMachine(ctx, memory))) || ((key == ctx.globals.m_native__thx__K_ISA.expect("global initialisation")) && (!crate::m_native__thx::exactIsa(ctx, memory)))) {
return ctx.globals.m_native__thx__BAD_METADATA.expect("global initialisation");
}
let value: i128 = crate::m_native__memory::loadU64(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_NUMBER.expect("global initialisation")));
if ((key == ctx.globals.m_native__thx__K_VER.expect("global initialisation")) && (value != 2i128)) {
return ctx.globals.m_native__thx__BAD_METADATA.expect("global initialisation");
}
if ((((key == ctx.globals.m_native__thx__K_PHENT.expect("global initialisation")) || (key == ctx.globals.m_native__thx__K_PHNUM.expect("global initialisation")))) && (value > 4294967295i128)) {
return ctx.globals.m_native__thx__BAD_METADATA.expect("global initialisation");
}
return cursor;
}


pub fn validateHeader<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut header: f64, mut end: f64) -> f64 {
let mut cursor: f64 = crate::m_native__thx::space(ctx, memory, header, end);
if ((cursor >= end) || (crate::m_native__memory::loadU8(ctx, memory, cursor) != 123.0f64)) {
return ctx.globals.m_native__abi__THISTLE_THX_JSON.expect("global initialisation");
}
let _ = { cursor = crate::m_native__thx::space(ctx, memory, (cursor + 1.0f64), end); cursor };
let mut seen: f64 = 0.0f64;
'__crust_v74: while ((cursor < end) && (crate::m_native__memory::loadU8(ctx, memory, cursor) != 125.0f64)) {
{
let _ = { cursor = crate::m_native__thx::parseString(ctx, memory, cursor, end); cursor };
if (cursor == ctx.globals.m_native__thx__BAD.expect("global initialisation")) {
return ctx.globals.m_native__abi__THISTLE_THX_JSON.expect("global initialisation");
}
let key: f64 = crate::m_native__thx::keyId(ctx, memory);
let _ = { cursor = crate::m_native__thx::space(ctx, memory, cursor, end); cursor };
if ((cursor >= end) || (crate::m_native__memory::loadU8(ctx, memory, cursor) != 58.0f64)) {
return ctx.globals.m_native__abi__THISTLE_THX_JSON.expect("global initialisation");
}
let _ = { cursor = crate::m_native__thx::space(ctx, memory, (cursor + 1.0f64), end); cursor };
let bit: f64 = (if ((key >= ctx.globals.m_native__thx__K_MACHINE.expect("global initialisation")) && (key <= ctx.globals.m_native__thx__K_PHNUM.expect("global initialisation"))) { numbers::shl(1.0f64, ((key - ctx.globals.m_native__thx__K_MACHINE.expect("global initialisation")))) } else { 0.0f64 });
if ((bit != 0.0f64) && ((numbers::and(seen, bit)) != 0.0f64)) {
return ctx.globals.m_native__abi__THISTLE_THX_METADATA.expect("global initialisation");
}
let _ = { cursor = crate::m_native__thx::parseHeaderValue(ctx, memory, key, cursor, end); cursor };
if (cursor == ctx.globals.m_native__thx__BAD.expect("global initialisation")) {
return ctx.globals.m_native__abi__THISTLE_THX_JSON.expect("global initialisation");
}
if (cursor == ctx.globals.m_native__thx__BAD_METADATA.expect("global initialisation")) {
return ctx.globals.m_native__abi__THISTLE_THX_METADATA.expect("global initialisation");
}
let _ = { seen = numbers::or(seen, bit); seen };
let _ = { cursor = crate::m_native__thx::space(ctx, memory, cursor, end); cursor };
if ((cursor < end) && (crate::m_native__memory::loadU8(ctx, memory, cursor) == 125.0f64)) {
break '__crust_v74;
}
if ((cursor >= end) || (crate::m_native__memory::loadU8(ctx, memory, cursor) != 44.0f64)) {
return ctx.globals.m_native__abi__THISTLE_THX_JSON.expect("global initialisation");
}
let _ = { cursor = crate::m_native__thx::space(ctx, memory, (cursor + 1.0f64), end); cursor };
}
}
if ((((cursor >= end) || (crate::m_native__memory::loadU8(ctx, memory, cursor) != 125.0f64)) || (crate::m_native__thx::space(ctx, memory, (cursor + 1.0f64), end) != end)) || ((numbers::and(seen, 1023.0f64)) != 1023.0f64)) {
return ctx.globals.m_native__abi__THISTLE_THX_METADATA.expect("global initialisation");
}
return ctx.globals.m_native__abi__THISTLE_THX_OK.expect("global initialisation");
}


pub fn headerValue<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut header: f64, mut end: f64, mut wanted: f64) -> i128 {
let mut cursor: f64 = crate::m_native__thx::space(ctx, memory, (crate::m_native__thx::space(ctx, memory, header, end) + 1.0f64), end);
'__crust_v75: while ((cursor < end) && (crate::m_native__memory::loadU8(ctx, memory, cursor) != 125.0f64)) {
{
let _ = { cursor = crate::m_native__thx::parseString(ctx, memory, cursor, end); cursor };
let key: f64 = crate::m_native__thx::keyId(ctx, memory);
let _ = { cursor = crate::m_native__thx::space(ctx, memory, (crate::m_native__thx::space(ctx, memory, cursor, end) + 1.0f64), end); cursor };
if ((key == wanted) && (key == ctx.globals.m_native__thx__K_SEC.expect("global initialisation"))) {
return crate::m_native__word::u32word(ctx, Numeric::Number(cursor));
}
let _ = { cursor = crate::m_native__thx::parseHeaderValue(ctx, memory, key, cursor, end); cursor };
if (key == wanted) {
return crate::m_native__memory::loadU64(ctx, memory, (ctx.globals.m_native__abi__THISTLE_PARSER_BASE.expect("global initialisation") + ctx.globals.m_native__abi__THISTLE_PARSER_NUMBER.expect("global initialisation")));
}
let _ = { cursor = crate::m_native__thx::space(ctx, memory, cursor, end); cursor };
if ((cursor >= end) || (crate::m_native__memory::loadU8(ctx, memory, cursor) == 125.0f64)) {
break '__crust_v75;
}
let _ = { cursor = crate::m_native__thx::space(ctx, memory, (cursor + 1.0f64), end); cursor };
}
}
return 0i128;
}


pub fn thistleLoadThx<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut hart: f64, mut image: f64, mut imageLength: f64) -> f64 {
let status: f64 = crate::m_native__thx::validateEnvelope(ctx, memory, image, imageLength);
if (status != ctx.globals.m_native__abi__THISTLE_THX_OK.expect("global initialisation")) {
return status;
}
let header: f64 = (image + 16.0f64);
let end: f64 = (header + crate::m_native__memory::loadU32(ctx, memory, (image + 4.0f64)));
let payloadLength: f64 = crate::m_native__memory::loadU32(ctx, memory, (image + 8.0f64));
let metadata: f64 = crate::m_native__thx::validateHeader(ctx, memory, header, end);
if (metadata != ctx.globals.m_native__abi__THISTLE_THX_OK.expect("global initialisation")) {
return metadata;
}
let entry: i128 = crate::m_native__thx::headerValue(ctx, memory, header, end, ctx.globals.m_native__thx__K_ENTRY.expect("global initialisation"));
let top: i128 = crate::m_native__thx::headerValue(ctx, memory, header, end, ctx.globals.m_native__thx__K_MEM.expect("global initialisation"));
let phdr: i128 = crate::m_native__thx::headerValue(ctx, memory, header, end, ctx.globals.m_native__thx__K_PHDR.expect("global initialisation"));
let phent: f64 = crate::m_native__word::wordToU32(ctx, crate::m_native__thx::headerValue(ctx, memory, header, end, ctx.globals.m_native__thx__K_PHENT.expect("global initialisation")));
let phnum: f64 = crate::m_native__word::wordToU32(ctx, crate::m_native__thx::headerValue(ctx, memory, header, end, ctx.globals.m_native__thx__K_PHNUM.expect("global initialisation")));
let sectionAt: f64 = crate::m_native__word::wordToU32(ctx, crate::m_native__thx::headerValue(ctx, memory, header, end, ctx.globals.m_native__thx__K_SEC.expect("global initialisation")));
let mut result: f64 = crate::m_native__kernel::thistleImageBegin(ctx, memory, hart, top, entry, phdr, phent, phnum);
if (result != ctx.globals.m_native__abi__THISTLE_THX_OK.expect("global initialisation")) {
return result;
}
let _ = { result = crate::m_native__thx::parseSections(ctx, memory, hart, sectionAt, end, end, payloadLength); result };
if (result != ctx.globals.m_native__abi__THISTLE_THX_OK.expect("global initialisation")) {
return result;
}
return crate::m_native__kernel::thistleImageFinish(ctx, memory, hart, imageLength);
}

