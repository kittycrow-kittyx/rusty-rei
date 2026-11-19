use crate::*;

pub fn memoryContains<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut size: f64) -> bool {
let total: f64 = crate::m_native__memory::memorySize(ctx, memory);
return ((at <= total) && (size <= (total - at)));
}


pub fn inodeAt<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut inode: f64) -> f64 {
return (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_INODE_BASE.expect("global initialisation"))) + (inode * ctx.globals.m_native__abi__THISTLE_VFS_INODE_STRIDE.expect("global initialisation")));
}


pub fn dentryAt<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut index: f64) -> f64 {
return (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DENTRY_BASE.expect("global initialisation"))) + (index * ctx.globals.m_native__abi__THISTLE_VFS_DENTRY_STRIDE.expect("global initialisation")));
}


pub fn align4<P: Platform>(ctx: &Context<P>, mut value: f64) -> f64 {
return numbers::and(((value + 3.0f64)), (-4.0f64));
}


pub fn hashBytes<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut size: f64) -> f64 {
let mut hash: f64 = 2166136261.0f64;
let mut index: f64 = 0.0f64;
'__crust_v3: while (index < size) {
{
let _ = { hash = crate::m_native__word::mulU32(ctx, numbers::xor(hash, crate::m_native__memory::loadU8(ctx, memory, (at + index))), 16777619.0f64); hash };
let _ = { index = (index + 1.0f64); index };
}
}
return hash;
}


pub fn allocateData<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut size: f64) -> f64 {
if (size == 0.0f64) {
return 0.0f64;
}
let aligned: f64 = crate::m_native__vfs::align4(ctx, size);
if (aligned < size) {
return 4294967295.0f64;
}
let next: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DATA_NEXT.expect("global initialisation")));
let end: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DATA_END.expect("global initialisation")));
if ((next > end) || (aligned > (end - next))) {
return 4294967295.0f64;
}
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DATA_NEXT.expect("global initialisation")), Numeric::Number((next + aligned)));
return next;
}


pub fn validName<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut size: f64) -> bool {
if ((size == 0.0f64) || (size > 255.0f64)) {
return false;
}
let mut index: f64 = 0.0f64;
'__crust_v4: while (index < size) {
{
let value: f64 = crate::m_native__memory::loadU8(ctx, memory, (at + index));
if ((value == 0.0f64) || (value == 47.0f64)) {
return false;
}
let _ = { index = (index + 1.0f64); index };
}
}
if ((size == 1.0f64) && (crate::m_native__memory::loadU8(ctx, memory, at) == 46.0f64)) {
return false;
}
if (((size == 2.0f64) && (crate::m_native__memory::loadU8(ctx, memory, at) == 46.0f64)) && (crate::m_native__memory::loadU8(ctx, memory, (at + 1.0f64)) == 46.0f64)) {
return false;
}
return true;
}


pub fn sameName<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut dentry: f64, mut at: f64, mut size: f64, mut hash: f64) -> bool {
if ((crate::m_native__memory::loadU32(ctx, memory, (dentry + ctx.globals.m_native__abi__D_NAME_LENGTH.expect("global initialisation"))) != size) || (crate::m_native__memory::loadU32(ctx, memory, (dentry + ctx.globals.m_native__abi__D_NAME_HASH.expect("global initialisation"))) != hash)) {
return false;
}
let stored: f64 = crate::m_native__memory::loadU32(ctx, memory, (dentry + ctx.globals.m_native__abi__D_NAME.expect("global initialisation")));
let mut index: f64 = 0.0f64;
'__crust_v5: while (index < size) {
{
if (crate::m_native__memory::loadU8(ctx, memory, (stored + index)) != crate::m_native__memory::loadU8(ctx, memory, (at + index))) {
return false;
}
let _ = { index = (index + 1.0f64); index };
}
}
return true;
}


pub fn findChild<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut parent: f64, mut name: f64, mut nameLength: f64) -> f64 {
if (!crate::m_native__vfs::validName(ctx, memory, name, nameLength)) {
return 0.0f64;
}
let hash: f64 = crate::m_native__vfs::hashBytes(ctx, memory, name, nameLength);
let count: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DENTRY_COUNT.expect("global initialisation")));
let mut index: f64 = 0.0f64;
'__crust_v6: while (index < count) {
{
let dentry: f64 = crate::m_native__vfs::dentryAt(ctx, memory, index);
if ((crate::m_native__memory::loadU32(ctx, memory, (dentry + ctx.globals.m_native__abi__D_PARENT.expect("global initialisation"))) == parent) && crate::m_native__vfs::sameName(ctx, memory, dentry, name, nameLength, hash)) {
{
return crate::m_native__memory::loadU32(ctx, memory, (dentry + ctx.globals.m_native__abi__D_INODE.expect("global initialisation")));
}
}
let _ = { index = (index + 1.0f64); index };
}
}
return 0.0f64;
}


pub fn parentOf<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut inode: f64) -> f64 {
let root: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_ROOT.expect("global initialisation")));
if (inode == root) {
return root;
}
let count: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DENTRY_COUNT.expect("global initialisation")));
let mut index: f64 = 0.0f64;
'__crust_v7: while (index < count) {
{
let dentry: f64 = crate::m_native__vfs::dentryAt(ctx, memory, index);
if (crate::m_native__memory::loadU32(ctx, memory, (dentry + ctx.globals.m_native__abi__D_INODE.expect("global initialisation"))) == inode) {
return crate::m_native__memory::loadU32(ctx, memory, (dentry + ctx.globals.m_native__abi__D_PARENT.expect("global initialisation")));
}
let _ = { index = (index + 1.0f64); index };
}
}
return 0.0f64;
}


pub fn inSupplementaryGroup<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut proc: f64, mut gid: f64) -> bool {
let count: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_GROUP_COUNT.expect("global initialisation")));
let mut index: f64 = 0.0f64;
'__crust_v8: while (index < count) {
{
if (crate::m_native__memory::loadU32(ctx, memory, ((proc + ctx.globals.m_native__abi__P_GROUPS.expect("global initialisation")) + (index * 4.0f64))) == gid) {
return true;
}
let _ = { index = (index + 1.0f64); index };
}
}
return false;
}


pub fn allowed<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut proc: f64, mut inode: f64, mut bits: f64) -> bool {
let state: f64 = crate::m_native__vfs::inodeAt(ctx, memory, inode);
let uid: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_FSUID.expect("global initialisation")));
if (uid == 0.0f64) {
return true;
}
let owner: f64 = crate::m_native__memory::loadU32(ctx, memory, (state + ctx.globals.m_native__abi__V_UID.expect("global initialisation")));
let group: f64 = crate::m_native__memory::loadU32(ctx, memory, (state + ctx.globals.m_native__abi__V_GID.expect("global initialisation")));
let gid: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_FSGID.expect("global initialisation")));
let shift: f64 = (if (uid == owner) { 6.0f64 } else { (if ((gid == group) || crate::m_native__vfs::inSupplementaryGroup(ctx, memory, proc, group)) { 3.0f64 } else { 0.0f64 }) });
return ((numbers::and((numbers::ushr(crate::m_native__memory::loadU32(ctx, memory, (state + ctx.globals.m_native__abi__V_MODE.expect("global initialisation"))), shift)), bits)) == bits);
}


pub fn moreComponents<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut at: f64, mut end: f64) -> bool {
let mut cursor: f64 = at;
'__crust_v9: while (cursor < end) {
{
if (crate::m_native__memory::loadU8(ctx, memory, cursor) != 47.0f64) {
return true;
}
let _ = { cursor = (cursor + 1.0f64); cursor };
}
}
return false;
}


pub fn resolveLink<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut proc: f64, mut current: f64, mut child: f64, mut hops: f64) -> f64 {
let link: f64 = crate::m_native__vfs::inodeAt(ctx, memory, child);
let target: f64 = crate::m_native__memory::loadU32(ctx, memory, (link + ctx.globals.m_native__abi__V_DATA.expect("global initialisation")));
let targetLength: f64 = crate::m_native__word::wordToU32(ctx, crate::m_native__memory::loadU64(ctx, memory, (link + ctx.globals.m_native__abi__V_SIZE.expect("global initialisation"))));
if ((targetLength == 0.0f64) || (targetLength > 4096.0f64)) {
return (-2.0f64);
}
let root: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_ROOT.expect("global initialisation")));
let base: f64 = (if (crate::m_native__memory::loadU8(ctx, memory, target) == 47.0f64) { root } else { current });
return crate::m_native__vfs::resolveRange(ctx, memory, proc, base, target, targetLength, numbers::or((numbers::shl(((hops + 1.0f64)), 1.0f64)), 1.0f64));
}


pub fn resolveRange<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut proc: f64, mut start: f64, mut path: f64, mut pathLength: f64, mut traversal: f64) -> f64 {
let followFinal: bool = ((numbers::and(traversal, 1.0f64)) != 0.0f64);
let hops: f64 = numbers::ushr(traversal, 1.0f64);
if (hops > 32.0f64) {
return (-40.0f64);
}
let root: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_ROOT.expect("global initialisation")));
let end: f64 = (path + pathLength);
let mut cursor: f64 = path;
let mut current: f64 = (if ((pathLength != 0.0f64) && (crate::m_native__memory::loadU8(ctx, memory, path) == 47.0f64)) { root } else { start });
if ((current == 0.0f64) || (crate::m_native__vfs::thistleVfsKind(ctx, memory, current) != ctx.globals.m_native__abi__THISTLE_VFS_KIND_DIRECTORY.expect("global initialisation"))) {
return (-20.0f64);
}
if (pathLength == 0.0f64) {
return (-2.0f64);
}
'__crust_v10: while (cursor < end) {
{
'__crust_v11: while ((cursor < end) && (crate::m_native__memory::loadU8(ctx, memory, cursor) == 47.0f64)) {
let _ = { cursor = (cursor + 1.0f64); cursor };
}
if (cursor == end) {
break '__crust_v10;
}
let component: f64 = cursor;
'__crust_v12: while ((cursor < end) && (crate::m_native__memory::loadU8(ctx, memory, cursor) != 47.0f64)) {
let _ = { cursor = (cursor + 1.0f64); cursor };
}
let length: f64 = (cursor - component);
if (length > 255.0f64) {
return (-36.0f64);
}
let ts_final: bool = (!crate::m_native__vfs::moreComponents(ctx, memory, cursor, end));
if ((length == 1.0f64) && (crate::m_native__memory::loadU8(ctx, memory, component) == 46.0f64)) {
continue '__crust_v10;
}
let dotdot: bool = (((length == 2.0f64) && (crate::m_native__memory::loadU8(ctx, memory, component) == 46.0f64)) && (crate::m_native__memory::loadU8(ctx, memory, (component + 1.0f64)) == 46.0f64));
let parent: f64 = (if dotdot { crate::m_native__vfs::parentOf(ctx, memory, current) } else { current });
if (dotdot && (parent == 0.0f64)) {
return (-2.0f64);
}
if dotdot {
{
let _ = { current = parent; current };
continue '__crust_v10;
}
}
if (crate::m_native__vfs::thistleVfsKind(ctx, memory, current) != ctx.globals.m_native__abi__THISTLE_VFS_KIND_DIRECTORY.expect("global initialisation")) {
return (-20.0f64);
}
if (!crate::m_native__vfs::allowed(ctx, memory, proc, current, 1.0f64)) {
return (-13.0f64);
}
let child: f64 = crate::m_native__vfs::findChild(ctx, memory, current, component, length);
if (child == 0.0f64) {
return (-2.0f64);
}
let follow: bool = ((crate::m_native__vfs::thistleVfsKind(ctx, memory, child) == ctx.globals.m_native__abi__THISTLE_VFS_KIND_LINK.expect("global initialisation")) && ((((!ts_final) || followFinal) || (cursor < end))));
let resolved: f64 = (if follow { crate::m_native__vfs::resolveLink(ctx, memory, proc, current, child, hops) } else { crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, child)) });
if (resolved < 0.0f64) {
return resolved;
}
let _ = { current = crate::m_native__word::wordToU32(ctx, crate::m_native__word::word(ctx, resolved)); current };
}
}
if (((pathLength != 0.0f64) && (crate::m_native__memory::loadU8(ctx, memory, (end - 1.0f64)) == 47.0f64)) && (crate::m_native__vfs::thistleVfsKind(ctx, memory, current) != ctx.globals.m_native__abi__THISTLE_VFS_KIND_DIRECTORY.expect("global initialisation"))) {
return (-20.0f64);
}
return crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, current));
}


pub fn thistleVfsLoaded<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory) -> bool {
return (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_LOADED.expect("global initialisation"))) == 1.0f64);
}


pub fn thistleVfsRoot<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory) -> f64 {
return (if crate::m_native__vfs::thistleVfsLoaded(ctx, memory) { crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_ROOT.expect("global initialisation"))) } else { 0.0f64 });
}


pub fn thistleVfsInodeCount<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory) -> f64 {
return (if crate::m_native__vfs::thistleVfsLoaded(ctx, memory) { crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_INODE_COUNT.expect("global initialisation"))) } else { 0.0f64 });
}


pub fn thistleVfsDentryCount<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory) -> f64 {
return (if crate::m_native__vfs::thistleVfsLoaded(ctx, memory) { crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DENTRY_COUNT.expect("global initialisation"))) } else { 0.0f64 });
}


pub fn thistleVfsKind<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut inode: f64) -> f64 {
if (((!crate::m_native__vfs::thistleVfsLoaded(ctx, memory)) || (inode == 0.0f64)) || (inode >= crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_INODE_CAPACITY.expect("global initialisation"))))) {
return 0.0f64;
}
return crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__vfs::inodeAt(ctx, memory, inode) + ctx.globals.m_native__abi__V_KIND.expect("global initialisation")));
}


pub fn thistleVfsFileSize<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut inode: f64) -> i128 {
return (if (crate::m_native__vfs::thistleVfsKind(ctx, memory, inode) == ctx.globals.m_native__abi__THISTLE_VFS_KIND_EMPTY.expect("global initialisation")) { 0i128 } else { crate::m_native__memory::loadU64(ctx, memory, (crate::m_native__vfs::inodeAt(ctx, memory, inode) + ctx.globals.m_native__abi__V_SIZE.expect("global initialisation"))) });
}


pub fn thistleVfsMode<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut inode: f64) -> f64 {
return (if (crate::m_native__vfs::thistleVfsKind(ctx, memory, inode) == ctx.globals.m_native__abi__THISTLE_VFS_KIND_EMPTY.expect("global initialisation")) { 0.0f64 } else { crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__vfs::inodeAt(ctx, memory, inode) + ctx.globals.m_native__abi__V_MODE.expect("global initialisation"))) });
}


pub fn thistleVfsUid<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut inode: f64) -> f64 {
return (if (crate::m_native__vfs::thistleVfsKind(ctx, memory, inode) == ctx.globals.m_native__abi__THISTLE_VFS_KIND_EMPTY.expect("global initialisation")) { 0.0f64 } else { crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__vfs::inodeAt(ctx, memory, inode) + ctx.globals.m_native__abi__V_UID.expect("global initialisation"))) });
}


pub fn thistleVfsGid<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut inode: f64) -> f64 {
return (if (crate::m_native__vfs::thistleVfsKind(ctx, memory, inode) == ctx.globals.m_native__abi__THISTLE_VFS_KIND_EMPTY.expect("global initialisation")) { 0.0f64 } else { crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__vfs::inodeAt(ctx, memory, inode) + ctx.globals.m_native__abi__V_GID.expect("global initialisation"))) });
}


pub fn thistleVfsNlink<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut inode: f64) -> f64 {
return (if (crate::m_native__vfs::thistleVfsKind(ctx, memory, inode) == ctx.globals.m_native__abi__THISTLE_VFS_KIND_EMPTY.expect("global initialisation")) { 0.0f64 } else { crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__vfs::inodeAt(ctx, memory, inode) + ctx.globals.m_native__abi__V_NLINK.expect("global initialisation"))) });
}


pub fn thistleVfsLookup<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut parent: f64, mut name: f64, mut nameLength: f64) -> f64 {
if ((((!crate::m_native__vfs::thistleVfsLoaded(ctx, memory)) || (!crate::m_native__image_range::inputContains(ctx, memory, name, nameLength))) || (!crate::m_native__vfs::validName(ctx, memory, name, nameLength))) || (crate::m_native__vfs::thistleVfsKind(ctx, memory, parent) != ctx.globals.m_native__abi__THISTLE_VFS_KIND_DIRECTORY.expect("global initialisation"))) {
return 0.0f64;
}
return crate::m_native__vfs::findChild(ctx, memory, parent, name, nameLength);
}


pub fn thistleVfsResolve<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut proc: f64, mut start: f64, mut path: f64, mut pathLength: f64, mut followFinal: bool) -> f64 {
if (((!crate::m_native__vfs::thistleVfsLoaded(ctx, memory)) || (pathLength > 4096.0f64)) || (!crate::m_native__vfs::memoryContains(ctx, memory, path, pathLength))) {
return (-22.0f64);
}
return crate::m_native__vfs::resolveRange(ctx, memory, proc, start, path, pathLength, (if followFinal { 1.0f64 } else { 0.0f64 }));
}


pub fn thistleVfsAccess<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut proc: f64, mut inode: f64, mut bits: f64) -> bool {
return (((crate::m_native__vfs::thistleVfsKind(ctx, memory, inode) != ctx.globals.m_native__abi__THISTLE_VFS_KIND_EMPTY.expect("global initialisation")) && (bits <= 7.0f64)) && crate::m_native__vfs::allowed(ctx, memory, proc, inode, bits));
}


pub fn readData<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut inode: f64, mut offset: i128, mut output: f64, mut length: f64) -> f64 {
let kind: f64 = crate::m_native__vfs::thistleVfsKind(ctx, memory, inode);
if ((kind != ctx.globals.m_native__abi__THISTLE_VFS_KIND_FILE.expect("global initialisation")) && (kind != ctx.globals.m_native__abi__THISTLE_VFS_KIND_LINK.expect("global initialisation"))) {
return (-22.0f64);
}
let state: f64 = crate::m_native__vfs::inodeAt(ctx, memory, inode);
let size: i128 = crate::m_native__memory::loadU64(ctx, memory, (state + ctx.globals.m_native__abi__V_SIZE.expect("global initialisation")));
if (offset >= size) {
return 0.0f64;
}
let available: i128 = (size).checked_sub(offset).expect("BigInt arithmetic");
let amount: f64 = (if (available < crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, length))) { crate::m_native__word::wordToU32(ctx, available) } else { length });
if (amount != 0.0f64) {
let _ = crate::m_native__memory::copyMemory(ctx, memory, output, (crate::m_native__memory::loadU32(ctx, memory, (state + ctx.globals.m_native__abi__V_DATA.expect("global initialisation"))) + crate::m_native__word::wordToU32(ctx, offset)), amount);
}
return crate::m_native__word::wordToU32(ctx, crate::m_native__word::word(ctx, amount));
}


pub fn thistleVfsReadData<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut inode: f64, mut offset: i128, mut output: f64, mut length: f64) -> f64 {
if (!crate::m_native__image_range::inputContains(ctx, memory, output, length)) {
return (-22.0f64);
}
return crate::m_native__vfs::readData(ctx, memory, inode, offset, output, length);
}


pub fn thistleVfsReadScratch<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut inode: f64, mut offset: i128, mut length: f64) -> f64 {
let output: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_SCRATCH_BASE.expect("global initialisation")));
if ((length > ctx.globals.m_native__abi__THISTLE_VFS_PATH_CAPACITY.expect("global initialisation")) || (!crate::m_native__vfs::memoryContains(ctx, memory, output, length))) {
return (-22.0f64);
}
return crate::m_native__vfs::readData(ctx, memory, inode, offset, output, length);
}


pub fn resetVfs<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut inodeCapacity: f64, mut dentryCapacity: f64) -> () {
let inodeBase: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_INODE_BASE.expect("global initialisation")));
let dentryBase: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DENTRY_BASE.expect("global initialisation")));
let dataBase: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DATA_BASE.expect("global initialisation")));
let oldNext: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DATA_NEXT.expect("global initialisation")));
let _ = crate::m_native__memory::fill(ctx, memory, inodeBase, (inodeCapacity * ctx.globals.m_native__abi__THISTLE_VFS_INODE_STRIDE.expect("global initialisation")), 0.0f64);
let _ = crate::m_native__memory::fill(ctx, memory, dentryBase, (dentryCapacity * ctx.globals.m_native__abi__THISTLE_VFS_DENTRY_STRIDE.expect("global initialisation")), 0.0f64);
if (oldNext > dataBase) {
let _ = crate::m_native__memory::fill(ctx, memory, dataBase, (oldNext - dataBase), 0.0f64);
}
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DATA_NEXT.expect("global initialisation")), Numeric::Number(dataBase));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_LOADED.expect("global initialisation")), Numeric::Number(0.0f64));
}


pub fn loadInodes<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut start: f64, mut end: f64, mut inodeCount: f64, mut inodeCapacity: f64, mut dataCapacity: f64) -> i128 {
let mut cursor: f64 = start;
let mut parsed: f64 = 0.0f64;
'__crust_v13: while (parsed < inodeCount) {
{
if ((cursor > end) || (ctx.globals.m_native__vfs__VFS_INODE_IMAGE_SIZE.expect("global initialisation") > (end - cursor))) {
return crate::m_native__word::word(ctx, (-ctx.globals.m_native__abi__THISTLE_VFS_FORMAT.expect("global initialisation")));
}
let inode: f64 = crate::m_native__memory::loadU32(ctx, memory, cursor);
let kind: f64 = crate::m_native__memory::loadU32(ctx, memory, (cursor + 4.0f64));
let payloadLength: f64 = crate::m_native__memory::loadU32(ctx, memory, (cursor + 24.0f64));
if ((((((((inode == 0.0f64) || (inode >= inodeCapacity)) || ((((kind != ctx.globals.m_native__abi__THISTLE_VFS_KIND_FILE.expect("global initialisation")) && (kind != ctx.globals.m_native__abi__THISTLE_VFS_KIND_DIRECTORY.expect("global initialisation"))) && (kind != ctx.globals.m_native__abi__THISTLE_VFS_KIND_LINK.expect("global initialisation"))))) || (crate::m_native__memory::loadU32(ctx, memory, (cursor + 20.0f64)) == 0.0f64)) || (payloadLength > dataCapacity)) || (((kind == ctx.globals.m_native__abi__THISTLE_VFS_KIND_DIRECTORY.expect("global initialisation")) && (payloadLength != 0.0f64)))) || ((cursor + ctx.globals.m_native__vfs__VFS_INODE_IMAGE_SIZE.expect("global initialisation")) > end)) || (payloadLength > (end - ((cursor + ctx.globals.m_native__vfs__VFS_INODE_IMAGE_SIZE.expect("global initialisation")))))) {
return crate::m_native__word::word(ctx, (-ctx.globals.m_native__abi__THISTLE_VFS_FORMAT.expect("global initialisation")));
}
let state: f64 = crate::m_native__vfs::inodeAt(ctx, memory, inode);
if (crate::m_native__memory::loadU32(ctx, memory, (state + ctx.globals.m_native__abi__V_KIND.expect("global initialisation"))) != ctx.globals.m_native__abi__THISTLE_VFS_KIND_EMPTY.expect("global initialisation")) {
return crate::m_native__word::word(ctx, (-ctx.globals.m_native__abi__THISTLE_VFS_DUPLICATE.expect("global initialisation")));
}
let data: f64 = crate::m_native__vfs::allocateData(ctx, memory, payloadLength);
if (data == 4294967295.0f64) {
return crate::m_native__word::word(ctx, (-ctx.globals.m_native__abi__THISTLE_VFS_CAPACITY_ERROR.expect("global initialisation")));
}
if (payloadLength != 0.0f64) {
let _ = crate::m_native__memory::copyMemory(ctx, memory, data, (cursor + ctx.globals.m_native__vfs__VFS_INODE_IMAGE_SIZE.expect("global initialisation")), payloadLength);
}
let _ = crate::m_native__memory::storeU32(ctx, memory, (state + ctx.globals.m_native__abi__V_KIND.expect("global initialisation")), Numeric::Number(kind));
let _ = crate::m_native__memory::storeU32(ctx, memory, (state + ctx.globals.m_native__abi__V_MODE.expect("global initialisation")), Numeric::Number(numbers::and(crate::m_native__memory::loadU32(ctx, memory, (cursor + 8.0f64)), 4095.0f64)));
let _ = crate::m_native__memory::storeU32(ctx, memory, (state + ctx.globals.m_native__abi__V_UID.expect("global initialisation")), Numeric::Number(crate::m_native__memory::loadU32(ctx, memory, (cursor + 12.0f64))));
let _ = crate::m_native__memory::storeU32(ctx, memory, (state + ctx.globals.m_native__abi__V_GID.expect("global initialisation")), Numeric::Number(crate::m_native__memory::loadU32(ctx, memory, (cursor + 16.0f64))));
let _ = crate::m_native__memory::storeU32(ctx, memory, (state + ctx.globals.m_native__abi__V_NLINK.expect("global initialisation")), Numeric::Number(crate::m_native__memory::loadU32(ctx, memory, (cursor + 20.0f64))));
let _ = crate::m_native__memory::storeU32(ctx, memory, (state + ctx.globals.m_native__abi__V_FLAGS.expect("global initialisation")), Numeric::Number(0.0f64));
let _ = crate::m_native__memory::storeU64(ctx, memory, (state + ctx.globals.m_native__abi__V_SIZE.expect("global initialisation")), crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, payloadLength)));
let _ = crate::m_native__memory::storeU32(ctx, memory, (state + ctx.globals.m_native__abi__V_DATA.expect("global initialisation")), Numeric::Number(data));
let _ = crate::m_native__memory::storeU32(ctx, memory, (state + ctx.globals.m_native__abi__V_CAPACITY.expect("global initialisation")), Numeric::Number(payloadLength));
let _ = crate::m_native__memory::storeU64(ctx, memory, (state + ctx.globals.m_native__abi__V_ATIME.expect("global initialisation")), crate::m_native__memory::loadU64(ctx, memory, (cursor + 32.0f64)));
let _ = crate::m_native__memory::storeU64(ctx, memory, (state + ctx.globals.m_native__abi__V_MTIME.expect("global initialisation")), crate::m_native__memory::loadU64(ctx, memory, (cursor + 40.0f64)));
let _ = crate::m_native__memory::storeU64(ctx, memory, (state + ctx.globals.m_native__abi__V_CTIME.expect("global initialisation")), crate::m_native__memory::loadU64(ctx, memory, (cursor + 48.0f64)));
let record: f64 = (ctx.globals.m_native__vfs__VFS_INODE_IMAGE_SIZE.expect("global initialisation") + payloadLength);
let aligned: f64 = crate::m_native__vfs::align4(ctx, record);
if ((aligned < record) || (aligned > (end - cursor))) {
return crate::m_native__word::word(ctx, (-ctx.globals.m_native__abi__THISTLE_VFS_FORMAT.expect("global initialisation")));
}
let _ = { cursor = (cursor + aligned); cursor };
let _ = { parsed = (parsed + 1.0f64); parsed };
}
}
return crate::m_native__word::word(ctx, cursor);
}


pub fn loadDentries<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut start: f64, mut end: f64, mut dentryCount: f64, mut inodeCapacity: f64) -> i128 {
let mut cursor: f64 = start;
let mut parsed: f64 = 0.0f64;
'__crust_v14: while (parsed < dentryCount) {
{
if ((cursor > end) || (ctx.globals.m_native__vfs__VFS_DENTRY_IMAGE_SIZE.expect("global initialisation") > (end - cursor))) {
return crate::m_native__word::word(ctx, (-ctx.globals.m_native__abi__THISTLE_VFS_FORMAT.expect("global initialisation")));
}
let parent: f64 = crate::m_native__memory::loadU32(ctx, memory, cursor);
let inode: f64 = crate::m_native__memory::loadU32(ctx, memory, (cursor + 4.0f64));
let nameLength: f64 = crate::m_native__memory::loadU32(ctx, memory, (cursor + 8.0f64));
let nameAt: f64 = (cursor + ctx.globals.m_native__vfs__VFS_DENTRY_IMAGE_SIZE.expect("global initialisation"));
if ((((((((parent == 0.0f64) || (inode == 0.0f64)) || (parent >= inodeCapacity)) || (inode >= inodeCapacity)) || (crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__vfs::inodeAt(ctx, memory, parent) + ctx.globals.m_native__abi__V_KIND.expect("global initialisation"))) != ctx.globals.m_native__abi__THISTLE_VFS_KIND_DIRECTORY.expect("global initialisation"))) || (crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__vfs::inodeAt(ctx, memory, inode) + ctx.globals.m_native__abi__V_KIND.expect("global initialisation"))) == ctx.globals.m_native__abi__THISTLE_VFS_KIND_EMPTY.expect("global initialisation"))) || (nameLength > (end - nameAt))) || (!crate::m_native__vfs::validName(ctx, memory, nameAt, nameLength))) {
return crate::m_native__word::word(ctx, (-ctx.globals.m_native__abi__THISTLE_VFS_FORMAT.expect("global initialisation")));
}
let hash: f64 = crate::m_native__vfs::hashBytes(ctx, memory, nameAt, nameLength);
let mut scan: f64 = 0.0f64;
'__crust_v15: while (scan < parsed) {
{
let existing: f64 = crate::m_native__vfs::dentryAt(ctx, memory, scan);
if ((crate::m_native__memory::loadU32(ctx, memory, (existing + ctx.globals.m_native__abi__D_PARENT.expect("global initialisation"))) == parent) && crate::m_native__vfs::sameName(ctx, memory, existing, nameAt, nameLength, hash)) {
{
return crate::m_native__word::word(ctx, (-ctx.globals.m_native__abi__THISTLE_VFS_DUPLICATE.expect("global initialisation")));
}
}
let _ = { scan = (scan + 1.0f64); scan };
}
}
let name: f64 = crate::m_native__vfs::allocateData(ctx, memory, nameLength);
if (name == 4294967295.0f64) {
return crate::m_native__word::word(ctx, (-ctx.globals.m_native__abi__THISTLE_VFS_CAPACITY_ERROR.expect("global initialisation")));
}
let _ = crate::m_native__memory::copyMemory(ctx, memory, name, nameAt, nameLength);
let dentry: f64 = crate::m_native__vfs::dentryAt(ctx, memory, parsed);
let _ = crate::m_native__memory::storeU32(ctx, memory, (dentry + ctx.globals.m_native__abi__D_PARENT.expect("global initialisation")), Numeric::Number(parent));
let _ = crate::m_native__memory::storeU32(ctx, memory, (dentry + ctx.globals.m_native__abi__D_INODE.expect("global initialisation")), Numeric::Number(inode));
let _ = crate::m_native__memory::storeU32(ctx, memory, (dentry + ctx.globals.m_native__abi__D_NAME.expect("global initialisation")), Numeric::Number(name));
let _ = crate::m_native__memory::storeU32(ctx, memory, (dentry + ctx.globals.m_native__abi__D_NAME_LENGTH.expect("global initialisation")), Numeric::Number(nameLength));
let _ = crate::m_native__memory::storeU32(ctx, memory, (dentry + ctx.globals.m_native__abi__D_NAME_HASH.expect("global initialisation")), Numeric::Number(hash));
let _ = crate::m_native__memory::storeU32(ctx, memory, (dentry + ctx.globals.m_native__abi__D_FLAGS.expect("global initialisation")), Numeric::Number(0.0f64));
let record: f64 = (ctx.globals.m_native__vfs__VFS_DENTRY_IMAGE_SIZE.expect("global initialisation") + nameLength);
let aligned: f64 = crate::m_native__vfs::align4(ctx, record);
if ((aligned < record) || (aligned > (end - cursor))) {
return crate::m_native__word::word(ctx, (-ctx.globals.m_native__abi__THISTLE_VFS_FORMAT.expect("global initialisation")));
}
let _ = { cursor = (cursor + aligned); cursor };
let _ = { parsed = (parsed + 1.0f64); parsed };
}
}
return crate::m_native__word::word(ctx, cursor);
}


pub fn publishVfs<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut inodeCount: f64, mut dentryCount: f64, mut root: f64) -> () {
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_INODE_COUNT.expect("global initialisation")), Numeric::Number(inodeCount));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DENTRY_COUNT.expect("global initialisation")), Numeric::Number(dentryCount));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_ROOT.expect("global initialisation")), Numeric::Number(root));
let processBase: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PROCESS_TABLE_BASE.expect("global initialisation")));
let processCapacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PROCESS_CAPACITY.expect("global initialisation")));
let mut processIndex: f64 = 0.0f64;
'__crust_v16: while (processIndex < processCapacity) {
{
let proc: f64 = (processBase + (processIndex * ctx.globals.m_native__abi__THISTLE_PROCESS_STRIDE.expect("global initialisation")));
if ((crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_STATE.expect("global initialisation"))) != crate::m_native__word::wordToU32(ctx, crate::m_native__word::word(ctx, ctx.globals.m_native__abi__THISTLE_PROCESS_EMPTY.expect("global initialisation")))) && (crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_CWD_INODE.expect("global initialisation"))) == 0.0f64)) {
let _ = crate::m_native__memory::storeU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_CWD_INODE.expect("global initialisation")), Numeric::Number(root));
}
let _ = { processIndex = (processIndex + 1.0f64); processIndex };
}
}
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DIRTY.expect("global initialisation")), Numeric::Number(0.0f64));
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_GENERATION.expect("global initialisation")), (crate::m_native__memory::atomicLoadU64(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_GENERATION.expect("global initialisation")))).checked_add(1i128).expect("BigInt arithmetic"));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_LOADED.expect("global initialisation")), Numeric::Number(1.0f64));
}


pub fn thistleLoadVfs<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::ThistleMemory, mut image: f64, mut imageLength: f64) -> f64 {
if (((((!crate::m_native__image_range::inputContains(ctx, memory, image, imageLength)) || (imageLength < ctx.globals.m_native__vfs__VFS_HEADER_SIZE.expect("global initialisation"))) || (crate::m_native__memory::loadU32(ctx, memory, image) != ctx.globals.m_native__abi__THISTLE_VFS_IMAGE_MAGIC.expect("global initialisation"))) || (crate::m_native__memory::loadU32(ctx, memory, (image + 4.0f64)) != ctx.globals.m_native__abi__THISTLE_VFS_IMAGE_VERSION.expect("global initialisation"))) || (crate::m_native__memory::loadU32(ctx, memory, (image + 20.0f64)) != imageLength)) {
return ctx.globals.m_native__abi__THISTLE_VFS_RANGE.expect("global initialisation");
}
let inodeCount: f64 = crate::m_native__memory::loadU32(ctx, memory, (image + 8.0f64));
let dentryCount: f64 = crate::m_native__memory::loadU32(ctx, memory, (image + 12.0f64));
let root: f64 = crate::m_native__memory::loadU32(ctx, memory, (image + 16.0f64));
let inodeCapacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_INODE_CAPACITY.expect("global initialisation")));
let dentryCapacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DENTRY_CAPACITY.expect("global initialisation")));
let dataCapacity: f64 = (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DATA_END.expect("global initialisation"))) - crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__THISTLE_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DATA_BASE.expect("global initialisation"))));
if (((((inodeCount == 0.0f64) || (inodeCount >= inodeCapacity)) || (dentryCount > dentryCapacity)) || (root == 0.0f64)) || (root >= inodeCapacity)) {
return ctx.globals.m_native__abi__THISTLE_VFS_CAPACITY_ERROR.expect("global initialisation");
}
if (crate::m_native__vfs::hashBytes(ctx, memory, (image + ctx.globals.m_native__vfs__VFS_HEADER_SIZE.expect("global initialisation")), (imageLength - ctx.globals.m_native__vfs__VFS_HEADER_SIZE.expect("global initialisation"))) != crate::m_native__memory::loadU32(ctx, memory, (image + 24.0f64))) {
{
return ctx.globals.m_native__abi__THISTLE_VFS_CHECKSUM.expect("global initialisation");
}
}
let _ = crate::m_native__vfs::resetVfs(ctx, memory, inodeCapacity, dentryCapacity);
let end: f64 = (image + imageLength);
let inodeResult: i128 = crate::m_native__vfs::loadInodes(ctx, memory, (image + ctx.globals.m_native__vfs__VFS_HEADER_SIZE.expect("global initialisation")), end, inodeCount, inodeCapacity, dataCapacity);
if (inodeResult < 0i128) {
return crate::m_native__word::wordToI32(ctx, (inodeResult).checked_neg().expect("BigInt negation"));
}
if (crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__vfs::inodeAt(ctx, memory, root) + ctx.globals.m_native__abi__V_KIND.expect("global initialisation"))) != ctx.globals.m_native__abi__THISTLE_VFS_KIND_DIRECTORY.expect("global initialisation")) {
return ctx.globals.m_native__abi__THISTLE_VFS_FORMAT.expect("global initialisation");
}
let dentryResult: i128 = crate::m_native__vfs::loadDentries(ctx, memory, numbers::number(Numeric::BigInt(inodeResult)), end, dentryCount, inodeCapacity);
if (dentryResult < 0i128) {
return crate::m_native__word::wordToI32(ctx, (dentryResult).checked_neg().expect("BigInt negation"));
}
if (dentryResult != crate::m_native__word::word(ctx, end)) {
return ctx.globals.m_native__abi__THISTLE_VFS_FORMAT.expect("global initialisation");
}
let _ = crate::m_native__vfs::publishVfs(ctx, memory, inodeCount, dentryCount, root);
return ctx.globals.m_native__abi__THISTLE_VFS_OK.expect("global initialisation");
}

