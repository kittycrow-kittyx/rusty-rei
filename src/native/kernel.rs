use crate::*;

pub fn traceMappingOperation<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut operation: f64, mut address: i128, mut length: i128, mut options: f64, mut detail: i128, mut result: i128) -> () {
if (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_WORKER_CAPACITY.expect("global initialisation"))) > 32.0f64) {
return;
}
let mapped: i128 = (if ((operation == 222.0f64) && (result >= 0i128)) { crate::m_native__word::ux(ctx, result) } else { address });
let requested: bool = ((address < ctx.globals.m_native__kernel__MMAP_TRACE_END.expect("global initialisation")) && ((address).checked_add(length).expect("BigInt arithmetic") > ctx.globals.m_native__kernel__MMAP_TRACE_START.expect("global initialisation")));
let affected: bool = ((mapped < ctx.globals.m_native__kernel__MMAP_TRACE_END.expect("global initialisation")) && ((mapped).checked_add(length).expect("BigInt arithmetic") > ctx.globals.m_native__kernel__MMAP_TRACE_START.expect("global initialisation")));
if ((!requested) && (!affected)) {
return;
}
let count: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__kernel__MMAP_TRACE_COUNTER.expect("global initialisation")));
let slot: f64 = ((ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__kernel__MMAP_TRACE_BASE.expect("global initialisation")) + ((count % ctx.globals.m_native__kernel__MMAP_TRACE_CAPACITY.expect("global initialisation")) * ctx.globals.m_native__kernel__MMAP_TRACE_STRIDE.expect("global initialisation")));
let _ = crate::m_native__memory::storeU32(ctx, memory, slot, Numeric::Number(operation));
let _ = crate::m_native__memory::storeU32(ctx, memory, (slot + 4.0f64), Numeric::Number(options));
let _ = crate::m_native__memory::storeU64(ctx, memory, (slot + 8.0f64), address);
let _ = crate::m_native__memory::storeU64(ctx, memory, (slot + 16.0f64), length);
let _ = crate::m_native__memory::storeI64(ctx, memory, (slot + 24.0f64), result);
let _ = crate::m_native__memory::storeU64(ctx, memory, (slot + 32.0f64), detail);
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__kernel__MMAP_TRACE_COUNTER.expect("global initialisation")), Numeric::Number((count + 1.0f64)));
}


pub fn traceNodePointerWrite<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut address: i128, mut value: i128, mut size: f64) -> () {
if (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_WORKER_CAPACITY.expect("global initialisation"))) > 32.0f64) {
return;
}
let top: i128 = crate::m_native__memory::loadU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_VIRTUAL_TOP.expect("global initialisation")));
if (((top < 65536i128) || (address < (top).checked_sub(65536i128).expect("BigInt arithmetic"))) || (address >= top)) {
return;
}
let low: i128 = (value & 4294967295i128);
let high: i128 = numbers::big_shr(value, 32i128);
let fullSmi: bool = ((((size == 8.0f64) && (low == 0i128)) && (high > 0i128)) && (high <= 16i128));
let upperSmi: bool = ((((size == 4.0f64) && (((address & 7i128)) == 4i128)) && (low > 0i128)) && (low <= 16i128));
if ((!fullSmi) && (!upperSmi)) {
return;
}
let count: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__kernel__MMAP_TRACE_COUNTER.expect("global initialisation")));
let slot: f64 = ((ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__kernel__MMAP_TRACE_BASE.expect("global initialisation")) + ((count % ctx.globals.m_native__kernel__MMAP_TRACE_CAPACITY.expect("global initialisation")) * ctx.globals.m_native__kernel__MMAP_TRACE_STRIDE.expect("global initialisation")));
let _ = crate::m_native__memory::storeU32(ctx, memory, slot, Numeric::Number(1398034258.0f64));
let _ = crate::m_native__memory::storeU32(ctx, memory, (slot + 4.0f64), Numeric::Number(size));
let _ = crate::m_native__memory::storeU64(ctx, memory, (slot + 8.0f64), address);
let _ = crate::m_native__memory::storeU64(ctx, memory, (slot + 16.0f64), crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, size)));
let _ = crate::m_native__memory::storeI64(ctx, memory, (slot + 24.0f64), crate::m_native__word::sx(ctx, value, 64.0f64));
let _ = crate::m_native__memory::storeU64(ctx, memory, (slot + 32.0f64), crate::m_native__memory::loadU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_PC.expect("global initialisation"))));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__kernel__MMAP_TRACE_COUNTER.expect("global initialisation")), Numeric::Number((count + 1.0f64)));
}


pub fn hartAt<P: Platform>(ctx: &Context<P>, mut hart: f64) -> f64 {
return (ctx.globals.m_native__abi__TETO_HART_BASE.expect("global initialisation") + (hart * ctx.globals.m_native__abi__TETO_HART_STRIDE.expect("global initialisation")));
}


pub fn regAt<P: Platform>(ctx: &Context<P>, mut hart: f64, mut register: f64) -> f64 {
return ((crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_X.expect("global initialisation")) + (register * 8.0f64));
}


pub fn floatRegAt<P: Platform>(ctx: &Context<P>, mut hart: f64, mut register: f64) -> f64 {
return ((crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_F.expect("global initialisation")) + (register * 8.0f64));
}


pub fn pageAlign<P: Platform>(ctx: &Context<P>, mut value: f64) -> f64 {
return numbers::and((((value + ctx.globals.m_native__abi__TETO_GUEST_PAGE_SIZE.expect("global initialisation")) - 1.0f64)), (-ctx.globals.m_native__abi__TETO_GUEST_PAGE_SIZE.expect("global initialisation")));
}


pub fn processAt<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut slot: f64) -> f64 {
return (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PROCESS_TABLE_BASE.expect("global initialisation"))) + (slot * ctx.globals.m_native__abi__TETO_PROCESS_STRIDE.expect("global initialisation")));
}


pub fn processForHart<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> f64 {
let slot: f64 = crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_PROCESS_SLOT.expect("global initialisation")));
return (if (slot < crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PROCESS_CAPACITY.expect("global initialisation")))) { crate::m_native__kernel::processAt(ctx, memory, slot) } else { ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation") });
}


pub fn sharedProcessThread<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> bool {
if (hart == 0.0f64) {
return false;
}
let slot: f64 = crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_PROCESS_SLOT.expect("global initialisation")));
return ((slot != ctx.globals.m_native__kernel__INVALID_PROCESS.expect("global initialisation")) && (slot == crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, 0.0f64) + ctx.globals.m_native__abi__H_PROCESS_SLOT.expect("global initialisation")))));
}


pub fn descriptorAt<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut proc: f64, mut descriptor: f64) -> f64 {
return (crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_DESCRIPTOR_TABLE.expect("global initialisation"))) + (descriptor * ctx.globals.m_native__abi__TETO_DESCRIPTOR_STRIDE.expect("global initialisation")));
}


pub fn workerAt<P: Platform>(ctx: &Context<P>, mut worker: f64) -> f64 {
return (ctx.globals.m_native__abi__C_WORKER_BASE.expect("global initialisation") + (worker * ctx.globals.m_native__abi__TETO_WORKER_STRIDE.expect("global initialisation")));
}


pub fn validHart<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> bool {
return (hart < crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_MAX_HARTS.expect("global initialisation"))));
}


pub fn bump<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64) -> () {
let _ = crate::m_native__memory::atomicAddU64(ctx, memory, at, 1i128);
}


pub fn acquire<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64) -> bool {
return (crate::m_native__memory::compareExchangeI32(ctx, memory, at, 0.0f64, 1.0f64) == 0.0f64);
}


pub fn release<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64) -> () {
return crate::m_native__memory::atomicStoreI32(ctx, memory, at, 0.0f64);
}


pub fn tetoKernelInit<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut maxHarts: f64, mut threaded: bool) -> f64 {
if ((maxHarts < 1.0f64) || (maxHarts > 1024.0f64)) {
return ctx.globals.m_native__abi__TETO_FAULT_BAD_STATE.expect("global initialisation");
}
let hartEnd: f64 = (ctx.globals.m_native__abi__TETO_HART_BASE.expect("global initialisation") + (maxHarts * ctx.globals.m_native__abi__TETO_HART_STRIDE.expect("global initialisation")));
let processCapacity: f64 = (if (maxHarts > 32.0f64) { 256.0f64 } else { (maxHarts * 8.0f64) });
let pageTables: f64 = crate::m_native__kernel::pageAlign(ctx, hartEnd);
let tableBytes: f64 = ((processCapacity * ctx.globals.m_native__abi__TETO_PAGE_CAPACITY.expect("global initialisation")) * ctx.globals.m_native__abi__TETO_PAGE_ENTRY_SIZE.expect("global initialisation"));
let segmentTables: f64 = crate::m_native__kernel::pageAlign(ctx, (pageTables + tableBytes));
let segmentBytes: f64 = ((processCapacity * ctx.globals.m_native__abi__TETO_SEGMENT_CAPACITY.expect("global initialisation")) * ctx.globals.m_native__abi__TETO_SEGMENT_STRIDE.expect("global initialisation"));
let mapTables: f64 = crate::m_native__kernel::pageAlign(ctx, (segmentTables + segmentBytes));
let mapBytes: f64 = ((processCapacity * ctx.globals.m_native__abi__TETO_MAP_CAPACITY.expect("global initialisation")) * ctx.globals.m_native__abi__TETO_MAP_STRIDE.expect("global initialisation"));
let vfsScratch: f64 = crate::m_native__kernel::pageAlign(ctx, (mapTables + mapBytes));
let inodeTable: f64 = crate::m_native__kernel::pageAlign(ctx, (vfsScratch + ctx.globals.m_native__abi__TETO_VFS_PATH_CAPACITY.expect("global initialisation")));
let dentryTable: f64 = crate::m_native__kernel::pageAlign(ctx, (inodeTable + (ctx.globals.m_native__abi__TETO_VFS_INODE_CAPACITY.expect("global initialisation") * ctx.globals.m_native__abi__TETO_VFS_INODE_STRIDE.expect("global initialisation"))));
let descriptorTables: f64 = crate::m_native__kernel::pageAlign(ctx, (dentryTable + (ctx.globals.m_native__abi__TETO_VFS_DENTRY_CAPACITY.expect("global initialisation") * ctx.globals.m_native__abi__TETO_VFS_DENTRY_STRIDE.expect("global initialisation"))));
let descriptorBytes: f64 = ((processCapacity * ctx.globals.m_native__abi__TETO_DESCRIPTOR_CAPACITY.expect("global initialisation")) * ctx.globals.m_native__abi__TETO_DESCRIPTOR_STRIDE.expect("global initialisation"));
let processTable: f64 = crate::m_native__kernel::pageAlign(ctx, (descriptorTables + descriptorBytes));
let vfsData: f64 = crate::m_native__kernel::pageAlign(ctx, (processTable + (processCapacity * ctx.globals.m_native__abi__TETO_PROCESS_STRIDE.expect("global initialisation"))));
let physical: f64 = crate::m_native__kernel::pageAlign(ctx, (vfsData + ctx.globals.m_native__abi__TETO_VFS_DATA_CAPACITY.expect("global initialisation")));
let total: f64 = crate::m_native__memory::memorySize(ctx, memory);
if (((physical < pageTables) || (physical > total)) || ((total - physical) < ctx.globals.m_native__abi__TETO_GUEST_PAGE_SIZE.expect("global initialisation"))) {
return ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation");
}
let _ = crate::m_native__memory::fill(ctx, memory, ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation"), physical, 0.0f64);
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_MAGIC.expect("global initialisation")), Numeric::Number(ctx.globals.m_native__abi__TETO_MAGIC.expect("global initialisation")));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_ABI.expect("global initialisation")), Numeric::Number(ctx.globals.m_native__abi__TETO_ABI_VERSION.expect("global initialisation")));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_FLAGS.expect("global initialisation")), (if threaded { Numeric::Number(ctx.globals.m_native__abi__TETO_FLAG_THREADED.expect("global initialisation")) } else { Numeric::Number(0.0f64) }));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_MAX_HARTS.expect("global initialisation")), Numeric::Number(maxHarts));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PAGE_TABLE_BASE.expect("global initialisation")), Numeric::Number(pageTables));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PAGE_CAPACITY.expect("global initialisation")), Numeric::Number(ctx.globals.m_native__abi__TETO_PAGE_CAPACITY.expect("global initialisation")));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_SEGMENT_TABLE_BASE.expect("global initialisation")), Numeric::Number(segmentTables));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_SEGMENT_CAPACITY.expect("global initialisation")), Numeric::Number(ctx.globals.m_native__abi__TETO_SEGMENT_CAPACITY.expect("global initialisation")));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_MAP_TABLE_BASE.expect("global initialisation")), Numeric::Number(mapTables));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_MAP_CAPACITY.expect("global initialisation")), Numeric::Number(ctx.globals.m_native__abi__TETO_MAP_CAPACITY.expect("global initialisation")));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_SCRATCH_BASE.expect("global initialisation")), Numeric::Number(vfsScratch));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_INODE_BASE.expect("global initialisation")), Numeric::Number(inodeTable));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_INODE_CAPACITY.expect("global initialisation")), Numeric::Number(ctx.globals.m_native__abi__TETO_VFS_INODE_CAPACITY.expect("global initialisation")));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DENTRY_BASE.expect("global initialisation")), Numeric::Number(dentryTable));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DENTRY_CAPACITY.expect("global initialisation")), Numeric::Number(ctx.globals.m_native__abi__TETO_VFS_DENTRY_CAPACITY.expect("global initialisation")));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DATA_BASE.expect("global initialisation")), Numeric::Number(vfsData));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DATA_END.expect("global initialisation")), Numeric::Number((vfsData + ctx.globals.m_native__abi__TETO_VFS_DATA_CAPACITY.expect("global initialisation"))));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DATA_NEXT.expect("global initialisation")), Numeric::Number(vfsData));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_INODE_COUNT.expect("global initialisation")), Numeric::Number(0.0f64));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DENTRY_COUNT.expect("global initialisation")), Numeric::Number(0.0f64));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_ROOT.expect("global initialisation")), Numeric::Number(0.0f64));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_LOADED.expect("global initialisation")), Numeric::Number(0.0f64));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_DIRTY.expect("global initialisation")), Numeric::Number(0.0f64));
let _ = crate::m_native__memory::storeU64(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_GENERATION.expect("global initialisation")), 0i128);
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_DESCRIPTOR_TABLE_BASE.expect("global initialisation")), Numeric::Number(descriptorTables));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_DESCRIPTOR_CAPACITY.expect("global initialisation")), Numeric::Number(ctx.globals.m_native__abi__TETO_DESCRIPTOR_CAPACITY.expect("global initialisation")));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PHYSICAL_NEXT.expect("global initialisation")), Numeric::Number(physical));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PHYSICAL_END.expect("global initialisation")), Numeric::Number(total));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_IMAGE_TOP.expect("global initialisation")), Numeric::Number(total));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_IMAGE_SIZE.expect("global initialisation")), Numeric::Number(0.0f64));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PROCESS_TABLE_BASE.expect("global initialisation")), Numeric::Number(processTable));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PROCESS_CAPACITY.expect("global initialisation")), Numeric::Number(processCapacity));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PROCESS_COUNT.expect("global initialisation")), Numeric::Number(0.0f64));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_NEXT_PID.expect("global initialisation")), Numeric::Number(2.0f64));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_SCHED_CURSOR.expect("global initialisation")), Numeric::Number(0.0f64));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_WORKER_CAPACITY.expect("global initialisation")), (if (maxHarts > 128.0f64) { Numeric::Number(128.0f64) } else { Numeric::Number(maxHarts) }));
return 0.0f64;
}


pub fn tetoKernelValid<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory) -> bool {
return ((crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_MAGIC.expect("global initialisation"))) == ctx.globals.m_native__abi__TETO_MAGIC.expect("global initialisation")) && (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_ABI.expect("global initialisation"))) == ctx.globals.m_native__abi__TETO_ABI_VERSION.expect("global initialisation")));
}


pub fn tetoHartInit<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut virtualTop: i128, mut pc: i128) -> f64 {
if ((!crate::m_native__kernel::tetoKernelValid(ctx, memory)) || (!crate::m_native__kernel::validHart(ctx, memory, hart))) {
return ctx.globals.m_native__abi__TETO_FAULT_BAD_STATE.expect("global initialisation");
}
if (((virtualTop < (1024i128).checked_mul(1024i128).expect("BigInt arithmetic")) || (virtualTop > 281474976710655i128)) || (pc >= virtualTop)) {
return ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation");
}
let at: f64 = crate::m_native__kernel::hartAt(ctx, hart);
let capacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PAGE_CAPACITY.expect("global initialisation")));
let _ = crate::m_native__memory::fill(ctx, memory, at, ctx.globals.m_native__abi__TETO_HART_STRIDE.expect("global initialisation"), 0.0f64);
let _ = crate::m_native__memory::storeU64(ctx, memory, (at + ctx.globals.m_native__abi__H_VIRTUAL_TOP.expect("global initialisation")), virtualTop);
let _ = crate::m_native__memory::storeU32(ctx, memory, (at + ctx.globals.m_native__abi__H_PAGE_TABLE.expect("global initialisation")), Numeric::Number(ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")));
let _ = crate::m_native__memory::storeU32(ctx, memory, (at + ctx.globals.m_native__abi__H_PAGE_CAPACITY.expect("global initialisation")), Numeric::Number(capacity));
let _ = crate::m_native__memory::storeU32(ctx, memory, (at + ctx.globals.m_native__abi__H_PAGE_COUNT.expect("global initialisation")), Numeric::Number(0.0f64));
let _ = crate::m_native__memory::storeU64(ctx, memory, (at + ctx.globals.m_native__abi__H_PC.expect("global initialisation")), pc);
let _ = crate::m_native__memory::storeI64(ctx, memory, (at + ctx.globals.m_native__abi__H_RESERVATION.expect("global initialisation")), ctx.globals.m_native__kernel__NO_RESERVATION.expect("global initialisation"));
let _ = crate::m_native__memory::storeI32(ctx, memory, (at + ctx.globals.m_native__abi__H_LAST_WORKER.expect("global initialisation")), (-1.0f64));
let _ = crate::m_native__memory::storeU32(ctx, memory, (at + ctx.globals.m_native__abi__H_PROCESS_SLOT.expect("global initialisation")), Numeric::Number(ctx.globals.m_native__kernel__INVALID_PROCESS.expect("global initialisation")));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (at + ctx.globals.m_native__abi__H_STATUS.expect("global initialisation")), ctx.globals.m_native__abi__TETO_HART_RUNNABLE.expect("global initialisation"));
return 0.0f64;
}


pub fn tetoProcessInit<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut pid: f64, mut ppid: f64, mut ruid: f64, mut euid: f64, mut suid: f64, mut rgid: f64, mut egid: f64, mut sgid: f64) -> f64 {
if ((!crate::m_native__kernel::validHart(ctx, memory, hart)) || (pid < 1.0f64)) {
return ctx.globals.m_native__abi__TETO_FAULT_BAD_STATE.expect("global initialisation");
}
let lock: f64 = (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PROCESS_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, lock)) {
return ctx.globals.m_native__abi__TETO_BATCH_CONTENDED.expect("global initialisation");
}
let capacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PROCESS_CAPACITY.expect("global initialisation")));
let mut slot: f64 = ctx.globals.m_native__kernel__INVALID_PROCESS.expect("global initialisation");
let mut index: f64 = 0.0f64;
'__crust_v17: while (index < capacity) {
{
let proc: f64 = crate::m_native__kernel::processAt(ctx, memory, index);
let state: f64 = crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_STATE.expect("global initialisation")));
if ((state != ctx.globals.m_native__abi__TETO_PROCESS_EMPTY.expect("global initialisation")) && (crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PID.expect("global initialisation"))) == pid)) {
{
let _ = crate::m_native__kernel::release(ctx, memory, lock);
return ctx.globals.m_native__abi__TETO_FAULT_BAD_STATE.expect("global initialisation");
}
}
if ((state == ctx.globals.m_native__abi__TETO_PROCESS_EMPTY.expect("global initialisation")) && (slot == ctx.globals.m_native__kernel__INVALID_PROCESS.expect("global initialisation"))) {
let _ = { slot = index; slot };
}
let _ = { index = (index + 1.0f64); index };
}
}
if (slot == ctx.globals.m_native__kernel__INVALID_PROCESS.expect("global initialisation")) {
{
let _ = crate::m_native__kernel::release(ctx, memory, lock);
return ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation");
}
}
let proc: f64 = crate::m_native__kernel::processAt(ctx, memory, slot);
let pageCapacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PAGE_CAPACITY.expect("global initialisation")));
let pageTable: f64 = (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PAGE_TABLE_BASE.expect("global initialisation"))) + ((slot * pageCapacity) * ctx.globals.m_native__abi__TETO_PAGE_ENTRY_SIZE.expect("global initialisation")));
let segmentCapacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_SEGMENT_CAPACITY.expect("global initialisation")));
let segmentTable: f64 = (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_SEGMENT_TABLE_BASE.expect("global initialisation"))) + ((slot * segmentCapacity) * ctx.globals.m_native__abi__TETO_SEGMENT_STRIDE.expect("global initialisation")));
let mapCapacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_MAP_CAPACITY.expect("global initialisation")));
let mapTable: f64 = (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_MAP_TABLE_BASE.expect("global initialisation"))) + ((slot * mapCapacity) * ctx.globals.m_native__abi__TETO_MAP_STRIDE.expect("global initialisation")));
let descriptorCapacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_DESCRIPTOR_CAPACITY.expect("global initialisation")));
let descriptorTable: f64 = (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_DESCRIPTOR_TABLE_BASE.expect("global initialisation"))) + ((slot * descriptorCapacity) * ctx.globals.m_native__abi__TETO_DESCRIPTOR_STRIDE.expect("global initialisation")));
let virtualTop: i128 = crate::m_native__memory::loadU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_VIRTUAL_TOP.expect("global initialisation")));
let _ = crate::m_native__memory::fill(ctx, memory, proc, ctx.globals.m_native__abi__TETO_PROCESS_STRIDE.expect("global initialisation"), 0.0f64);
let _ = crate::m_native__memory::fill(ctx, memory, pageTable, (pageCapacity * ctx.globals.m_native__abi__TETO_PAGE_ENTRY_SIZE.expect("global initialisation")), 0.0f64);
let _ = crate::m_native__memory::fill(ctx, memory, segmentTable, (segmentCapacity * ctx.globals.m_native__abi__TETO_SEGMENT_STRIDE.expect("global initialisation")), 0.0f64);
let _ = crate::m_native__memory::fill(ctx, memory, mapTable, (mapCapacity * ctx.globals.m_native__abi__TETO_MAP_STRIDE.expect("global initialisation")), 0.0f64);
let _ = crate::m_native__memory::fill(ctx, memory, descriptorTable, (descriptorCapacity * ctx.globals.m_native__abi__TETO_DESCRIPTOR_STRIDE.expect("global initialisation")), 0.0f64);
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PID.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, pid)));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PPID.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, ppid)));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PGID.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, pid)));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_RUID.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, ruid)));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_EUID.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, euid)));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_SUID.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, suid)));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_FSUID.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, euid)));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_RGID.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, rgid)));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_EGID.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, egid)));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_SGID.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, sgid)));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_FSGID.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, egid)));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PAGE_TABLE.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, pageTable)));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PAGE_CAPACITY.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, pageCapacity)));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PAGE_COUNT.expect("global initialisation")), 0.0f64);
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_VIRTUAL_TOP.expect("global initialisation")), virtualTop);
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_SEGMENT_TABLE.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, segmentTable)));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_SEGMENT_COUNT.expect("global initialisation")), 0.0f64);
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_TABLE.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, mapTable)));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation")), 0.0f64);
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_CAPACITY.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, mapCapacity)));
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_NEXT.expect("global initialisation")), 274877906944i128);
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_CWD_INODE.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, crate::m_native__vfs::tetoVfsRoot(ctx, memory))));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_UMASK.expect("global initialisation")), 18.0f64);
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_DESCRIPTOR_TABLE.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, descriptorTable)));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_DESCRIPTOR_CAPACITY.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, descriptorCapacity)));
let _ = crate::m_native__memory::storeU32(ctx, memory, (crate::m_native__kernel::descriptorAt(ctx, memory, proc, 0.0f64) + ctx.globals.m_native__abi__FD_KIND.expect("global initialisation")), Numeric::Number(ctx.globals.m_native__abi__TETO_FD_HOST.expect("global initialisation")));
let _ = crate::m_native__memory::storeU32(ctx, memory, (crate::m_native__kernel::descriptorAt(ctx, memory, proc, 0.0f64) + ctx.globals.m_native__abi__FD_FLAGS.expect("global initialisation")), Numeric::Number(ctx.globals.m_native__abi__TETO_FD_READ.expect("global initialisation")));
let _ = crate::m_native__memory::storeU32(ctx, memory, (crate::m_native__kernel::descriptorAt(ctx, memory, proc, 1.0f64) + ctx.globals.m_native__abi__FD_KIND.expect("global initialisation")), Numeric::Number(ctx.globals.m_native__abi__TETO_FD_HOST.expect("global initialisation")));
let _ = crate::m_native__memory::storeU32(ctx, memory, (crate::m_native__kernel::descriptorAt(ctx, memory, proc, 1.0f64) + ctx.globals.m_native__abi__FD_FLAGS.expect("global initialisation")), Numeric::Number(ctx.globals.m_native__abi__TETO_FD_WRITE.expect("global initialisation")));
let _ = crate::m_native__memory::storeU32(ctx, memory, (crate::m_native__kernel::descriptorAt(ctx, memory, proc, 2.0f64) + ctx.globals.m_native__abi__FD_KIND.expect("global initialisation")), Numeric::Number(ctx.globals.m_native__abi__TETO_FD_HOST.expect("global initialisation")));
let _ = crate::m_native__memory::storeU32(ctx, memory, (crate::m_native__kernel::descriptorAt(ctx, memory, proc, 2.0f64) + ctx.globals.m_native__abi__FD_FLAGS.expect("global initialisation")), Numeric::Number(ctx.globals.m_native__abi__TETO_FD_WRITE.expect("global initialisation")));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_STATE.expect("global initialisation")), ctx.globals.m_native__abi__TETO_PROCESS_RUNNABLE.expect("global initialisation"));
let _ = crate::m_native__memory::storeU32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_PROCESS_SLOT.expect("global initialisation")), Numeric::Number(slot));
let _ = crate::m_native__memory::storeU32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_PAGE_TABLE.expect("global initialisation")), Numeric::Number(pageTable));
let _ = crate::m_native__memory::storeU32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_PAGE_CAPACITY.expect("global initialisation")), Numeric::Number(pageCapacity));
let _ = crate::m_native__memory::storeU32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_PAGE_COUNT.expect("global initialisation")), Numeric::Number(0.0f64));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PROCESS_COUNT.expect("global initialisation")), Numeric::Number((crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PROCESS_COUNT.expect("global initialisation"))) + 1.0f64)));
if (pid >= crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_NEXT_PID.expect("global initialisation")))) {
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_NEXT_PID.expect("global initialisation")), Numeric::Number((pid + 1.0f64)));
}
let _ = crate::m_native__kernel::release(ctx, memory, lock);
return 0.0f64;
}


pub fn tetoProcessSetGroup<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut index: f64, mut gid: f64) -> f64 {
if ((!crate::m_native__kernel::validHart(ctx, memory, hart)) || (index >= ctx.globals.m_native__abi__TETO_GROUP_CAPACITY.expect("global initialisation"))) {
return ctx.globals.m_native__abi__TETO_FAULT_BAD_STATE.expect("global initialisation");
}
let proc: f64 = crate::m_native__kernel::processForHart(ctx, memory, hart);
if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return ctx.globals.m_native__abi__TETO_FAULT_BAD_STATE.expect("global initialisation");
}
let lock: f64 = (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_CREDENTIAL_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, lock)) {
return ctx.globals.m_native__abi__TETO_BATCH_CONTENDED.expect("global initialisation");
}
let count: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_GROUP_COUNT.expect("global initialisation")));
if (index > count) {
{
let _ = crate::m_native__kernel::release(ctx, memory, lock);
return ctx.globals.m_native__abi__TETO_FAULT_BAD_STATE.expect("global initialisation");
}
}
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, ((proc + ctx.globals.m_native__abi__P_GROUPS.expect("global initialisation")) + (index * 4.0f64)), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, gid)));
if (index == count) {
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_GROUP_COUNT.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, (count + 1.0f64))));
}
let _ = crate::m_native__kernel::release(ctx, memory, lock);
return 0.0f64;
}


pub fn tetoProcessCount<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory) -> f64 {
return (if crate::m_native__kernel::tetoKernelValid(ctx, memory) { crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PROCESS_COUNT.expect("global initialisation"))) } else { 0.0f64 });
}


pub fn tetoResolvePath<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut start: f64, mut path: f64, mut pathLength: f64, mut followFinal: bool) -> f64 {
if ((!crate::m_native__kernel::tetoKernelValid(ctx, memory)) || (!crate::m_native__kernel::validHart(ctx, memory, hart))) {
return (-22.0f64);
}
let proc: f64 = crate::m_native__kernel::processForHart(ctx, memory, hart);
if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return (-3.0f64);
}
let lock: f64 = (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, lock)) {
return (-11.0f64);
}
let result: f64 = crate::m_native__vfs::tetoVfsResolve(ctx, memory, proc, start, path, pathLength, followFinal);
let _ = crate::m_native__kernel::release(ctx, memory, lock);
return result;
}


pub fn tetoAccessInode<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut inode: f64, mut bits: f64) -> f64 {
if ((!crate::m_native__kernel::tetoKernelValid(ctx, memory)) || (!crate::m_native__kernel::validHart(ctx, memory, hart))) {
return (-22.0f64);
}
let proc: f64 = crate::m_native__kernel::processForHart(ctx, memory, hart);
if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return (-3.0f64);
}
let lock: f64 = (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, lock)) {
return (-11.0f64);
}
let result: f64 = (if crate::m_native__vfs::tetoVfsAccess(ctx, memory, proc, inode, bits) { 1.0f64 } else { 0.0f64 });
let _ = crate::m_native__kernel::release(ctx, memory, lock);
return result;
}


pub fn tetoOpenPath<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut start: f64, mut path: f64, mut pathLength: f64, mut flags: f64) -> f64 {
if (((!crate::m_native__kernel::tetoKernelValid(ctx, memory)) || (!crate::m_native__kernel::validHart(ctx, memory, hart))) || ((numbers::and(flags, 3.0f64)) != 0.0f64)) {
return (-30.0f64);
}
if ((numbers::and(flags, numbers::not((numbers::or(ctx.globals.m_native__kernel__OPEN_DIRECTORY.expect("global initialisation"), ctx.globals.m_native__kernel__OPEN_CLOEXEC.expect("global initialisation")))))) != 0.0f64) {
return (-22.0f64);
}
let proc: f64 = crate::m_native__kernel::processForHart(ctx, memory, hart);
if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return (-3.0f64);
}
if (!crate::m_native__kernel::acquire(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")))) {
return (-11.0f64);
}
let vfsLock: f64 = (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, vfsLock)) {
{
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return (-11.0f64);
}
}
let base: f64 = (if (start == 0.0f64) { crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_CWD_INODE.expect("global initialisation"))) } else { start });
let resolved: f64 = crate::m_native__vfs::tetoVfsResolve(ctx, memory, proc, base, path, pathLength, true);
if (resolved < 0.0f64) {
{
let _ = crate::m_native__kernel::release(ctx, memory, vfsLock);
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return resolved;
}
}
let inode: f64 = crate::m_native__word::wordToU32(ctx, crate::m_native__word::word(ctx, resolved));
let kind: f64 = crate::m_native__vfs::tetoVfsKind(ctx, memory, inode);
if (((numbers::and(flags, ctx.globals.m_native__kernel__OPEN_DIRECTORY.expect("global initialisation"))) != 0.0f64) && (kind != ctx.globals.m_native__abi__TETO_VFS_KIND_DIRECTORY.expect("global initialisation"))) {
{
let _ = crate::m_native__kernel::release(ctx, memory, vfsLock);
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return (-20.0f64);
}
}
if ((kind != ctx.globals.m_native__abi__TETO_VFS_KIND_FILE.expect("global initialisation")) && (kind != ctx.globals.m_native__abi__TETO_VFS_KIND_DIRECTORY.expect("global initialisation"))) {
{
let _ = crate::m_native__kernel::release(ctx, memory, vfsLock);
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return (-22.0f64);
}
}
if (!crate::m_native__vfs::tetoVfsAccess(ctx, memory, proc, inode, (if (kind == ctx.globals.m_native__abi__TETO_VFS_KIND_DIRECTORY.expect("global initialisation")) { 5.0f64 } else { 4.0f64 }))) {
{
let _ = crate::m_native__kernel::release(ctx, memory, vfsLock);
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return (-13.0f64);
}
}
let capacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_DESCRIPTOR_CAPACITY.expect("global initialisation")));
let mut descriptor: f64 = 3.0f64;
'__crust_v18: while ((descriptor < capacity) && (crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__kernel::descriptorAt(ctx, memory, proc, descriptor) + ctx.globals.m_native__abi__FD_KIND.expect("global initialisation"))) != ctx.globals.m_native__abi__TETO_FD_EMPTY.expect("global initialisation"))) {
{
let _ = { descriptor = (descriptor + 1.0f64); descriptor };
}
}
if (descriptor == capacity) {
{
let _ = crate::m_native__kernel::release(ctx, memory, vfsLock);
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return (-24.0f64);
}
}
let state: f64 = crate::m_native__kernel::descriptorAt(ctx, memory, proc, descriptor);
let _ = crate::m_native__memory::fill(ctx, memory, state, ctx.globals.m_native__abi__TETO_DESCRIPTOR_STRIDE.expect("global initialisation"), 0.0f64);
let _ = crate::m_native__memory::storeU32(ctx, memory, (state + ctx.globals.m_native__abi__FD_KIND.expect("global initialisation")), (if (kind == ctx.globals.m_native__abi__TETO_VFS_KIND_DIRECTORY.expect("global initialisation")) { Numeric::Number(ctx.globals.m_native__abi__TETO_FD_DIRECTORY.expect("global initialisation")) } else { Numeric::Number(ctx.globals.m_native__abi__TETO_FD_FILE.expect("global initialisation")) }));
let _ = crate::m_native__memory::storeU32(ctx, memory, (state + ctx.globals.m_native__abi__FD_FLAGS.expect("global initialisation")), Numeric::Number(numbers::or(ctx.globals.m_native__abi__TETO_FD_READ.expect("global initialisation"), ((if ((numbers::and(flags, ctx.globals.m_native__kernel__OPEN_CLOEXEC.expect("global initialisation"))) != 0.0f64) { ctx.globals.m_native__abi__TETO_FD_CLOEXEC.expect("global initialisation") } else { 0.0f64 })))));
let _ = crate::m_native__memory::storeU32(ctx, memory, (state + ctx.globals.m_native__abi__FD_INODE.expect("global initialisation")), Numeric::Number(inode));
let _ = crate::m_native__memory::storeU64(ctx, memory, (state + ctx.globals.m_native__abi__FD_OFFSET.expect("global initialisation")), 0i128);
let _ = crate::m_native__memory::storeU32(ctx, memory, (state + ctx.globals.m_native__abi__FD_CURSOR.expect("global initialisation")), Numeric::Number(0.0f64));
let _ = crate::m_native__kernel::release(ctx, memory, vfsLock);
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, descriptor));
}


pub fn tetoReadDescriptor<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut descriptor: f64, mut output: f64, mut length: f64) -> f64 {
if (((!crate::m_native__kernel::tetoKernelValid(ctx, memory)) || (!crate::m_native__kernel::validHart(ctx, memory, hart))) || (length > 1048576.0f64)) {
return (-22.0f64);
}
let proc: f64 = crate::m_native__kernel::processForHart(ctx, memory, hart);
if ((proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (descriptor >= crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_DESCRIPTOR_CAPACITY.expect("global initialisation"))))) {
return (-9.0f64);
}
if (!crate::m_native__kernel::acquire(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")))) {
return (-11.0f64);
}
let vfsLock: f64 = (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, vfsLock)) {
{
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return (-11.0f64);
}
}
let state: f64 = crate::m_native__kernel::descriptorAt(ctx, memory, proc, descriptor);
let kind: f64 = crate::m_native__memory::loadU32(ctx, memory, (state + ctx.globals.m_native__abi__FD_KIND.expect("global initialisation")));
if (kind == ctx.globals.m_native__abi__TETO_FD_DIRECTORY.expect("global initialisation")) {
{
let _ = crate::m_native__kernel::release(ctx, memory, vfsLock);
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return (-21.0f64);
}
}
if ((kind != ctx.globals.m_native__abi__TETO_FD_FILE.expect("global initialisation")) || ((numbers::and(crate::m_native__memory::loadU32(ctx, memory, (state + ctx.globals.m_native__abi__FD_FLAGS.expect("global initialisation"))), ctx.globals.m_native__abi__TETO_FD_READ.expect("global initialisation"))) == 0.0f64)) {
{
let _ = crate::m_native__kernel::release(ctx, memory, vfsLock);
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return (-9.0f64);
}
}
if (length == 0.0f64) {
{
let _ = crate::m_native__kernel::release(ctx, memory, vfsLock);
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return 0.0f64;
}
}
let amount: f64 = crate::m_native__vfs::tetoVfsReadData(ctx, memory, crate::m_native__memory::loadU32(ctx, memory, (state + ctx.globals.m_native__abi__FD_INODE.expect("global initialisation"))), crate::m_native__memory::loadU64(ctx, memory, (state + ctx.globals.m_native__abi__FD_OFFSET.expect("global initialisation"))), output, length);
if (amount > 0.0f64) {
let _ = crate::m_native__memory::storeU64(ctx, memory, (state + ctx.globals.m_native__abi__FD_OFFSET.expect("global initialisation")), (crate::m_native__memory::loadU64(ctx, memory, (state + ctx.globals.m_native__abi__FD_OFFSET.expect("global initialisation")))).checked_add(crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, amount))).expect("BigInt arithmetic"));
}
let _ = crate::m_native__kernel::release(ctx, memory, vfsLock);
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return amount;
}


pub fn tetoSeekDescriptor<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut descriptor: f64, mut offset: i128, mut whence: f64) -> i128 {
if (((!crate::m_native__kernel::tetoKernelValid(ctx, memory)) || (!crate::m_native__kernel::validHart(ctx, memory, hart))) || (whence > 2.0f64)) {
return (22i128).checked_neg().expect("BigInt negation");
}
let proc: f64 = crate::m_native__kernel::processForHart(ctx, memory, hart);
if ((proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (descriptor >= crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_DESCRIPTOR_CAPACITY.expect("global initialisation"))))) {
return (9i128).checked_neg().expect("BigInt negation");
}
if (!crate::m_native__kernel::acquire(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")))) {
return (11i128).checked_neg().expect("BigInt negation");
}
let vfsLock: f64 = (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_VFS_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, vfsLock)) {
{
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return (11i128).checked_neg().expect("BigInt negation");
}
}
let state: f64 = crate::m_native__kernel::descriptorAt(ctx, memory, proc, descriptor);
let kind: f64 = crate::m_native__memory::loadU32(ctx, memory, (state + ctx.globals.m_native__abi__FD_KIND.expect("global initialisation")));
if ((kind != ctx.globals.m_native__abi__TETO_FD_FILE.expect("global initialisation")) && (kind != ctx.globals.m_native__abi__TETO_FD_DIRECTORY.expect("global initialisation"))) {
{
let _ = crate::m_native__kernel::release(ctx, memory, vfsLock);
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return (9i128).checked_neg().expect("BigInt negation");
}
}
let mut base: i128 = 0i128;
if (whence == 1.0f64) {
let _ = { base = crate::m_native__word::sx(ctx, crate::m_native__memory::loadU64(ctx, memory, (state + ctx.globals.m_native__abi__FD_OFFSET.expect("global initialisation"))), 64.0f64); base };
}
else {
if (whence == 2.0f64) {
let _ = { base = crate::m_native__word::sx(ctx, crate::m_native__vfs::tetoVfsFileSize(ctx, memory, crate::m_native__memory::loadU32(ctx, memory, (state + ctx.globals.m_native__abi__FD_INODE.expect("global initialisation")))), 64.0f64); base };
}
}
let next: i128 = (base).checked_add(offset).expect("BigInt arithmetic");
if ((next < 0i128) || (((offset > 0i128) && (next < base)))) {
{
let _ = crate::m_native__kernel::release(ctx, memory, vfsLock);
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return (22i128).checked_neg().expect("BigInt negation");
}
}
let _ = crate::m_native__memory::storeU64(ctx, memory, (state + ctx.globals.m_native__abi__FD_OFFSET.expect("global initialisation")), crate::m_native__word::ux(ctx, next));
let _ = crate::m_native__kernel::release(ctx, memory, vfsLock);
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return next;
}


pub fn tetoCloseDescriptor<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut descriptor: f64) -> f64 {
if ((!crate::m_native__kernel::tetoKernelValid(ctx, memory)) || (!crate::m_native__kernel::validHart(ctx, memory, hart))) {
return (-22.0f64);
}
let proc: f64 = crate::m_native__kernel::processForHart(ctx, memory, hart);
if ((proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (descriptor >= crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_DESCRIPTOR_CAPACITY.expect("global initialisation"))))) {
return (-9.0f64);
}
if (!crate::m_native__kernel::acquire(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")))) {
return (-11.0f64);
}
let state: f64 = crate::m_native__kernel::descriptorAt(ctx, memory, proc, descriptor);
if (crate::m_native__memory::loadU32(ctx, memory, (state + ctx.globals.m_native__abi__FD_KIND.expect("global initialisation"))) == ctx.globals.m_native__abi__TETO_FD_EMPTY.expect("global initialisation")) {
{
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return (-9.0f64);
}
}
let _ = crate::m_native__memory::fill(ctx, memory, state, ctx.globals.m_native__abi__TETO_DESCRIPTOR_STRIDE.expect("global initialisation"), 0.0f64);
let _ = crate::m_native__kernel::release(ctx, memory, (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation")));
return 0.0f64;
}


pub fn tetoDescriptorKind<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut descriptor: f64) -> f64 {
let proc: f64 = (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__kernel::processForHart(ctx, memory, hart) } else { ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation") });
if ((proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (descriptor >= crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_DESCRIPTOR_CAPACITY.expect("global initialisation"))))) {
return 0.0f64;
}
return crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__kernel::descriptorAt(ctx, memory, proc, descriptor) + ctx.globals.m_native__abi__FD_KIND.expect("global initialisation")));
}


pub fn tetoDescriptorInode<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut descriptor: f64) -> f64 {
let proc: f64 = (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__kernel::processForHart(ctx, memory, hart) } else { ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation") });
if ((proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (descriptor >= crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_DESCRIPTOR_CAPACITY.expect("global initialisation"))))) {
return 0.0f64;
}
return crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__kernel::descriptorAt(ctx, memory, proc, descriptor) + ctx.globals.m_native__abi__FD_INODE.expect("global initialisation")));
}


pub fn tetoDescriptorOffset<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut descriptor: f64) -> i128 {
let proc: f64 = (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__kernel::processForHart(ctx, memory, hart) } else { ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation") });
if ((proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (descriptor >= crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_DESCRIPTOR_CAPACITY.expect("global initialisation"))))) {
return 0i128;
}
return crate::m_native__memory::loadU64(ctx, memory, (crate::m_native__kernel::descriptorAt(ctx, memory, proc, descriptor) + ctx.globals.m_native__abi__FD_OFFSET.expect("global initialisation")));
}


pub fn tetoImageReserve<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut size: f64) -> f64 {
if (((!crate::m_native__kernel::tetoKernelValid(ctx, memory)) || (size == 0.0f64)) || (size > 2147483632.0f64)) {
return ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation");
}
let imageLock: f64 = (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_IMAGE_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, imageLock)) {
return ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation");
}
let memoryLock: f64 = (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_MEMORY_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, memoryLock)) {
{
let _ = crate::m_native__kernel::release(ctx, memory, imageLock);
return ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation");
}
}
let aligned: f64 = numbers::and(((size + 15.0f64)), (-16.0f64));
let end: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PHYSICAL_END.expect("global initialisation")));
let next: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PHYSICAL_NEXT.expect("global initialisation")));
if (((aligned < size) || (aligned > end)) || ((end - aligned) < next)) {
{
let _ = crate::m_native__kernel::release(ctx, memory, memoryLock);
let _ = crate::m_native__kernel::release(ctx, memory, imageLock);
return ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation");
}
}
let at: f64 = (end - aligned);
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_IMAGE_TOP.expect("global initialisation")), Numeric::Number(at));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_IMAGE_SIZE.expect("global initialisation")), Numeric::Number(aligned));
let _ = crate::m_native__kernel::release(ctx, memory, memoryLock);
return at;
}


pub fn tetoImageContains<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64, mut size: f64) -> bool {
if (crate::m_native__memory::atomicLoadI32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_IMAGE_LOCK.expect("global initialisation"))) != 1.0f64) {
return false;
}
let start: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_IMAGE_TOP.expect("global initialisation")));
let length: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_IMAGE_SIZE.expect("global initialisation")));
return (((at >= start) && ((at - start) <= length)) && (size <= (length - ((at - start)))));
}


pub fn tetoImageRelease<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut at: f64, mut size: f64) -> f64 {
if (((!crate::m_native__kernel::tetoKernelValid(ctx, memory)) || (size == 0.0f64)) || (size > 2147483632.0f64)) {
return ctx.globals.m_native__abi__TETO_THX_RANGE.expect("global initialisation");
}
let aligned: f64 = numbers::and(((size + 15.0f64)), (-16.0f64));
let imageLock: f64 = (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_IMAGE_LOCK.expect("global initialisation"));
if (((crate::m_native__memory::atomicLoadI32(ctx, memory, imageLock) != 1.0f64) || (at != crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_IMAGE_TOP.expect("global initialisation"))))) || (aligned != crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_IMAGE_SIZE.expect("global initialisation"))))) {
return ctx.globals.m_native__abi__TETO_THX_RANGE.expect("global initialisation");
}
let memoryLock: f64 = (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_MEMORY_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, memoryLock)) {
return ctx.globals.m_native__abi__TETO_THX_BUSY.expect("global initialisation");
}
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_IMAGE_TOP.expect("global initialisation")), Numeric::Number(crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PHYSICAL_END.expect("global initialisation")))));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_IMAGE_SIZE.expect("global initialisation")), Numeric::Number(0.0f64));
let _ = crate::m_native__kernel::release(ctx, memory, memoryLock);
let _ = crate::m_native__kernel::release(ctx, memory, imageLock);
return ctx.globals.m_native__abi__TETO_THX_OK.expect("global initialisation");
}


pub fn tetoHartVirtualTop<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> i128 {
return (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__memory::loadU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_VIRTUAL_TOP.expect("global initialisation"))) } else { 0i128 });
}


pub fn tetoHartImageFloor<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> i128 {
return (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__memory::loadU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_IMAGE_FLOOR.expect("global initialisation"))) } else { 0i128 });
}


pub fn tetoHartStackBottom<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> i128 {
return (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__memory::loadU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_STACK_BOTTOM.expect("global initialisation"))) } else { 0i128 });
}


pub fn tetoHartStackPointer<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> i128 {
return (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__memory::loadU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_STACK_POINTER.expect("global initialisation"))) } else { 0i128 });
}


pub fn tetoHartBreak<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> i128 {
let proc: f64 = (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__kernel::processForHart(ctx, memory, hart) } else { ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation") });
return (if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) { 0i128 } else { crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_BRK.expect("global initialisation"))) });
}


pub fn tetoProcessSegmentCount<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> f64 {
let proc: f64 = (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__kernel::processForHart(ctx, memory, hart) } else { ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation") });
return (if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) { 0.0f64 } else { crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_SEGMENT_COUNT.expect("global initialisation"))) });
}


pub fn tetoProcessMapCount<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> f64 {
let proc: f64 = (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__kernel::processForHart(ctx, memory, hart) } else { ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation") });
return (if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) { 0.0f64 } else { crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation"))) });
}


pub fn tetoProcessMapAddress<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut index: f64) -> i128 {
let proc: f64 = (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__kernel::processForHart(ctx, memory, hart) } else { ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation") });
if ((proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (index >= crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation"))))) {
return 0i128;
}
return crate::m_native__memory::loadU64(ctx, memory, (crate::m_native__kernel::mapAt(ctx, memory, proc, index) + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation")));
}


pub fn tetoProcessMapEnd<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut index: f64) -> i128 {
let proc: f64 = (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__kernel::processForHart(ctx, memory, hart) } else { ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation") });
if ((proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (index >= crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation"))))) {
return 0i128;
}
return crate::m_native__memory::loadU64(ctx, memory, (crate::m_native__kernel::mapAt(ctx, memory, proc, index) + ctx.globals.m_native__abi__M_END.expect("global initialisation")));
}


pub fn tetoProcessMapProtection<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut index: f64) -> f64 {
let proc: f64 = (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__kernel::processForHart(ctx, memory, hart) } else { ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation") });
if ((proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (index >= crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation"))))) {
return 0.0f64;
}
return crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__kernel::mapAt(ctx, memory, proc, index) + ctx.globals.m_native__abi__M_PROTECTION.expect("global initialisation")));
}


pub fn tetoHartGetX<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64) -> i128 {
if ((!crate::m_native__kernel::validHart(ctx, memory, hart)) || (register > 31.0f64)) {
return 0i128;
}
return (if (register == 0.0f64) { 0i128 } else { crate::m_native__memory::loadI64(ctx, memory, crate::m_native__kernel::regAt(ctx, hart, register)) });
}


pub fn tetoHartSetX<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut value: i128) -> f64 {
if ((!crate::m_native__kernel::validHart(ctx, memory, hart)) || (register > 31.0f64)) {
return ctx.globals.m_native__abi__TETO_FAULT_BAD_STATE.expect("global initialisation");
}
if (register != 0.0f64) {
let _ = crate::m_native__memory::storeI64(ctx, memory, crate::m_native__kernel::regAt(ctx, hart, register), crate::m_native__word::sx(ctx, value, 64.0f64));
}
return 0.0f64;
}


pub fn tetoHartGetF<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64) -> i128 {
if ((!crate::m_native__kernel::validHart(ctx, memory, hart)) || (register > 31.0f64)) {
return 0i128;
}
return crate::m_native__memory::loadU64(ctx, memory, crate::m_native__kernel::floatRegAt(ctx, hart, register));
}


pub fn tetoHartSetF<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut value: i128) -> f64 {
if ((!crate::m_native__kernel::validHart(ctx, memory, hart)) || (register > 31.0f64)) {
return ctx.globals.m_native__abi__TETO_FAULT_BAD_STATE.expect("global initialisation");
}
let _ = crate::m_native__memory::storeU64(ctx, memory, crate::m_native__kernel::floatRegAt(ctx, hart, register), value);
return 0.0f64;
}


pub fn tetoHartPc<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> i128 {
return (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__memory::loadU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_PC.expect("global initialisation"))) } else { 0i128 });
}


pub fn tetoHartSetPc<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut pc: i128) -> f64 {
if ((!crate::m_native__kernel::validHart(ctx, memory, hart)) || (pc >= crate::m_native__memory::loadU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_VIRTUAL_TOP.expect("global initialisation"))))) {
return ctx.globals.m_native__abi__TETO_FAULT_BAD_STATE.expect("global initialisation");
}
let _ = crate::m_native__memory::storeU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_PC.expect("global initialisation")), pc);
return 0.0f64;
}


pub fn tetoHartStatus<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> f64 {
return (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__memory::atomicLoadI32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_STATUS.expect("global initialisation"))) } else { ctx.globals.m_native__abi__TETO_HART_EMPTY.expect("global initialisation") });
}


pub fn tetoHartMetric<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut offset: f64) -> i128 {
if (((!crate::m_native__kernel::validHart(ctx, memory, hart)) || ((offset + 8.0f64) > ctx.globals.m_native__abi__TETO_HART_STRIDE.expect("global initialisation"))) || ((numbers::and(offset, 7.0f64)) != 0.0f64)) {
return 0i128;
}
return crate::m_native__memory::loadU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + offset));
}


pub fn tetoHostOperation<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> f64 {
return (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__memory::loadI32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_HOST_OPERATION.expect("global initialisation"))) } else { ctx.globals.m_native__abi__TETO_HOST_NONE.expect("global initialisation") });
}


pub fn tetoHostDescriptor<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> f64 {
return (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__memory::loadI32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_HOST_DESCRIPTOR.expect("global initialisation"))) } else { (-1.0f64) });
}


pub fn tetoHostAddress<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> i128 {
return (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__memory::loadU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_HOST_ADDRESS.expect("global initialisation"))) } else { 0i128 });
}


pub fn tetoHostLength<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> f64 {
return (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_HOST_LENGTH.expect("global initialisation"))) } else { 0.0f64 });
}


pub fn tetoHartExitCode<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> f64 {
return (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__memory::loadI32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_EXIT_CODE.expect("global initialisation"))) } else { (-1.0f64) });
}


pub fn setX<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut value: i128) -> () {
if (register != 0.0f64) {
let _ = crate::m_native__memory::storeI64(ctx, memory, crate::m_native__kernel::regAt(ctx, hart, register), crate::m_native__word::sx(ctx, value, 64.0f64));
}
}


pub fn x<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64) -> i128 {
return (if (register == 0.0f64) { 0i128 } else { crate::m_native__memory::loadI64(ctx, memory, crate::m_native__kernel::regAt(ctx, hart, register)) });
}


pub fn f<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64) -> i128 {
return crate::m_native__memory::loadU64(ctx, memory, crate::m_native__kernel::floatRegAt(ctx, hart, register));
}


pub fn setF<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut value: i128) -> () {
return crate::m_native__memory::storeU64(ctx, memory, crate::m_native__kernel::floatRegAt(ctx, hart, register), value);
}


pub fn setPc<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut value: i128) -> () {
return crate::m_native__memory::storeU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_PC.expect("global initialisation")), crate::m_native__word::ux(ctx, value));
}


pub fn guestRange<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut address: i128, mut size: f64) -> bool {
let proc: f64 = crate::m_native__kernel::processForHart(ctx, memory, hart);
if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return false;
}
let top: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_VIRTUAL_TOP.expect("global initialisation")));
return ((address < top) && (crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, size)) <= (top).checked_sub(address).expect("BigInt arithmetic")));
}


pub fn guestAllowed<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut address: i128, mut size: f64, mut permission: f64) -> bool {
if (!crate::m_native__kernel::guestRange(ctx, memory, hart, address, size)) {
return false;
}
let proc: f64 = crate::m_native__kernel::processForHart(ctx, memory, hart);
if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return false;
}
let end: i128 = (address).checked_add(crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, size))).expect("BigInt arithmetic");
let count: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_SEGMENT_COUNT.expect("global initialisation")));
let table: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_SEGMENT_TABLE.expect("global initialisation")));
let mut index: f64 = 0.0f64;
'__crust_v19: while (index < count) {
{
let segment: f64 = (table + (index * ctx.globals.m_native__abi__TETO_SEGMENT_STRIDE.expect("global initialisation")));
if ((address >= crate::m_native__memory::loadU64(ctx, memory, (segment + ctx.globals.m_native__abi__S_ADDRESS.expect("global initialisation")))) && (end <= crate::m_native__memory::loadU64(ctx, memory, (segment + ctx.globals.m_native__abi__S_END.expect("global initialisation"))))) {
{
return ((numbers::and(crate::m_native__memory::loadU32(ctx, memory, (segment + ctx.globals.m_native__abi__S_FLAGS.expect("global initialisation"))), permission)) == permission);
}
}
let _ = { index = (index + 1.0f64); index };
}
}
let executable: bool = ((numbers::and(permission, ctx.globals.m_native__abi__TETO_SEGMENT_EXECUTE.expect("global initialisation"))) != 0.0f64);
let stack: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_STACK_BOTTOM.expect("global initialisation")));
if (((stack != 0i128) && (address >= stack)) && (end <= crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_VIRTUAL_TOP.expect("global initialisation"))))) {
return (!executable);
}
let heap: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_BRK_BASE.expect("global initialisation")));
if (((heap != 0i128) && (address >= heap)) && (end <= crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_BRK.expect("global initialisation"))))) {
return (!executable);
}
let mapCount: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation")));
let mapTable: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_TABLE.expect("global initialisation")));
let _ = { index = 0.0f64; index };
'__crust_v20: while (index < mapCount) {
{
let mapping: f64 = (mapTable + (index * ctx.globals.m_native__abi__TETO_MAP_STRIDE.expect("global initialisation")));
if ((address >= crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation")))) && (end <= crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation"))))) {
{
return ((numbers::and(crate::m_native__memory::loadU32(ctx, memory, (mapping + ctx.globals.m_native__abi__M_PROTECTION.expect("global initialisation"))), permission)) == permission);
}
}
let _ = { index = (index + 1.0f64); index };
}
}
return false;
}


pub fn pageHash<P: Platform>(ctx: &Context<P>, mut page: i128, mut capacity: f64) -> f64 {
return numbers::and(crate::m_native__word::wordToU32(ctx, (((page ^ numbers::big_shr(page, 32i128)))).checked_mul(2654435761i128).expect("BigInt arithmetic")), ((capacity - 1.0f64)));
}


pub fn guestPage<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut address: i128, mut create: bool) -> f64 {
if (!crate::m_native__kernel::guestRange(ctx, memory, hart, address, 1.0f64)) {
return ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation");
}
let state: f64 = crate::m_native__kernel::hartAt(ctx, hart);
let proc: f64 = crate::m_native__kernel::processForHart(ctx, memory, hart);
if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation");
}
let table: f64 = crate::m_native__word::wordToU32(ctx, crate::m_native__word::word(ctx, crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PAGE_TABLE.expect("global initialisation")))));
let capacity: f64 = crate::m_native__word::wordToU32(ctx, crate::m_native__word::word(ctx, crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PAGE_CAPACITY.expect("global initialisation")))));
let page: i128 = numbers::big_shr(address, 16i128);
let key: i128 = (page).checked_add(1i128).expect("BigInt arithmetic");
let mut slot: f64 = crate::m_native__kernel::pageHash(ctx, page, capacity);
let mut probe: f64 = 0.0f64;
'__crust_v21: while (probe < capacity) {
{
let entry: f64 = (table + (slot * ctx.globals.m_native__abi__TETO_PAGE_ENTRY_SIZE.expect("global initialisation")));
let found: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, entry);
if (found == key) {
return crate::m_native__memory::loadU32(ctx, memory, (entry + 8.0f64));
}
if (found == 0i128) {
{
if (!create) {
return ctx.globals.m_native__kernel__UNMAPPED_POINTER.expect("global initialisation");
}
let lockAt: f64 = (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_MEMORY_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, lockAt)) {
return ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation");
}
let recheck: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, entry);
if (recheck == key) {
{
let existing: f64 = crate::m_native__memory::loadU32(ctx, memory, (entry + 8.0f64));
let _ = crate::m_native__kernel::release(ctx, memory, lockAt);
return existing;
}
}
if (recheck != 0i128) {
{
let _ = crate::m_native__kernel::release(ctx, memory, lockAt);
let _ = { slot = numbers::and(((slot + 1.0f64)), ((capacity - 1.0f64))); slot };
let _ = { probe = (probe + 1.0f64); probe };
continue '__crust_v21;
}
}
let frame: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PHYSICAL_NEXT.expect("global initialisation")));
let end: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_IMAGE_TOP.expect("global initialisation")));
if ((frame > end) || (ctx.globals.m_native__abi__TETO_GUEST_PAGE_SIZE.expect("global initialisation") > (end - frame))) {
{
let _ = crate::m_native__kernel::release(ctx, memory, lockAt);
return ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation");
}
}
let _ = crate::m_native__memory::fill(ctx, memory, frame, ctx.globals.m_native__abi__TETO_GUEST_PAGE_SIZE.expect("global initialisation"), 0.0f64);
let _ = crate::m_native__memory::storeU32(ctx, memory, (entry + 8.0f64), Numeric::Number(frame));
let _ = crate::m_native__memory::storeU32(ctx, memory, (entry + 12.0f64), Numeric::Number(0.0f64));
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_PHYSICAL_NEXT.expect("global initialisation")), Numeric::Number((frame + ctx.globals.m_native__abi__TETO_GUEST_PAGE_SIZE.expect("global initialisation"))));
let pageCount: f64 = (crate::m_native__word::wordToU32(ctx, crate::m_native__word::word(ctx, crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PAGE_COUNT.expect("global initialisation"))))) + 1.0f64);
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PAGE_COUNT.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, pageCount)));
let _ = crate::m_native__memory::storeU32(ctx, memory, (state + ctx.globals.m_native__abi__H_PAGE_COUNT.expect("global initialisation")), Numeric::Number(pageCount));
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, entry, key);
let _ = crate::m_native__kernel::release(ctx, memory, lockAt);
return frame;
}
}
let _ = { slot = numbers::and(((slot + 1.0f64)), ((capacity - 1.0f64))); slot };
let _ = { probe = (probe + 1.0f64); probe };
}
}
return ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation");
}


pub fn clearMappedRange<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut proc: f64, mut start: i128, mut end: i128) -> () {
let table: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PAGE_TABLE.expect("global initialisation")));
let capacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PAGE_CAPACITY.expect("global initialisation")));
let mut slot: f64 = 0.0f64;
'__crust_v22: while (slot < capacity) {
{
let entry: f64 = (table + (slot * ctx.globals.m_native__abi__TETO_PAGE_ENTRY_SIZE.expect("global initialisation")));
let key: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, entry);
if (key != 0i128) {
{
let page: i128 = numbers::big_shl(((key).checked_sub(1i128).expect("BigInt arithmetic")), 16i128);
let pageEnd: i128 = (page).checked_add(crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, ctx.globals.m_native__abi__TETO_GUEST_PAGE_SIZE.expect("global initialisation")))).expect("BigInt arithmetic");
if ((start < pageEnd) && (end > page)) {
{
let low: i128 = (if (start > page) { start } else { page });
let high: i128 = (if (end < pageEnd) { end } else { pageEnd });
let _ = crate::m_native__memory::fill(ctx, memory, (crate::m_native__memory::loadU32(ctx, memory, (entry + 8.0f64)) + crate::m_native__word::wordToU32(ctx, (low).checked_sub(page).expect("BigInt arithmetic"))), crate::m_native__word::wordToU32(ctx, (high).checked_sub(low).expect("BigInt arithmetic")), 0.0f64);
}
}
}
}
let _ = { slot = (slot + 1.0f64); slot };
}
}
}


pub fn mapAt<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut proc: f64, mut index: f64) -> f64 {
return (crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_TABLE.expect("global initialisation"))) + (index * ctx.globals.m_native__abi__TETO_MAP_STRIDE.expect("global initialisation")));
}


pub fn copyMap<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut to: f64, mut from: f64) -> () {
let _ = crate::m_native__memory::storeU64(ctx, memory, (to + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation")), crate::m_native__memory::loadU64(ctx, memory, (from + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation"))));
let _ = crate::m_native__memory::storeU64(ctx, memory, (to + ctx.globals.m_native__abi__M_END.expect("global initialisation")), crate::m_native__memory::loadU64(ctx, memory, (from + ctx.globals.m_native__abi__M_END.expect("global initialisation"))));
let _ = crate::m_native__memory::storeU32(ctx, memory, (to + ctx.globals.m_native__abi__M_PROTECTION.expect("global initialisation")), Numeric::Number(crate::m_native__memory::loadU32(ctx, memory, (from + ctx.globals.m_native__abi__M_PROTECTION.expect("global initialisation")))));
let _ = crate::m_native__memory::storeU32(ctx, memory, (to + ctx.globals.m_native__abi__M_FLAGS.expect("global initialisation")), Numeric::Number(crate::m_native__memory::loadU32(ctx, memory, (from + ctx.globals.m_native__abi__M_FLAGS.expect("global initialisation")))));
let _ = crate::m_native__memory::storeI32(ctx, memory, (to + ctx.globals.m_native__abi__M_FILE_DESCRIPTOR.expect("global initialisation")), crate::m_native__memory::loadI32(ctx, memory, (from + ctx.globals.m_native__abi__M_FILE_DESCRIPTOR.expect("global initialisation"))));
let _ = crate::m_native__memory::storeU64(ctx, memory, (to + ctx.globals.m_native__abi__M_FILE_OFFSET.expect("global initialisation")), crate::m_native__memory::loadU64(ctx, memory, (from + ctx.globals.m_native__abi__M_FILE_OFFSET.expect("global initialisation"))));
let _ = crate::m_native__memory::storeU32(ctx, memory, (to + ctx.globals.m_native__abi__M_BACKING.expect("global initialisation")), Numeric::Number(crate::m_native__memory::loadU32(ctx, memory, (from + ctx.globals.m_native__abi__M_BACKING.expect("global initialisation")))));
let _ = crate::m_native__memory::storeU32(ctx, memory, ((to + ctx.globals.m_native__abi__M_BACKING.expect("global initialisation")) + 4.0f64), Numeric::Number(0.0f64));
}


pub fn mappingConflict<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut proc: f64, mut address: i128, mut end: i128) -> bool {
let top: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_VIRTUAL_TOP.expect("global initialisation")));
let stack: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_STACK_BOTTOM.expect("global initialisation")));
let heap: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_BRK.expect("global initialisation")));
if (((((address < (((((heap).checked_add(ctx.globals.m_native__kernel__MAP_GUARD.expect("global initialisation")).expect("BigInt arithmetic")).checked_add(ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).expect("BigInt arithmetic")).checked_sub(1i128).expect("BigInt arithmetic") & (ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_neg().expect("BigInt negation")))) || (end <= address)) || (end > top)) || (stack <= ctx.globals.m_native__kernel__MAP_GUARD.expect("global initialisation"))) || (end >= (stack).checked_sub(ctx.globals.m_native__kernel__MAP_GUARD.expect("global initialisation")).expect("BigInt arithmetic"))) {
return true;
}
let count: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation")));
let mut index: f64 = 0.0f64;
'__crust_v23: while (index < count) {
{
let mapping: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, index);
if ((address < crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation")))) && (end > crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation"))))) {
return true;
}
let _ = { index = (index + 1.0f64); index };
}
}
return false;
}


pub fn anonymousMap<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut proc: f64, mut address: i128, mut length: i128, mut protection: i128, mut flagsWide: i128) -> i128 {
let top: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_VIRTUAL_TOP.expect("global initialisation")));
if ((((length == 0i128) || (length > top)) || (protection > 7i128)) || (flagsWide > 2147483647i128)) {
return (22i128).checked_neg().expect("BigInt negation");
}
let flags: f64 = crate::m_native__word::wordToU32(ctx, flagsWide);
let kind: f64 = numbers::and(flags, 3.0f64);
if ((((numbers::and(flags, 32.0f64)) == 0.0f64) || (((kind != 1.0f64) && (kind != 2.0f64)))) || ((numbers::and(flags, numbers::not(1065011.0f64))) != 0.0f64)) {
return (22i128).checked_neg().expect("BigInt negation");
}
let size: i128 = (((length).checked_add(ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).expect("BigInt arithmetic")).checked_sub(1i128).expect("BigInt arithmetic") & (ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_neg().expect("BigInt negation"));
if ((size == 0i128) || (size > top)) {
return (12i128).checked_neg().expect("BigInt negation");
}
let fixed: bool = ((numbers::and(flags, 1048592.0f64)) != 0.0f64);
let replace: bool = (((numbers::and(flags, 16.0f64)) != 0.0f64) && ((numbers::and(flags, 1048576.0f64)) == 0.0f64));
let mut candidate: i128 = address;
if fixed {
{
if (((((candidate & ((ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_sub(1i128).expect("BigInt arithmetic")))) != 0i128) || (candidate > top)) || (size > (top).checked_sub(candidate).expect("BigInt arithmetic"))) {
return (12i128).checked_neg().expect("BigInt negation");
}
let end: i128 = (candidate).checked_add(size).expect("BigInt arithmetic");
if replace {
{
let heap: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_BRK.expect("global initialisation")));
let stack: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_STACK_BOTTOM.expect("global initialisation")));
if (((candidate < (((((heap).checked_add(ctx.globals.m_native__kernel__MAP_GUARD.expect("global initialisation")).expect("BigInt arithmetic")).checked_add(ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).expect("BigInt arithmetic")).checked_sub(1i128).expect("BigInt arithmetic") & (ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_neg().expect("BigInt negation")))) || (stack <= ctx.globals.m_native__kernel__MAP_GUARD.expect("global initialisation"))) || (end >= (stack).checked_sub(ctx.globals.m_native__kernel__MAP_GUARD.expect("global initialisation")).expect("BigInt arithmetic"))) {
return (12i128).checked_neg().expect("BigInt negation");
}
let count: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation")));
let capacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_CAPACITY.expect("global initialisation")));
let mut finalCount: f64 = (count + 1.0f64);
let mut index: f64 = 0.0f64;
'__crust_v24: while (index < count) {
{
let mapping: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, index);
let start: i128 = crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation")));
let stop: i128 = crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation")));
if ((candidate < stop) && (end > start)) {
{
if ((candidate <= start) && (end >= stop)) {
let _ = { finalCount = (finalCount - 1.0f64); finalCount };
}
else {
if ((candidate > start) && (end < stop)) {
let _ = { finalCount = (finalCount + 1.0f64); finalCount };
}
}
}
}
let _ = { index = (index + 1.0f64); index };
}
}
if (finalCount > capacity) {
return (12i128).checked_neg().expect("BigInt negation");
}
let removed: i128 = crate::m_native__kernel::anonymousUnmap(ctx, memory, proc, candidate, size);
if ((removed != 0i128) && (removed != ctx.globals.m_native__kernel__MAP_NOT_OWNED.expect("global initialisation"))) {
return removed;
}
}
}
else {
if crate::m_native__kernel::mappingConflict(ctx, memory, proc, candidate, end) {
{
return (if ((numbers::and(flags, 1048576.0f64)) != 0.0f64) { (17i128).checked_neg().expect("BigInt negation") } else { (12i128).checked_neg().expect("BigInt negation") });
}
}
}
}
}
else {
{
if (candidate != 0i128) {
{
if (candidate > ((top).checked_sub(ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).expect("BigInt arithmetic")).checked_add(1i128).expect("BigInt arithmetic")) {
let _ = { candidate = 0i128; candidate };
}
else {
let _ = { candidate = (((candidate).checked_add(ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).expect("BigInt arithmetic")).checked_sub(1i128).expect("BigInt arithmetic") & (ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_neg().expect("BigInt negation")); candidate };
}
}
}
if ((((candidate == 0i128) || (candidate > top)) || (size > (top).checked_sub(candidate).expect("BigInt arithmetic"))) || crate::m_native__kernel::mappingConflict(ctx, memory, proc, candidate, (candidate).checked_add(size).expect("BigInt arithmetic"))) {
{
let minimum: i128 = ((((crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_BRK.expect("global initialisation")))).checked_add(ctx.globals.m_native__kernel__MAP_SPACING.expect("global initialisation")).expect("BigInt arithmetic")).checked_add(ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).expect("BigInt arithmetic")).checked_sub(1i128).expect("BigInt arithmetic") & (ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_neg().expect("BigInt negation"));
let _ = { candidate = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_NEXT.expect("global initialisation"))); candidate };
if (((candidate < minimum) || (candidate > top)) || (size > (top).checked_sub(candidate).expect("BigInt arithmetic"))) {
let _ = { candidate = minimum; candidate };
}
let mut attempts: f64 = 0.0f64;
let capacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_CAPACITY.expect("global initialisation")));
'__crust_v25: while ((attempts <= capacity) && ((((candidate > top) || (size > (top).checked_sub(candidate).expect("BigInt arithmetic"))) || crate::m_native__kernel::mappingConflict(ctx, memory, proc, candidate, (candidate).checked_add(size).expect("BigInt arithmetic"))))) {
{
if (candidate > (top).checked_sub(ctx.globals.m_native__kernel__MAP_SPACING.expect("global initialisation")).expect("BigInt arithmetic")) {
return (12i128).checked_neg().expect("BigInt negation");
}
let _ = { candidate = ((((candidate).checked_add(ctx.globals.m_native__kernel__MAP_SPACING.expect("global initialisation")).expect("BigInt arithmetic")).checked_add(ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).expect("BigInt arithmetic")).checked_sub(1i128).expect("BigInt arithmetic") & (ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_neg().expect("BigInt negation")); candidate };
let _ = { attempts = (attempts + 1.0f64); attempts };
}
}
if (((candidate > top) || (size > (top).checked_sub(candidate).expect("BigInt arithmetic"))) || crate::m_native__kernel::mappingConflict(ctx, memory, proc, candidate, (candidate).checked_add(size).expect("BigInt arithmetic"))) {
return (12i128).checked_neg().expect("BigInt negation");
}
}
}
}
}
let count: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation")));
if (count >= crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_CAPACITY.expect("global initialisation")))) {
return (12i128).checked_neg().expect("BigInt negation");
}
let _ = crate::m_native__kernel::clearMappedRange(ctx, memory, proc, candidate, (candidate).checked_add(size).expect("BigInt arithmetic"));
let mapping: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, count);
let _ = crate::m_native__memory::fill(ctx, memory, mapping, ctx.globals.m_native__abi__TETO_MAP_STRIDE.expect("global initialisation"), 0.0f64);
let _ = crate::m_native__memory::storeU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation")), candidate);
let _ = crate::m_native__memory::storeU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation")), (candidate).checked_add(size).expect("BigInt arithmetic"));
let _ = crate::m_native__memory::storeU32(ctx, memory, (mapping + ctx.globals.m_native__abi__M_PROTECTION.expect("global initialisation")), Numeric::Number(crate::m_native__word::wordToU32(ctx, protection)));
let _ = crate::m_native__memory::storeU32(ctx, memory, (mapping + ctx.globals.m_native__abi__M_FLAGS.expect("global initialisation")), Numeric::Number(flags));
let _ = crate::m_native__memory::storeI32(ctx, memory, (mapping + ctx.globals.m_native__abi__M_FILE_DESCRIPTOR.expect("global initialisation")), (-1.0f64));
let _ = crate::m_native__memory::storeU32(ctx, memory, (mapping + ctx.globals.m_native__abi__M_BACKING.expect("global initialisation")), Numeric::Number(ctx.globals.m_native__abi__TETO_MAP_BACKING_ANONYMOUS.expect("global initialisation")));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, (count + 1.0f64))));
let next: i128 = ((((candidate).checked_add(size).expect("BigInt arithmetic")).checked_add(ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).expect("BigInt arithmetic")).checked_sub(1i128).expect("BigInt arithmetic") & (ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_neg().expect("BigInt negation"));
if (next > crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_NEXT.expect("global initialisation")))) {
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_NEXT.expect("global initialisation")), next);
}
return crate::m_native__word::sx(ctx, candidate, 64.0f64);
}


pub fn anonymousUnmap<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut proc: f64, mut address: i128, mut length: i128) -> i128 {
let top: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_VIRTUAL_TOP.expect("global initialisation")));
if ((((length == 0i128) || (((address & ((ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_sub(1i128).expect("BigInt arithmetic")))) != 0i128)) || (address >= top)) || (length > (top).checked_sub(address).expect("BigInt arithmetic"))) {
return (22i128).checked_neg().expect("BigInt negation");
}
let size: i128 = (((length).checked_add(ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).expect("BigInt arithmetic")).checked_sub(1i128).expect("BigInt arithmetic") & (ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_neg().expect("BigInt negation"));
if ((size == 0i128) || (size > (top).checked_sub(address).expect("BigInt arithmetic"))) {
return (22i128).checked_neg().expect("BigInt negation");
}
let end: i128 = (address).checked_add(size).expect("BigInt arithmetic");
let mut count: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation")));
let capacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_CAPACITY.expect("global initialisation")));
let mut index: f64 = 0.0f64;
let mut splits: f64 = 0.0f64;
let mut hit: bool = false;
'__crust_v26: while (index < count) {
{
let mapping: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, index);
let start: i128 = crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation")));
let stop: i128 = crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation")));
if ((address < stop) && (end > start)) {
{
let _ = { hit = true; hit };
if ((address > start) && (end < stop)) {
let _ = { splits = (splits + 1.0f64); splits };
}
}
}
let _ = { index = (index + 1.0f64); index };
}
}
if (!hit) {
return ctx.globals.m_native__kernel__MAP_NOT_OWNED.expect("global initialisation");
}
if (splits > (capacity - count)) {
return (12i128).checked_neg().expect("BigInt negation");
}
let _ = { index = 0.0f64; index };
'__crust_v27: while (index < count) {
{
let mapping: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, index);
let start: i128 = crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation")));
let stop: i128 = crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation")));
if ((address >= stop) || (end <= start)) {
{
let _ = { index = (index + 1.0f64); index };
continue '__crust_v27;
}
}
let low: i128 = (if (address > start) { address } else { start });
let high: i128 = (if (end < stop) { end } else { stop });
let _ = crate::m_native__kernel::clearMappedRange(ctx, memory, proc, low, high);
if ((address <= start) && (end >= stop)) {
{
let mut ts_move: f64 = index;
'__crust_v28: while ((ts_move + 1.0f64) < count) {
{
let _ = crate::m_native__kernel::copyMap(ctx, memory, crate::m_native__kernel::mapAt(ctx, memory, proc, ts_move), crate::m_native__kernel::mapAt(ctx, memory, proc, (ts_move + 1.0f64)));
let _ = { ts_move = (ts_move + 1.0f64); ts_move };
}
}
let _ = { count = (count - 1.0f64); count };
let _ = crate::m_native__memory::fill(ctx, memory, crate::m_native__kernel::mapAt(ctx, memory, proc, count), ctx.globals.m_native__abi__TETO_MAP_STRIDE.expect("global initialisation"), 0.0f64);
continue '__crust_v27;
}
}
if (address <= start) {
{
let _ = crate::m_native__memory::storeU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation")), end);
let _ = crate::m_native__memory::storeU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_FILE_OFFSET.expect("global initialisation")), ((crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_FILE_OFFSET.expect("global initialisation")))).checked_add(end).expect("BigInt arithmetic")).checked_sub(start).expect("BigInt arithmetic"));
let _ = { index = (index + 1.0f64); index };
continue '__crust_v27;
}
}
if (end >= stop) {
{
let _ = crate::m_native__memory::storeU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation")), address);
let _ = { index = (index + 1.0f64); index };
continue '__crust_v27;
}
}
let right: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, count);
let _ = crate::m_native__kernel::copyMap(ctx, memory, right, mapping);
let _ = crate::m_native__memory::storeU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation")), address);
let _ = crate::m_native__memory::storeU64(ctx, memory, (right + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation")), end);
let _ = crate::m_native__memory::storeU64(ctx, memory, (right + ctx.globals.m_native__abi__M_FILE_OFFSET.expect("global initialisation")), ((crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_FILE_OFFSET.expect("global initialisation")))).checked_add(end).expect("BigInt arithmetic")).checked_sub(start).expect("BigInt arithmetic"));
let _ = { count = (count + 1.0f64); count };
let _ = { index = (index + 1.0f64); index };
}
}
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, count)));
return 0i128;
}


pub fn mappingsCover<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut proc: f64, mut address: i128, mut end: i128) -> bool {
let count: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation")));
let mut cursor: i128 = address;
'__crust_v29: while (cursor < end) {
{
let mut found: bool = false;
let mut index: f64 = 0.0f64;
'__crust_v30: while (index < count) {
{
let mapping: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, index);
let start: i128 = crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation")));
let stop: i128 = crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation")));
if ((cursor >= start) && (cursor < stop)) {
{
let _ = { cursor = (if (stop < end) { stop } else { end }); cursor };
let _ = { found = true; found };
break '__crust_v30;
}
}
let _ = { index = (index + 1.0f64); index };
}
}
if (!found) {
return false;
}
}
}
return true;
}


pub fn protectMappings<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut proc: f64, mut address: i128, mut length: i128, mut protection: i128) -> i128 {
let top: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_VIRTUAL_TOP.expect("global initialisation")));
if (((((length == 0i128) || (protection > 7i128)) || (((address & ((ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_sub(1i128).expect("BigInt arithmetic")))) != 0i128)) || (address >= top)) || (length > (top).checked_sub(address).expect("BigInt arithmetic"))) {
return (22i128).checked_neg().expect("BigInt negation");
}
let size: i128 = (((length).checked_add(ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).expect("BigInt arithmetic")).checked_sub(1i128).expect("BigInt arithmetic") & (ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_neg().expect("BigInt negation"));
if ((size == 0i128) || (size > (top).checked_sub(address).expect("BigInt arithmetic"))) {
return (22i128).checked_neg().expect("BigInt negation");
}
let end: i128 = (address).checked_add(size).expect("BigInt arithmetic");
let original: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation")));
let mut hit: bool = false;
let mut needed: f64 = 0.0f64;
let mut index: f64 = 0.0f64;
'__crust_v31: while (index < original) {
{
let mapping: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, index);
let start: i128 = crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation")));
let stop: i128 = crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation")));
if ((address < stop) && (end > start)) {
{
let _ = { hit = true; hit };
let low: i128 = (if (address > start) { address } else { start });
let high: i128 = (if (end < stop) { end } else { stop });
if (low > start) {
let _ = { needed = (needed + 1.0f64); needed };
}
if (high < stop) {
let _ = { needed = (needed + 1.0f64); needed };
}
}
}
let _ = { index = (index + 1.0f64); index };
}
}
if (!hit) {
return ctx.globals.m_native__kernel__MAP_NOT_OWNED.expect("global initialisation");
}
if (!crate::m_native__kernel::mappingsCover(ctx, memory, proc, address, end)) {
return (12i128).checked_neg().expect("BigInt negation");
}
let capacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_CAPACITY.expect("global initialisation")));
if (needed > (capacity - original)) {
return (12i128).checked_neg().expect("BigInt negation");
}
let mut count: f64 = original;
let _ = { index = 0.0f64; index };
'__crust_v32: while (index < original) {
{
let mapping: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, index);
let start: i128 = crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation")));
let stop: i128 = crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation")));
if ((address >= stop) || (end <= start)) {
{
let _ = { index = (index + 1.0f64); index };
continue '__crust_v32;
}
}
let low: i128 = (if (address > start) { address } else { start });
let high: i128 = (if (end < stop) { end } else { stop });
let oldOffset: i128 = crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_FILE_OFFSET.expect("global initialisation")));
if ((low == start) && (high == stop)) {
{
let _ = crate::m_native__memory::storeU32(ctx, memory, (mapping + ctx.globals.m_native__abi__M_PROTECTION.expect("global initialisation")), Numeric::Number(crate::m_native__word::wordToU32(ctx, protection)));
}
}
else {
if (low == start) {
{
let right: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, count);
let _ = crate::m_native__kernel::copyMap(ctx, memory, right, mapping);
let _ = crate::m_native__memory::storeU64(ctx, memory, (right + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation")), high);
let _ = crate::m_native__memory::storeU64(ctx, memory, (right + ctx.globals.m_native__abi__M_FILE_OFFSET.expect("global initialisation")), ((oldOffset).checked_add(high).expect("BigInt arithmetic")).checked_sub(start).expect("BigInt arithmetic"));
let _ = crate::m_native__memory::storeU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation")), high);
let _ = crate::m_native__memory::storeU32(ctx, memory, (mapping + ctx.globals.m_native__abi__M_PROTECTION.expect("global initialisation")), Numeric::Number(crate::m_native__word::wordToU32(ctx, protection)));
let _ = { count = (count + 1.0f64); count };
}
}
else {
if (high == stop) {
{
let changed: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, count);
let _ = crate::m_native__kernel::copyMap(ctx, memory, changed, mapping);
let _ = crate::m_native__memory::storeU64(ctx, memory, (changed + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation")), low);
let _ = crate::m_native__memory::storeU64(ctx, memory, (changed + ctx.globals.m_native__abi__M_FILE_OFFSET.expect("global initialisation")), ((oldOffset).checked_add(low).expect("BigInt arithmetic")).checked_sub(start).expect("BigInt arithmetic"));
let _ = crate::m_native__memory::storeU32(ctx, memory, (changed + ctx.globals.m_native__abi__M_PROTECTION.expect("global initialisation")), Numeric::Number(crate::m_native__word::wordToU32(ctx, protection)));
let _ = crate::m_native__memory::storeU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation")), low);
let _ = { count = (count + 1.0f64); count };
}
}
else {
{
let changed: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, count);
let right: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, (count + 1.0f64));
let _ = crate::m_native__kernel::copyMap(ctx, memory, changed, mapping);
let _ = crate::m_native__kernel::copyMap(ctx, memory, right, mapping);
let _ = crate::m_native__memory::storeU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation")), low);
let _ = crate::m_native__memory::storeU64(ctx, memory, (changed + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation")), low);
let _ = crate::m_native__memory::storeU64(ctx, memory, (changed + ctx.globals.m_native__abi__M_END.expect("global initialisation")), high);
let _ = crate::m_native__memory::storeU64(ctx, memory, (changed + ctx.globals.m_native__abi__M_FILE_OFFSET.expect("global initialisation")), ((oldOffset).checked_add(low).expect("BigInt arithmetic")).checked_sub(start).expect("BigInt arithmetic"));
let _ = crate::m_native__memory::storeU32(ctx, memory, (changed + ctx.globals.m_native__abi__M_PROTECTION.expect("global initialisation")), Numeric::Number(crate::m_native__word::wordToU32(ctx, protection)));
let _ = crate::m_native__memory::storeU64(ctx, memory, (right + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation")), high);
let _ = crate::m_native__memory::storeU64(ctx, memory, (right + ctx.globals.m_native__abi__M_FILE_OFFSET.expect("global initialisation")), ((oldOffset).checked_add(high).expect("BigInt arithmetic")).checked_sub(start).expect("BigInt arithmetic"));
let _ = { count = (count + 2.0f64); count };
}
}
}
}
let _ = { index = (index + 1.0f64); index };
}
}
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, count)));
return 0i128;
}


pub fn mappingConflictExcept<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut proc: f64, mut address: i128, mut end: i128, mut except: f64) -> bool {
let top: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_VIRTUAL_TOP.expect("global initialisation")));
let stack: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_STACK_BOTTOM.expect("global initialisation")));
let heap: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_BRK.expect("global initialisation")));
if (((((address < (((((heap).checked_add(ctx.globals.m_native__kernel__MAP_GUARD.expect("global initialisation")).expect("BigInt arithmetic")).checked_add(ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).expect("BigInt arithmetic")).checked_sub(1i128).expect("BigInt arithmetic") & (ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_neg().expect("BigInt negation")))) || (end <= address)) || (end > top)) || (stack <= ctx.globals.m_native__kernel__MAP_GUARD.expect("global initialisation"))) || (end >= (stack).checked_sub(ctx.globals.m_native__kernel__MAP_GUARD.expect("global initialisation")).expect("BigInt arithmetic"))) {
return true;
}
let count: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation")));
let mut index: f64 = 0.0f64;
'__crust_v33: while (index < count) {
{
if (index != except) {
{
let mapping: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, index);
if ((address < crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation")))) && (end > crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation"))))) {
return true;
}
}
}
let _ = { index = (index + 1.0f64); index };
}
}
return false;
}


pub fn copyAllocatedRange<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut proc: f64, mut source: i128, mut destination: i128, mut length: i128) -> bool {
let table: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PAGE_TABLE.expect("global initialisation")));
let capacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PAGE_CAPACITY.expect("global initialisation")));
let sourceEnd: i128 = (source).checked_add(length).expect("BigInt arithmetic");
let mut slot: f64 = 0.0f64;
'__crust_v34: while (slot < capacity) {
{
let entry: f64 = (table + (slot * ctx.globals.m_native__abi__TETO_PAGE_ENTRY_SIZE.expect("global initialisation")));
let key: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, entry);
if (key != 0i128) {
{
let page: i128 = numbers::big_shl(((key).checked_sub(1i128).expect("BigInt arithmetic")), 16i128);
let pageEnd: i128 = (page).checked_add(crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, ctx.globals.m_native__abi__TETO_GUEST_PAGE_SIZE.expect("global initialisation")))).expect("BigInt arithmetic");
if ((source < pageEnd) && (sourceEnd > page)) {
{
let low: i128 = (if (source > page) { source } else { page });
let high: i128 = (if (sourceEnd < pageEnd) { sourceEnd } else { pageEnd });
let sourceFrame: f64 = crate::m_native__memory::loadU32(ctx, memory, (entry + 8.0f64));
let mut copied: f64 = 0.0f64;
let amount: f64 = crate::m_native__word::wordToU32(ctx, (high).checked_sub(low).expect("BigInt arithmetic"));
'__crust_v35: while (copied < amount) {
{
let target: i128 = (((destination).checked_add(low).expect("BigInt arithmetic")).checked_sub(source).expect("BigInt arithmetic")).checked_add(crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, copied))).expect("BigInt arithmetic");
let targetFrame: f64 = crate::m_native__kernel::guestPage(ctx, memory, hart, target, true);
if ((targetFrame == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (targetFrame == ctx.globals.m_native__kernel__UNMAPPED_POINTER.expect("global initialisation"))) {
return false;
}
let within: f64 = crate::m_native__word::wordToU32(ctx, (target & 65535i128));
let room: f64 = (ctx.globals.m_native__abi__TETO_GUEST_PAGE_SIZE.expect("global initialisation") - within);
let part: f64 = (if (room < (amount - copied)) { room } else { (amount - copied) });
let _ = crate::m_native__memory::copyMemory(ctx, memory, (targetFrame + within), ((sourceFrame + crate::m_native__word::wordToU32(ctx, (low).checked_sub(page).expect("BigInt arithmetic"))) + copied), part);
let _ = { copied = (copied + part); copied };
}
}
}
}
}
}
let _ = { slot = (slot + 1.0f64); slot };
}
}
return true;
}


pub fn remapAnonymous<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut proc: f64, mut oldAddress: i128, mut oldLength: i128, mut newLength: i128, mut flagsWide: i128, mut fixedAddress: i128) -> i128 {
let top: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_VIRTUAL_TOP.expect("global initialisation")));
if (((((((((oldAddress & ((ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_sub(1i128).expect("BigInt arithmetic")))) != 0i128) || (oldLength == 0i128)) || (newLength == 0i128)) || (oldLength > top)) || (newLength > top)) || (flagsWide > 1i128)) || (fixedAddress != 0i128)) {
return (22i128).checked_neg().expect("BigInt negation");
}
let oldSize: i128 = (((oldLength).checked_add(ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).expect("BigInt arithmetic")).checked_sub(1i128).expect("BigInt arithmetic") & (ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_neg().expect("BigInt negation"));
let newSize: i128 = (((newLength).checked_add(ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).expect("BigInt arithmetic")).checked_sub(1i128).expect("BigInt arithmetic") & (ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_neg().expect("BigInt negation"));
if ((((oldSize == 0i128) || (newSize == 0i128)) || (oldAddress > top)) || (oldSize > (top).checked_sub(oldAddress).expect("BigInt arithmetic"))) {
return (22i128).checked_neg().expect("BigInt negation");
}
let count: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation")));
let mut index: f64 = 0.0f64;
let mut found: f64 = count;
'__crust_v36: while (index < count) {
{
let mapping: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, index);
if (((crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation"))) == oldAddress) && (crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation"))) == (oldAddress).checked_add(oldSize).expect("BigInt arithmetic"))) && (crate::m_native__memory::loadU32(ctx, memory, (mapping + ctx.globals.m_native__abi__M_BACKING.expect("global initialisation"))) == ctx.globals.m_native__abi__TETO_MAP_BACKING_ANONYMOUS.expect("global initialisation"))) {
let _ = { found = index; found };
}
let _ = { index = (index + 1.0f64); index };
}
}
if (found == count) {
return ctx.globals.m_native__kernel__MAP_NOT_OWNED.expect("global initialisation");
}
let mapping: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, found);
if (newSize == oldSize) {
return crate::m_native__word::sx(ctx, oldAddress, 64.0f64);
}
if (newSize < oldSize) {
{
let _ = crate::m_native__kernel::clearMappedRange(ctx, memory, proc, (oldAddress).checked_add(newSize).expect("BigInt arithmetic"), (oldAddress).checked_add(oldSize).expect("BigInt arithmetic"));
let _ = crate::m_native__memory::storeU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation")), (oldAddress).checked_add(newSize).expect("BigInt arithmetic"));
return crate::m_native__word::sx(ctx, oldAddress, 64.0f64);
}
}
let extended: i128 = (oldAddress).checked_add(newSize).expect("BigInt arithmetic");
if (!crate::m_native__kernel::mappingConflictExcept(ctx, memory, proc, oldAddress, extended, found)) {
{
let _ = crate::m_native__kernel::clearMappedRange(ctx, memory, proc, (oldAddress).checked_add(oldSize).expect("BigInt arithmetic"), extended);
let _ = crate::m_native__memory::storeU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation")), extended);
if (extended > crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_NEXT.expect("global initialisation")))) {
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_NEXT.expect("global initialisation")), extended);
}
return crate::m_native__word::sx(ctx, oldAddress, 64.0f64);
}
}
if ((numbers::and(crate::m_native__word::wordToU32(ctx, flagsWide), 1.0f64)) == 0.0f64) {
return (12i128).checked_neg().expect("BigInt negation");
}
let mapFlags: f64 = numbers::and(crate::m_native__memory::loadU32(ctx, memory, (mapping + ctx.globals.m_native__abi__M_FLAGS.expect("global initialisation"))), numbers::not(1048592.0f64));
let moved: i128 = crate::m_native__kernel::anonymousMap(ctx, memory, proc, 0i128, newSize, crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, crate::m_native__memory::loadU32(ctx, memory, (mapping + ctx.globals.m_native__abi__M_PROTECTION.expect("global initialisation"))))), crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, mapFlags)));
if (moved < 0i128) {
return moved;
}
let destination: i128 = crate::m_native__word::ux(ctx, moved);
if (!crate::m_native__kernel::copyAllocatedRange(ctx, memory, hart, proc, oldAddress, destination, oldSize)) {
{
let _ = crate::m_native__kernel::anonymousUnmap(ctx, memory, proc, destination, newSize);
return (12i128).checked_neg().expect("BigInt negation");
}
}
let removed: i128 = crate::m_native__kernel::anonymousUnmap(ctx, memory, proc, oldAddress, oldSize);
if (removed != 0i128) {
return (12i128).checked_neg().expect("BigInt negation");
}
return crate::m_native__word::sx(ctx, destination, 64.0f64);
}


pub fn syncAnonymous<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut proc: f64, mut address: i128, mut length: i128, mut flagsWide: i128) -> i128 {
let top: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_VIRTUAL_TOP.expect("global initialisation")));
if ((((((length == 0i128) || (((address & ((ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_sub(1i128).expect("BigInt arithmetic")))) != 0i128)) || (address >= top)) || (length > (top).checked_sub(address).expect("BigInt arithmetic"))) || (flagsWide > 7i128)) || ((numbers::and(crate::m_native__word::wordToU32(ctx, flagsWide), 5.0f64)) == 0.0f64)) {
return (22i128).checked_neg().expect("BigInt negation");
}
let size: i128 = (((length).checked_add(ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).expect("BigInt arithmetic")).checked_sub(1i128).expect("BigInt arithmetic") & (ctx.globals.m_native__kernel__MAP_PAGE.expect("global initialisation")).checked_neg().expect("BigInt negation"));
if ((size == 0i128) || (size > (top).checked_sub(address).expect("BigInt arithmetic"))) {
return (22i128).checked_neg().expect("BigInt negation");
}
let end: i128 = (address).checked_add(size).expect("BigInt arithmetic");
if (!crate::m_native__kernel::mappingsCover(ctx, memory, proc, address, end)) {
return ctx.globals.m_native__kernel__MAP_NOT_OWNED.expect("global initialisation");
}
let count: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation")));
let mut index: f64 = 0.0f64;
'__crust_v37: while (index < count) {
{
let mapping: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, index);
if (((address < crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_END.expect("global initialisation")))) && (end > crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation"))))) && (crate::m_native__memory::loadU32(ctx, memory, (mapping + ctx.globals.m_native__abi__M_BACKING.expect("global initialisation"))) != ctx.globals.m_native__abi__TETO_MAP_BACKING_ANONYMOUS.expect("global initialisation"))) {
return ctx.globals.m_native__kernel__MAP_NOT_OWNED.expect("global initialisation");
}
let _ = { index = (index + 1.0f64); index };
}
}
return 0i128;
}


pub fn tetoGuestPage<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut address: i128, mut create: bool) -> f64 {
return (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__kernel::guestPage(ctx, memory, hart, address, create) } else { ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation") });
}


pub fn guestPointer<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut address: i128, mut size: f64, mut create: bool) -> f64 {
if (!crate::m_native__kernel::guestRange(ctx, memory, hart, address, size)) {
return ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation");
}
let offset: f64 = crate::m_native__word::wordToU32(ctx, (address & 65535i128));
if (size > (ctx.globals.m_native__abi__TETO_GUEST_PAGE_SIZE.expect("global initialisation") - offset)) {
return ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation");
}
let page: f64 = crate::m_native__kernel::guestPage(ctx, memory, hart, address, create);
if ((page == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (page == ctx.globals.m_native__kernel__UNMAPPED_POINTER.expect("global initialisation"))) {
return page;
}
return (page + offset);
}


pub fn guestLoadU8<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut address: i128) -> f64 {
if (!crate::m_native__kernel::guestRange(ctx, memory, hart, address, 1.0f64)) {
return (-1.0f64);
}
let pointer: f64 = crate::m_native__kernel::guestPointer(ctx, memory, hart, address, 1.0f64, false);
if (pointer == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return (-1.0f64);
}
return (if (pointer == ctx.globals.m_native__kernel__UNMAPPED_POINTER.expect("global initialisation")) { 0.0f64 } else { crate::m_native__memory::loadU8(ctx, memory, pointer) });
}


pub fn guestLoadU64<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut address: i128, mut size: f64) -> i128 {
let mut value: i128 = 0i128;
let mut index: f64 = 0.0f64;
'__crust_v38: while (index < size) {
{
let byte: f64 = crate::m_native__kernel::guestLoadU8(ctx, memory, hart, (address).checked_add(crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, index))).expect("BigInt arithmetic"));
if (byte < 0.0f64) {
return 0i128;
}
let _ = { value = (value | numbers::big_shl(crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, byte)), crate::m_native__word::word(ctx, (index * 8.0f64)))); value };
let _ = { index = (index + 1.0f64); index };
}
}
return value;
}


pub fn guestStoreU64<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut address: i128, mut value: i128, mut size: f64) -> bool {
if (!crate::m_native__kernel::guestRange(ctx, memory, hart, address, size)) {
return false;
}
let _ = crate::m_native__kernel::traceNodePointerWrite(ctx, memory, hart, address, value, size);
let mut index: f64 = 0.0f64;
'__crust_v39: while (index < size) {
{
let pointer: f64 = crate::m_native__kernel::guestPointer(ctx, memory, hart, (address).checked_add(crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, index))).expect("BigInt arithmetic"), 1.0f64, true);
if ((pointer == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (pointer == ctx.globals.m_native__kernel__UNMAPPED_POINTER.expect("global initialisation"))) {
return false;
}
let _ = crate::m_native__memory::storeU8(ctx, memory, pointer, Numeric::BigInt(crate::m_native__word::sx(ctx, numbers::big_shr(value, crate::m_native__word::word(ctx, (index * 8.0f64))), 64.0f64)));
let _ = { index = (index + 1.0f64); index };
}
}
return true;
}


pub fn guestWritable<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut address: i128, mut size: f64) -> bool {
if (!crate::m_native__kernel::guestRange(ctx, memory, hart, address, size)) {
return false;
}
let mut offset: f64 = 0.0f64;
'__crust_v40: while (offset < size) {
{
let current: i128 = (address).checked_add(crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, offset))).expect("BigInt arithmetic");
let page: f64 = crate::m_native__kernel::guestPage(ctx, memory, hart, current, true);
if ((page == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (page == ctx.globals.m_native__kernel__UNMAPPED_POINTER.expect("global initialisation"))) {
return false;
}
let within: f64 = crate::m_native__word::wordToU32(ctx, (current & 65535i128));
let left: f64 = (ctx.globals.m_native__abi__TETO_GUEST_PAGE_SIZE.expect("global initialisation") - within);
let _ = { offset = (offset + (if (left < (size - offset)) { left } else { (size - offset) })); offset };
}
}
return true;
}


pub fn tetoImageBegin<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut virtualTop: i128, mut entry: i128, mut phdr: i128, mut phent: f64, mut phnum: f64) -> f64 {
if ((((((!crate::m_native__kernel::validHart(ctx, memory, hart)) || (virtualTop < (1024i128).checked_mul(1024i128).expect("BigInt arithmetic"))) || (virtualTop > 281474976710655i128)) || (entry < 65536i128)) || (entry >= virtualTop)) || ((phdr >= virtualTop) && (phdr != 0i128))) {
return ctx.globals.m_native__abi__TETO_THX_MEMORY.expect("global initialisation");
}
let proc: f64 = crate::m_native__kernel::processForHart(ctx, memory, hart);
if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return ctx.globals.m_native__abi__TETO_THX_MEMORY.expect("global initialisation");
}
let pageTable: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PAGE_TABLE.expect("global initialisation")));
let pageCapacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PAGE_CAPACITY.expect("global initialisation")));
let segmentTable: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_SEGMENT_TABLE.expect("global initialisation")));
let segmentCapacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_SEGMENT_CAPACITY.expect("global initialisation")));
let mapTable: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_TABLE.expect("global initialisation")));
let mapCapacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_CAPACITY.expect("global initialisation")));
let _ = crate::m_native__memory::fill(ctx, memory, pageTable, (pageCapacity * ctx.globals.m_native__abi__TETO_PAGE_ENTRY_SIZE.expect("global initialisation")), 0.0f64);
let _ = crate::m_native__memory::fill(ctx, memory, segmentTable, (segmentCapacity * ctx.globals.m_native__abi__TETO_SEGMENT_STRIDE.expect("global initialisation")), 0.0f64);
let _ = crate::m_native__memory::fill(ctx, memory, mapTable, (mapCapacity * ctx.globals.m_native__abi__TETO_MAP_STRIDE.expect("global initialisation")), 0.0f64);
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PAGE_COUNT.expect("global initialisation")), 0.0f64);
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_SEGMENT_COUNT.expect("global initialisation")), 0.0f64);
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation")), 0.0f64);
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_NEXT.expect("global initialisation")), 274877906944i128);
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_VIRTUAL_TOP.expect("global initialisation")), virtualTop);
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_IMAGE_FLOOR.expect("global initialisation")), 0i128);
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_IMAGE_ENTRY.expect("global initialisation")), entry);
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_BRK_BASE.expect("global initialisation")), 0i128);
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_BRK.expect("global initialisation")), 0i128);
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_PHDR.expect("global initialisation")), phdr);
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PHENT.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, phent)));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PHNUM.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, phnum)));
let state: f64 = crate::m_native__kernel::hartAt(ctx, hart);
let _ = crate::m_native__memory::storeU64(ctx, memory, (state + ctx.globals.m_native__abi__H_VIRTUAL_TOP.expect("global initialisation")), virtualTop);
let _ = crate::m_native__memory::storeU64(ctx, memory, (state + ctx.globals.m_native__abi__H_PC.expect("global initialisation")), entry);
let _ = crate::m_native__memory::storeU64(ctx, memory, (state + ctx.globals.m_native__abi__H_IMAGE_FLOOR.expect("global initialisation")), 0i128);
let _ = crate::m_native__memory::storeU32(ctx, memory, (state + ctx.globals.m_native__abi__H_PAGE_COUNT.expect("global initialisation")), Numeric::Number(0.0f64));
let _ = crate::m_native__memory::storeI64(ctx, memory, (state + ctx.globals.m_native__abi__H_RESERVATION.expect("global initialisation")), ctx.globals.m_native__kernel__NO_RESERVATION.expect("global initialisation"));
return ctx.globals.m_native__abi__TETO_THX_OK.expect("global initialisation");
}


pub fn tetoImageSegment<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut nameHash: f64, mut nameLength: f64, mut address: i128, mut size: i128, mut flags: f64, mut imageAt: f64, mut length: f64) -> f64 {
let proc: f64 = (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__kernel::processForHart(ctx, memory, hart) } else { ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation") });
if ((((proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || { let __crust_v41 = length; let __crust_v42 = size; numbers::compare_number_bigint(__crust_v41, __crust_v42).is_some_and(|order| order.is_gt()) }) || (flags > 7.0f64)) || (!crate::m_native__kernel::tetoImageContains(ctx, memory, imageAt, length))) {
return ctx.globals.m_native__abi__TETO_THX_SECTION.expect("global initialisation");
}
let top: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_VIRTUAL_TOP.expect("global initialisation")));
let end: i128 = (address).checked_add(size).expect("BigInt arithmetic");
if (((address < 65536i128) || (end < address)) || (end > top)) {
return ctx.globals.m_native__abi__TETO_THX_SECTION.expect("global initialisation");
}
let count: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_SEGMENT_COUNT.expect("global initialisation")));
let capacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_SEGMENT_CAPACITY.expect("global initialisation")));
if (count >= capacity) {
return ctx.globals.m_native__abi__TETO_THX_SECTION.expect("global initialisation");
}
let table: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_SEGMENT_TABLE.expect("global initialisation")));
let mut index: f64 = 0.0f64;
'__crust_v43: while (index < count) {
{
let segment: f64 = (table + (index * ctx.globals.m_native__abi__TETO_SEGMENT_STRIDE.expect("global initialisation")));
let left: i128 = crate::m_native__memory::loadU64(ctx, memory, (segment + ctx.globals.m_native__abi__S_ADDRESS.expect("global initialisation")));
let right: i128 = crate::m_native__memory::loadU64(ctx, memory, (segment + ctx.globals.m_native__abi__S_END.expect("global initialisation")));
if ((address < right) && (end > left)) {
return ctx.globals.m_native__abi__TETO_THX_SECTION.expect("global initialisation");
}
if ((crate::m_native__memory::loadU32(ctx, memory, (segment + ctx.globals.m_native__abi__S_NAME_HASH.expect("global initialisation"))) == nameHash) && (crate::m_native__memory::loadU32(ctx, memory, (segment + ctx.globals.m_native__abi__S_NAME_LENGTH.expect("global initialisation"))) == nameLength)) {
return ctx.globals.m_native__abi__TETO_THX_SECTION.expect("global initialisation");
}
let _ = { index = (index + 1.0f64); index };
}
}
if ((length > 0.0f64) && (!crate::m_native__kernel::guestWritable(ctx, memory, hart, address, length))) {
return ctx.globals.m_native__abi__TETO_THX_MEMORY.expect("global initialisation");
}
let mut copied: f64 = 0.0f64;
'__crust_v44: while (copied < length) {
{
let guest: i128 = (address).checked_add(crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, copied))).expect("BigInt arithmetic");
let frame: f64 = crate::m_native__kernel::guestPage(ctx, memory, hart, guest, false);
if ((frame == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (frame == ctx.globals.m_native__kernel__UNMAPPED_POINTER.expect("global initialisation"))) {
return ctx.globals.m_native__abi__TETO_THX_MEMORY.expect("global initialisation");
}
let within: f64 = crate::m_native__word::wordToU32(ctx, (guest & 65535i128));
let remaining: f64 = (length - copied);
let room: f64 = (ctx.globals.m_native__abi__TETO_GUEST_PAGE_SIZE.expect("global initialisation") - within);
let amount: f64 = (if (room < remaining) { room } else { remaining });
let mut offset: f64 = 0.0f64;
'__crust_v45: while (offset < amount) {
{
let _ = crate::m_native__memory::storeU8(ctx, memory, ((frame + within) + offset), Numeric::BigInt(crate::m_native__word::word(ctx, crate::m_native__memory::loadU8(ctx, memory, ((imageAt + copied) + offset)))));
let _ = { offset = (offset + 1.0f64); offset };
}
}
let _ = { copied = (copied + amount); copied };
}
}
let segment: f64 = (table + (count * ctx.globals.m_native__abi__TETO_SEGMENT_STRIDE.expect("global initialisation")));
let _ = crate::m_native__memory::storeU64(ctx, memory, (segment + ctx.globals.m_native__abi__S_ADDRESS.expect("global initialisation")), address);
let _ = crate::m_native__memory::storeU64(ctx, memory, (segment + ctx.globals.m_native__abi__S_END.expect("global initialisation")), end);
let _ = crate::m_native__memory::storeU32(ctx, memory, (segment + ctx.globals.m_native__abi__S_FLAGS.expect("global initialisation")), Numeric::Number(flags));
let _ = crate::m_native__memory::storeU32(ctx, memory, (segment + ctx.globals.m_native__abi__S_NAME_HASH.expect("global initialisation")), Numeric::Number(nameHash));
let _ = crate::m_native__memory::storeU32(ctx, memory, (segment + ctx.globals.m_native__abi__S_NAME_LENGTH.expect("global initialisation")), Numeric::Number(nameLength));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_SEGMENT_COUNT.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, (count + 1.0f64))));
let floor: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_IMAGE_FLOOR.expect("global initialisation")));
if (end > floor) {
{
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_IMAGE_FLOOR.expect("global initialisation")), end);
let _ = crate::m_native__memory::storeU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_IMAGE_FLOOR.expect("global initialisation")), end);
}
}
return ctx.globals.m_native__abi__TETO_THX_OK.expect("global initialisation");
}


pub fn tetoImageFinish<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut imageBytes: f64) -> f64 {
let proc: f64 = (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__kernel::processForHart(ctx, memory, hart) } else { ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation") });
if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return ctx.globals.m_native__abi__TETO_THX_MEMORY.expect("global initialisation");
}
let count: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_SEGMENT_COUNT.expect("global initialisation")));
let table: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_SEGMENT_TABLE.expect("global initialisation")));
let entry: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_IMAGE_ENTRY.expect("global initialisation")));
let mut index: f64 = 0.0f64;
let mut executable: bool = false;
'__crust_v46: while (index < count) {
{
let segment: f64 = (table + (index * ctx.globals.m_native__abi__TETO_SEGMENT_STRIDE.expect("global initialisation")));
if ((((numbers::and(crate::m_native__memory::loadU32(ctx, memory, (segment + ctx.globals.m_native__abi__S_FLAGS.expect("global initialisation"))), ctx.globals.m_native__abi__TETO_SEGMENT_EXECUTE.expect("global initialisation"))) != 0.0f64) && (entry >= crate::m_native__memory::loadU64(ctx, memory, (segment + ctx.globals.m_native__abi__S_ADDRESS.expect("global initialisation"))))) && (entry < crate::m_native__memory::loadU64(ctx, memory, (segment + ctx.globals.m_native__abi__S_END.expect("global initialisation"))))) {
let _ = { executable = true; executable };
}
let _ = { index = (index + 1.0f64); index };
}
}
if (!executable) {
return ctx.globals.m_native__abi__TETO_THX_SECTION.expect("global initialisation");
}
let floor: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_IMAGE_FLOOR.expect("global initialisation")));
let heap: i128 = ((floor).checked_add(4095i128).expect("BigInt arithmetic") & (4096i128).checked_neg().expect("BigInt negation"));
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_BRK_BASE.expect("global initialisation")), floor);
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_BRK.expect("global initialisation")), heap);
let _ = crate::m_native__kernel::bump(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_IMAGE_LOADS.expect("global initialisation")));
let _ = crate::m_native__memory::atomicAddU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_IMAGE_BYTES.expect("global initialisation")), crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, imageBytes)));
return ctx.globals.m_native__abi__TETO_THX_OK.expect("global initialisation");
}


pub fn tetoStackPrepare<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut bottom: i128, mut stackPointer: i128) -> f64 {
let proc: f64 = (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__kernel::processForHart(ctx, memory, hart) } else { ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation") });
if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let top: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_VIRTUAL_TOP.expect("global initialisation")));
if (((bottom >= stackPointer) || (stackPointer >= top)) || ((top).checked_sub(stackPointer).expect("BigInt arithmetic") > 2147483647i128)) {
return ctx.globals.m_native__abi__TETO_START_RANGE.expect("global initialisation");
}
let length: f64 = crate::m_native__word::wordToU32(ctx, (top).checked_sub(stackPointer).expect("BigInt arithmetic"));
if (!crate::m_native__kernel::guestWritable(ctx, memory, hart, stackPointer, length)) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let mut cleared: f64 = 0.0f64;
'__crust_v47: while (cleared < length) {
{
let guest: i128 = (stackPointer).checked_add(crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, cleared))).expect("BigInt arithmetic");
let frame: f64 = crate::m_native__kernel::guestPage(ctx, memory, hart, guest, false);
if ((frame == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (frame == ctx.globals.m_native__kernel__UNMAPPED_POINTER.expect("global initialisation"))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let within: f64 = crate::m_native__word::wordToU32(ctx, (guest & 65535i128));
let remaining: f64 = (length - cleared);
let room: f64 = (ctx.globals.m_native__abi__TETO_GUEST_PAGE_SIZE.expect("global initialisation") - within);
let amount: f64 = (if (room < remaining) { room } else { remaining });
let _ = crate::m_native__memory::fill(ctx, memory, (frame + within), amount, 0.0f64);
let _ = { cleared = (cleared + amount); cleared };
}
}
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_STACK_BOTTOM.expect("global initialisation")), bottom);
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_STACK_POINTER.expect("global initialisation")), stackPointer);
let _ = crate::m_native__memory::storeU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_STACK_BOTTOM.expect("global initialisation")), bottom);
let _ = crate::m_native__memory::storeU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_STACK_POINTER.expect("global initialisation")), stackPointer);
return ctx.globals.m_native__abi__TETO_START_OK.expect("global initialisation");
}


pub fn tetoStackCopy<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut destination: i128, mut source: f64, mut length: f64) -> f64 {
if (!crate::m_native__kernel::tetoImageContains(ctx, memory, source, length)) {
return ctx.globals.m_native__abi__TETO_START_FORMAT.expect("global initialisation");
}
if ((length > 0.0f64) && (!crate::m_native__kernel::guestWritable(ctx, memory, hart, destination, length))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let mut copied: f64 = 0.0f64;
'__crust_v48: while (copied < length) {
{
let guest: i128 = (destination).checked_add(crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, copied))).expect("BigInt arithmetic");
let frame: f64 = crate::m_native__kernel::guestPage(ctx, memory, hart, guest, false);
if ((frame == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (frame == ctx.globals.m_native__kernel__UNMAPPED_POINTER.expect("global initialisation"))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let within: f64 = crate::m_native__word::wordToU32(ctx, (guest & 65535i128));
let remaining: f64 = (length - copied);
let room: f64 = (ctx.globals.m_native__abi__TETO_GUEST_PAGE_SIZE.expect("global initialisation") - within);
let amount: f64 = (if (room < remaining) { room } else { remaining });
let mut offset: f64 = 0.0f64;
'__crust_v49: while (offset < amount) {
{
let _ = crate::m_native__memory::storeU8(ctx, memory, ((frame + within) + offset), Numeric::BigInt(crate::m_native__word::word(ctx, crate::m_native__memory::loadU8(ctx, memory, ((source + copied) + offset)))));
let _ = { offset = (offset + 1.0f64); offset };
}
}
let _ = { copied = (copied + amount); copied };
}
}
return ctx.globals.m_native__abi__TETO_START_OK.expect("global initialisation");
}


pub fn tetoStackStoreU64<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut address: i128, mut value: i128) -> f64 {
return (if crate::m_native__kernel::guestStoreU64(ctx, memory, hart, address, value, 8.0f64) { ctx.globals.m_native__abi__TETO_START_OK.expect("global initialisation") } else { ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation") });
}


pub fn tetoStackAux<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut at: i128, mut random: i128, mut exec: i128) -> f64 {
let proc: f64 = (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__kernel::processForHart(ctx, memory, hart) } else { ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation") });
if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let ruid: i128 = crate::m_native__word::u32word(ctx, Numeric::BigInt(crate::m_native__word::word(ctx, crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_RUID.expect("global initialisation"))))));
let euid: i128 = crate::m_native__word::u32word(ctx, Numeric::BigInt(crate::m_native__word::word(ctx, crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_EUID.expect("global initialisation"))))));
let rgid: i128 = crate::m_native__word::u32word(ctx, Numeric::BigInt(crate::m_native__word::word(ctx, crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_RGID.expect("global initialisation"))))));
let egid: i128 = crate::m_native__word::u32word(ctx, Numeric::BigInt(crate::m_native__word::word(ctx, crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_EGID.expect("global initialisation"))))));
let secure: i128 = (if ((ruid != euid) || (rgid != egid)) { 1i128 } else { 0i128 });
let mut cursor: i128 = at;
if ((!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, cursor, 3i128, 8.0f64)) || (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, (cursor).checked_add(8i128).expect("BigInt arithmetic"), crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_PHDR.expect("global initialisation"))), 8.0f64))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let _ = { cursor = (cursor).checked_add(16i128).expect("BigInt arithmetic"); cursor };
if ((!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, cursor, 4i128, 8.0f64)) || (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, (cursor).checked_add(8i128).expect("BigInt arithmetic"), crate::m_native__word::u32word(ctx, Numeric::BigInt(crate::m_native__word::word(ctx, crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PHENT.expect("global initialisation")))))), 8.0f64))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let _ = { cursor = (cursor).checked_add(16i128).expect("BigInt arithmetic"); cursor };
if ((!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, cursor, 5i128, 8.0f64)) || (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, (cursor).checked_add(8i128).expect("BigInt arithmetic"), crate::m_native__word::u32word(ctx, Numeric::BigInt(crate::m_native__word::word(ctx, crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PHNUM.expect("global initialisation")))))), 8.0f64))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let _ = { cursor = (cursor).checked_add(16i128).expect("BigInt arithmetic"); cursor };
if ((!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, cursor, 6i128, 8.0f64)) || (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, (cursor).checked_add(8i128).expect("BigInt arithmetic"), 4096i128, 8.0f64))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let _ = { cursor = (cursor).checked_add(16i128).expect("BigInt arithmetic"); cursor };
if ((!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, cursor, 7i128, 8.0f64)) || (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, (cursor).checked_add(8i128).expect("BigInt arithmetic"), 0i128, 8.0f64))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let _ = { cursor = (cursor).checked_add(16i128).expect("BigInt arithmetic"); cursor };
if ((!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, cursor, 8i128, 8.0f64)) || (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, (cursor).checked_add(8i128).expect("BigInt arithmetic"), 0i128, 8.0f64))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let _ = { cursor = (cursor).checked_add(16i128).expect("BigInt arithmetic"); cursor };
if ((!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, cursor, 9i128, 8.0f64)) || (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, (cursor).checked_add(8i128).expect("BigInt arithmetic"), crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_IMAGE_ENTRY.expect("global initialisation"))), 8.0f64))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let _ = { cursor = (cursor).checked_add(16i128).expect("BigInt arithmetic"); cursor };
if ((!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, cursor, 11i128, 8.0f64)) || (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, (cursor).checked_add(8i128).expect("BigInt arithmetic"), ruid, 8.0f64))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let _ = { cursor = (cursor).checked_add(16i128).expect("BigInt arithmetic"); cursor };
if ((!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, cursor, 12i128, 8.0f64)) || (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, (cursor).checked_add(8i128).expect("BigInt arithmetic"), euid, 8.0f64))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let _ = { cursor = (cursor).checked_add(16i128).expect("BigInt arithmetic"); cursor };
if ((!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, cursor, 13i128, 8.0f64)) || (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, (cursor).checked_add(8i128).expect("BigInt arithmetic"), rgid, 8.0f64))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let _ = { cursor = (cursor).checked_add(16i128).expect("BigInt arithmetic"); cursor };
if ((!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, cursor, 14i128, 8.0f64)) || (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, (cursor).checked_add(8i128).expect("BigInt arithmetic"), egid, 8.0f64))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let _ = { cursor = (cursor).checked_add(16i128).expect("BigInt arithmetic"); cursor };
if ((!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, cursor, 23i128, 8.0f64)) || (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, (cursor).checked_add(8i128).expect("BigInt arithmetic"), secure, 8.0f64))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let _ = { cursor = (cursor).checked_add(16i128).expect("BigInt arithmetic"); cursor };
if ((!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, cursor, 25i128, 8.0f64)) || (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, (cursor).checked_add(8i128).expect("BigInt arithmetic"), random, 8.0f64))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let _ = { cursor = (cursor).checked_add(16i128).expect("BigInt arithmetic"); cursor };
if ((!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, cursor, 31i128, 8.0f64)) || (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, (cursor).checked_add(8i128).expect("BigInt arithmetic"), exec, 8.0f64))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let _ = { cursor = (cursor).checked_add(16i128).expect("BigInt arithmetic"); cursor };
if ((!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, cursor, 0i128, 8.0f64)) || (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, (cursor).checked_add(8i128).expect("BigInt arithmetic"), 0i128, 8.0f64))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
return ctx.globals.m_native__abi__TETO_START_OK.expect("global initialisation");
}


pub fn tetoStackFinish<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut stackPointer: i128) -> f64 {
let proc: f64 = (if crate::m_native__kernel::validHart(ctx, memory, hart) { crate::m_native__kernel::processForHart(ctx, memory, hart) } else { ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation") });
if ((proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (stackPointer != crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_STACK_POINTER.expect("global initialisation"))))) {
return ctx.globals.m_native__abi__TETO_START_MEMORY.expect("global initialisation");
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, 2.0f64, crate::m_native__word::sx(ctx, stackPointer, 64.0f64));
let _ = crate::m_native__kernel::bump(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_STARTUP_LOADS.expect("global initialisation")));
return ctx.globals.m_native__abi__TETO_START_OK.expect("global initialisation");
}


pub fn fault<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut code: f64, mut instruction: f64) -> f64 {
let at: f64 = crate::m_native__kernel::hartAt(ctx, hart);
let _ = crate::m_native__memory::storeI32(ctx, memory, (at + ctx.globals.m_native__abi__H_FAULT.expect("global initialisation")), code);
let _ = crate::m_native__memory::storeU32(ctx, memory, (at + ctx.globals.m_native__abi__H_LAST_INSTRUCTION.expect("global initialisation")), Numeric::Number(instruction));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (at + ctx.globals.m_native__abi__H_STATUS.expect("global initialisation")), ctx.globals.m_native__abi__TETO_HART_FAULTED.expect("global initialisation"));
return ctx.globals.m_native__abi__TETO_BATCH_FAULT.expect("global initialisation");
}


pub fn immediateI<P: Platform>(ctx: &Context<P>, mut instruction: f64) -> i128 {
return crate::m_native__word::word(ctx, crate::m_vm__rv64_immediate::rv64ImmediateI(ctx, instruction));
}


pub fn immediateS<P: Platform>(ctx: &Context<P>, mut instruction: f64) -> i128 {
return crate::m_native__word::word(ctx, crate::m_vm__rv64_immediate::rv64ImmediateS(ctx, instruction));
}


pub fn immediateB<P: Platform>(ctx: &Context<P>, mut instruction: f64) -> i128 {
return crate::m_native__word::word(ctx, crate::m_vm__rv64_immediate::rv64ImmediateB(ctx, instruction));
}


pub fn immediateU<P: Platform>(ctx: &Context<P>, mut instruction: f64) -> i128 {
return crate::m_native__word::word(ctx, crate::m_vm__rv64_immediate::rv64ImmediateU(ctx, instruction));
}


pub fn immediateJ<P: Platform>(ctx: &Context<P>, mut instruction: f64) -> i128 {
return crate::m_native__word::word(ctx, crate::m_vm__rv64_immediate::rv64ImmediateJ(ctx, instruction));
}


pub fn compressedJump<P: Platform>(ctx: &Context<P>, mut instruction: f64) -> i128 {
return crate::m_native__word::word(ctx, crate::m_vm__rv64_immediate::rv64CompressedJump(ctx, instruction));
}


pub fn loadInteger<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut funct3: f64, mut address: i128) -> bool {
let size: f64 = (if ((funct3 == 0.0f64) || (funct3 == 4.0f64)) { 1.0f64 } else { (if ((funct3 == 1.0f64) || (funct3 == 5.0f64)) { 2.0f64 } else { (if ((funct3 == 2.0f64) || (funct3 == 6.0f64)) { 4.0f64 } else { (if (funct3 == 3.0f64) { 8.0f64 } else { 0.0f64 }) }) }) });
if ((size == 0.0f64) || (!crate::m_native__kernel::guestAllowed(ctx, memory, hart, crate::m_native__word::ux(ctx, address), size, ctx.globals.m_native__abi__TETO_SEGMENT_READ.expect("global initialisation")))) {
return false;
}
let value: i128 = crate::m_native__kernel::guestLoadU64(ctx, memory, hart, crate::m_native__word::ux(ctx, address), size);
if (funct3 == 0.0f64) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, register, crate::m_native__word::sx(ctx, value, 8.0f64));
}
else {
if (funct3 == 1.0f64) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, register, crate::m_native__word::sx(ctx, value, 16.0f64));
}
else {
if (funct3 == 2.0f64) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, register, crate::m_native__word::sx(ctx, value, 32.0f64));
}
else {
if (funct3 == 3.0f64) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, register, crate::m_native__word::sx(ctx, value, 64.0f64));
}
else {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, register, crate::m_native__word::sx(ctx, value, 64.0f64));
}
}
}
}
return true;
}


pub fn storeInteger<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut funct3: f64, mut address: i128, mut value: i128) -> bool {
let size: f64 = (if (funct3 == 0.0f64) { 1.0f64 } else { (if (funct3 == 1.0f64) { 2.0f64 } else { (if (funct3 == 2.0f64) { 4.0f64 } else { (if (funct3 == 3.0f64) { 8.0f64 } else { 0.0f64 }) }) }) });
return (((size != 0.0f64) && crate::m_native__kernel::guestAllowed(ctx, memory, hart, crate::m_native__word::ux(ctx, address), size, ctx.globals.m_native__abi__TETO_SEGMENT_WRITE.expect("global initialisation"))) && crate::m_native__kernel::guestStoreU64(ctx, memory, hart, crate::m_native__word::ux(ctx, address), crate::m_native__word::ux(ctx, value), size));
}


pub fn loadFloating<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut funct3: f64, mut address: i128) -> bool {
let size: f64 = (if (funct3 == 2.0f64) { 4.0f64 } else { (if (funct3 == 3.0f64) { 8.0f64 } else { 0.0f64 }) });
if ((size == 0.0f64) || (!crate::m_native__kernel::guestAllowed(ctx, memory, hart, crate::m_native__word::ux(ctx, address), size, ctx.globals.m_native__abi__TETO_SEGMENT_READ.expect("global initialisation")))) {
return false;
}
let value: i128 = crate::m_native__kernel::guestLoadU64(ctx, memory, hart, crate::m_native__word::ux(ctx, address), size);
if (funct3 == 2.0f64) {
let _ = crate::m_native__kernel::setF(ctx, memory, hart, register, (18446744069414584320i128 | crate::m_native__word::u32word(ctx, Numeric::BigInt(value))));
}
else {
let _ = crate::m_native__kernel::setF(ctx, memory, hart, register, value);
}
return true;
}


pub fn storeFloating<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut funct3: f64, mut address: i128) -> bool {
let size: f64 = (if (funct3 == 2.0f64) { 4.0f64 } else { (if (funct3 == 3.0f64) { 8.0f64 } else { 0.0f64 }) });
return (((size != 0.0f64) && crate::m_native__kernel::guestAllowed(ctx, memory, hart, crate::m_native__word::ux(ctx, address), size, ctx.globals.m_native__abi__TETO_SEGMENT_WRITE.expect("global initialisation"))) && crate::m_native__kernel::guestStoreU64(ctx, memory, hart, crate::m_native__word::ux(ctx, address), crate::m_native__kernel::f(ctx, memory, hart, register), size));
}


pub fn floatingRaw<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut single: bool) -> i128 {
let value: i128 = crate::m_native__kernel::f(ctx, memory, hart, register);
if (!single) {
return value;
}
return (if (numbers::big_shr(value, 32i128) == 4294967295i128) { (value & 4294967295i128) } else { 2143289344i128 });
}


pub fn floatingValue<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut single: bool) -> f64 {
let raw: i128 = crate::m_native__kernel::floatingRaw(ctx, memory, hart, register, single);
let scratch: f64 = (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_FLOAT_SCRATCH.expect("global initialisation"));
return (if single { crate::m_native__word::bitsToF32(ctx, memory, scratch, raw) } else { crate::m_native__word::bitsToF64(ctx, memory, scratch, raw) });
}


pub fn putFloatingBits<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut value: i128, mut single: bool) -> () {
let _ = crate::m_native__kernel::setF(ctx, memory, hart, register, (if single { (18446744069414584320i128 | crate::m_native__word::u32word(ctx, Numeric::BigInt(value))) } else { crate::m_native__word::ux(ctx, value) }));
}


pub fn putFloating<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut value: f64, mut single: bool) -> () {
let scratch: f64 = (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_FLOAT_SCRATCH.expect("global initialisation"));
let _ = crate::m_native__kernel::putFloatingBits(ctx, memory, hart, register, (if single { crate::m_native__word::f32ToBits(ctx, memory, scratch, crate::m_native__word::roundF32(ctx, value)) } else { crate::m_native__word::f64ToBits(ctx, memory, scratch, value) }), single);
}


pub fn floatingRoundMode<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut encoded: f64) -> f64 {
let mode: f64 = (if (encoded == 7.0f64) { numbers::and(numbers::ushr(crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_FCSR.expect("global initialisation"))), 5.0f64), 7.0f64) } else { encoded });
return (if (mode > 4.0f64) { (-1.0f64) } else { mode });
}


pub fn setFloatingFlags<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut flags: f64) -> () {
let at: f64 = (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_FCSR.expect("global initialisation"));
let _ = crate::m_native__memory::storeU32(ctx, memory, at, Numeric::Number(numbers::or(crate::m_native__memory::loadU32(ctx, memory, at), numbers::and(flags, 31.0f64))));
}


pub fn floatingClass<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut single: bool) -> i128 {
return crate::m_core__rv64_fclass::rv64FloatClass(ctx, crate::m_native__kernel::floatingRaw(ctx, memory, hart, register, single), (if single { 32.0f64 } else { 64.0f64 }));
}


pub fn floatingToInteger<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut value: f64, mut bits: f64, mut unsigned: bool, mut mode: f64) -> i128 {
let mut rounded: f64;
if (mode == 1.0f64) {
let _ = { rounded = crate::m_native__word::floatTrunc(ctx, value); rounded };
}
else {
if (mode == 2.0f64) {
let _ = { rounded = crate::m_native__word::floatFloor(ctx, value); rounded };
}
else {
if (mode == 3.0f64) {
let _ = { rounded = crate::m_native__word::floatCeil(ctx, value); rounded };
}
else {
{
let lower: f64 = crate::m_native__word::floatFloor(ctx, value);
let fraction: f64 = (value - lower);
let _ = { rounded = (if (fraction < 0.5f64) { lower } else { (if (fraction > 0.5f64) { (lower + 1.0f64) } else { (if (mode == 4.0f64) { (if (value < 0.0f64) { lower } else { (lower + 1.0f64) }) } else { (if ((crate::m_native__word::floatTrunc(ctx, (lower / 2.0f64)) * 2.0f64) == lower) { lower } else { (lower + 1.0f64) }) }) }) }); rounded };
}
}
}
}
if crate::m_native__word::floatIsNaN(ctx, rounded) {
{
let _ = crate::m_native__kernel::setFloatingFlags(ctx, memory, hart, 16.0f64);
return (if unsigned { (if (bits == 32.0f64) { crate::m_native__word::sx(ctx, 4294967295i128, 64.0f64) } else { (1i128).checked_neg().expect("BigInt negation") }) } else { (if (bits == 32.0f64) { 2147483647i128 } else { 9223372036854775807i128 }) });
}
}
let mut minimum: f64 = 0.0f64;
let mut exclusiveMaximum: f64 = 4294967296.0f64;
if unsigned {
{
if (bits == 64.0f64) {
let _ = { exclusiveMaximum = 18446744073709552000.0f64; exclusiveMaximum };
}
}
}
else {
if (bits == 32.0f64) {
{
let _ = { minimum = (-2147483648.0f64); minimum };
let _ = { exclusiveMaximum = 2147483648.0f64; exclusiveMaximum };
}
}
else {
{
let _ = { minimum = (-9223372036854776000.0f64); minimum };
let _ = { exclusiveMaximum = 9223372036854776000.0f64; exclusiveMaximum };
}
}
}
if (rounded < minimum) {
{
let _ = crate::m_native__kernel::setFloatingFlags(ctx, memory, hart, 16.0f64);
return (if unsigned { 0i128 } else { (if (bits == 32.0f64) { (2147483648i128).checked_neg().expect("BigInt negation") } else { (9223372036854775808i128).checked_neg().expect("BigInt negation") }) });
}
}
if (rounded >= exclusiveMaximum) {
{
let _ = crate::m_native__kernel::setFloatingFlags(ctx, memory, hart, 16.0f64);
return (if unsigned { (if (bits == 32.0f64) { crate::m_native__word::sx(ctx, 4294967295i128, 64.0f64) } else { (1i128).checked_neg().expect("BigInt negation") }) } else { (if (bits == 32.0f64) { 2147483647i128 } else { 9223372036854775807i128 }) });
}
}
return (if unsigned { crate::m_native__word::sx(ctx, crate::m_native__word::floatToU64(ctx, rounded), 64.0f64) } else { crate::m_native__word::floatToI64(ctx, rounded) });
}


pub fn floatingMultiplyAdd<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut opcode: f64, mut destination: f64, mut instruction: f64, mut sourceA: f64, mut sourceB: f64) -> bool {
let sourceC: f64 = numbers::and(numbers::ushr(instruction, 27.0f64), 31.0f64);
let format: f64 = numbers::and(numbers::ushr(instruction, 25.0f64), 3.0f64);
let single: bool = (format == 0.0f64);
if ((!single) && (format != 1.0f64)) {
return false;
}
if (crate::m_native__kernel::floatingRoundMode(ctx, memory, hart, numbers::and(numbers::ushr(instruction, 12.0f64), 7.0f64)) < 0.0f64) {
return false;
}
let left: f64 = crate::m_native__kernel::floatingValue(ctx, memory, hart, sourceA, single);
let right: f64 = crate::m_native__kernel::floatingValue(ctx, memory, hart, sourceB, single);
let addend: f64 = crate::m_native__kernel::floatingValue(ctx, memory, hart, sourceC, single);
let result: f64 = (if (opcode == 67.0f64) { ((left * right) + addend) } else { (if (opcode == 71.0f64) { ((left * right) - addend) } else { (if (opcode == 75.0f64) { (((-left) * right) + addend) } else { (((-left) * right) - addend) }) }) });
let _ = crate::m_native__kernel::putFloating(ctx, memory, hart, destination, result, single);
return true;
}


pub fn floatingOperation<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut destination: f64, mut funct3: f64, mut top: f64, mut sourceA: f64, mut sourceB: f64, mut instruction: f64) -> bool {
let single: bool = ((numbers::and(top, 1.0f64)) == 0.0f64);
let left: f64 = crate::m_native__kernel::floatingValue(ctx, memory, hart, sourceA, single);
let right: f64 = crate::m_native__kernel::floatingValue(ctx, memory, hart, sourceB, single);
if ((top == 0.0f64) || (top == 1.0f64)) {
{
if (crate::m_native__kernel::floatingRoundMode(ctx, memory, hart, funct3) < 0.0f64) {
return false;
}
let _ = crate::m_native__kernel::putFloating(ctx, memory, hart, destination, (left + right), single);
return true;
}
}
if ((top == 4.0f64) || (top == 5.0f64)) {
{
if (crate::m_native__kernel::floatingRoundMode(ctx, memory, hart, funct3) < 0.0f64) {
return false;
}
let _ = crate::m_native__kernel::putFloating(ctx, memory, hart, destination, (left - right), single);
return true;
}
}
if ((top == 8.0f64) || (top == 9.0f64)) {
{
if (crate::m_native__kernel::floatingRoundMode(ctx, memory, hart, funct3) < 0.0f64) {
return false;
}
let _ = crate::m_native__kernel::putFloating(ctx, memory, hart, destination, (left * right), single);
return true;
}
}
if ((top == 12.0f64) || (top == 13.0f64)) {
{
if (crate::m_native__kernel::floatingRoundMode(ctx, memory, hart, funct3) < 0.0f64) {
return false;
}
if ((right == 0.0f64) && (left != 0.0f64)) {
let _ = crate::m_native__kernel::setFloatingFlags(ctx, memory, hart, 8.0f64);
}
else {
if ((right == 0.0f64) && (left == 0.0f64)) {
let _ = crate::m_native__kernel::setFloatingFlags(ctx, memory, hart, 16.0f64);
}
}
let _ = crate::m_native__kernel::putFloating(ctx, memory, hart, destination, (left / right), single);
return true;
}
}
if ((top == 16.0f64) || (top == 17.0f64)) {
{
if (funct3 > 2.0f64) {
return false;
}
let sign: i128 = (if single { 2147483648i128 } else { 9223372036854775808i128 });
let magnitude: i128 = (crate::m_native__kernel::floatingRaw(ctx, memory, hart, sourceA, single) & (!sign));
let signA: i128 = (crate::m_native__kernel::floatingRaw(ctx, memory, hart, sourceA, single) & sign);
let signB: i128 = (crate::m_native__kernel::floatingRaw(ctx, memory, hart, sourceB, single) & sign);
let _ = crate::m_native__kernel::putFloatingBits(ctx, memory, hart, destination, (magnitude | ((if (funct3 == 0.0f64) { signB } else { (if (funct3 == 1.0f64) { (signB ^ sign) } else { (signA ^ signB) }) }))), single);
return true;
}
}
if ((top == 20.0f64) || (top == 21.0f64)) {
{
if (funct3 > 1.0f64) {
return false;
}
if (crate::m_native__word::floatIsNaN(ctx, left) && crate::m_native__word::floatIsNaN(ctx, right)) {
let _ = crate::m_native__kernel::putFloatingBits(ctx, memory, hart, destination, (if single { 2143289344i128 } else { 9221120237041090560i128 }), single);
}
else {
if crate::m_native__word::floatIsNaN(ctx, left) {
let _ = crate::m_native__kernel::putFloatingBits(ctx, memory, hart, destination, crate::m_native__kernel::floatingRaw(ctx, memory, hart, sourceB, single), single);
}
else {
if crate::m_native__word::floatIsNaN(ctx, right) {
let _ = crate::m_native__kernel::putFloatingBits(ctx, memory, hart, destination, crate::m_native__kernel::floatingRaw(ctx, memory, hart, sourceA, single), single);
}
else {
let _ = crate::m_native__kernel::putFloating(ctx, memory, hart, destination, (if (funct3 == 1.0f64) { crate::m_native__word::floatMax(ctx, left, right) } else { crate::m_native__word::floatMin(ctx, left, right) }), single);
}
}
}
return true;
}
}
if (top == 32.0f64) {
{
if ((sourceB != 1.0f64) || (crate::m_native__kernel::floatingRoundMode(ctx, memory, hart, funct3) < 0.0f64)) {
return false;
}
let _ = crate::m_native__kernel::putFloating(ctx, memory, hart, destination, crate::m_native__kernel::floatingValue(ctx, memory, hart, sourceA, false), true);
return true;
}
}
if (top == 33.0f64) {
{
if ((sourceB != 0.0f64) || (crate::m_native__kernel::floatingRoundMode(ctx, memory, hart, funct3) < 0.0f64)) {
return false;
}
let _ = crate::m_native__kernel::putFloating(ctx, memory, hart, destination, crate::m_native__kernel::floatingValue(ctx, memory, hart, sourceA, true), false);
return true;
}
}
if ((top == 44.0f64) || (top == 45.0f64)) {
{
if ((sourceB != 0.0f64) || (crate::m_native__kernel::floatingRoundMode(ctx, memory, hart, funct3) < 0.0f64)) {
return false;
}
let _ = crate::m_native__kernel::putFloating(ctx, memory, hart, destination, crate::m_native__word::floatSqrt(ctx, left), single);
return true;
}
}
if ((top == 80.0f64) || (top == 81.0f64)) {
{
if (funct3 > 2.0f64) {
return false;
}
if (crate::m_native__word::floatIsNaN(ctx, left) || crate::m_native__word::floatIsNaN(ctx, right)) {
{
if (funct3 != 2.0f64) {
let _ = crate::m_native__kernel::setFloatingFlags(ctx, memory, hart, 16.0f64);
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, destination, 0i128);
}
}
else {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, destination, (if (funct3 == 0.0f64) { (if (left <= right) { 1i128 } else { 0i128 }) } else { (if (funct3 == 1.0f64) { (if (left < right) { 1i128 } else { 0i128 }) } else { (if (left == right) { 1i128 } else { 0i128 }) }) }));
}
return true;
}
}
if ((top == 96.0f64) || (top == 97.0f64)) {
{
if (sourceB > 3.0f64) {
return false;
}
let bits: f64 = (if (sourceB < 2.0f64) { 32.0f64 } else { 64.0f64 });
let unsigned: bool = ((numbers::and(sourceB, 1.0f64)) != 0.0f64);
let mode: f64 = crate::m_native__kernel::floatingRoundMode(ctx, memory, hart, funct3);
if (mode < 0.0f64) {
return false;
}
let result: i128 = crate::m_native__kernel::floatingToInteger(ctx, memory, hart, left, bits, unsigned, mode);
let _ = crate::m_native__kernel::setX(ctx, memory, hart, destination, (if (bits == 32.0f64) { crate::m_native__word::sx(ctx, result, 32.0f64) } else { result }));
return true;
}
}
if ((top == 104.0f64) || (top == 105.0f64)) {
{
if ((sourceB > 3.0f64) || (crate::m_native__kernel::floatingRoundMode(ctx, memory, hart, funct3) < 0.0f64)) {
return false;
}
let source: i128 = crate::m_native__kernel::x(ctx, memory, hart, sourceA);
let value: f64 = (if (sourceB == 0.0f64) { crate::m_native__word::wordToFloat(ctx, crate::m_native__word::sx(ctx, source, 32.0f64)) } else { (if (sourceB == 1.0f64) { crate::m_native__word::unsignedWordToFloat(ctx, crate::m_native__word::u32word(ctx, Numeric::BigInt(source))) } else { (if (sourceB == 2.0f64) { crate::m_native__word::wordToFloat(ctx, source) } else { crate::m_native__word::unsignedWordToFloat(ctx, crate::m_native__word::ux(ctx, source)) }) }) });
let _ = crate::m_native__kernel::putFloating(ctx, memory, hart, destination, value, single);
return true;
}
}
if ((top == 112.0f64) || (top == 113.0f64)) {
{
if (sourceB != 0.0f64) {
return false;
}
if (funct3 == 0.0f64) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, destination, (if single { crate::m_native__word::sx(ctx, crate::m_native__kernel::floatingRaw(ctx, memory, hart, sourceA, true), 32.0f64) } else { crate::m_native__word::sx(ctx, crate::m_native__kernel::floatingRaw(ctx, memory, hart, sourceA, false), 64.0f64) }));
}
else {
if (funct3 == 1.0f64) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, destination, crate::m_native__kernel::floatingClass(ctx, memory, hart, sourceA, single));
}
else {
return false;
}
}
return true;
}
}
if ((top == 120.0f64) || (top == 121.0f64)) {
{
if ((sourceB != 0.0f64) || (funct3 != 0.0f64)) {
return false;
}
let _ = crate::m_native__kernel::putFloatingBits(ctx, memory, hart, destination, (if single { crate::m_native__word::u32word(ctx, Numeric::BigInt(crate::m_native__kernel::x(ctx, memory, hart, sourceA))) } else { crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, sourceA)) }), single);
return true;
}
}
return false;
}


pub fn branch<P: Platform>(ctx: &Context<P>, mut funct3: f64, mut left: i128, mut right: i128) -> f64 {
return crate::m_core__rv64_branch::rv64Branch(ctx, funct3, left, right);
}


pub fn opImmediate<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut funct3: f64, mut instruction: f64, mut left: i128) -> bool {
if (crate::m_core__rv64_immediate_op::rv64ImmediateOpValid(ctx, funct3, instruction) == 0.0f64) {
return false;
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, register, crate::m_core__rv64_immediate_op::rv64ImmediateOpValue(ctx, funct3, instruction, left));
return true;
}


pub fn opImmediateWord<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut funct3: f64, mut instruction: f64, mut left: i128) -> bool {
if (crate::m_core__rv64_word::rv64ImmediateWordValid(ctx, funct3, instruction) == 0.0f64) {
return false;
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, register, crate::m_core__rv64_word::rv64ImmediateWordValue(ctx, funct3, instruction, left));
return true;
}


pub fn opRegister<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut funct3: f64, mut top: f64, mut left: i128, mut right: i128) -> bool {
if (crate::m_core__rv64_register::rv64RegisterValid(ctx, funct3, top) == 0.0f64) {
return false;
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, register, crate::m_core__rv64_register::rv64RegisterValue(ctx, funct3, top, left, right));
return true;
}


pub fn opRegisterWord<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut funct3: f64, mut top: f64, mut left: i128, mut right: i128) -> bool {
if (crate::m_core__rv64_word::rv64RegisterWordValid(ctx, funct3, top) == 0.0f64) {
return false;
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, register, crate::m_core__rv64_word::rv64RegisterWordValue(ctx, funct3, top, left, right));
return true;
}


pub fn csr<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut funct3: f64, mut instruction: f64, mut left: i128, mut source: f64, mut nowMicros: i128) -> bool {
let id: f64 = numbers::ushr(instruction, 20.0f64);
let writable: bool = ((id >= 1.0f64) && (id <= 3.0f64));
let mut old: i128;
if ((id == 3072.0f64) || (id == 3074.0f64)) {
let _ = { old = 0i128; old };
}
else {
if (id == 3073.0f64) {
let _ = { old = nowMicros; old };
}
else {
if (id == 1.0f64) {
let _ = { old = crate::m_native__word::word(ctx, numbers::and(crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_FCSR.expect("global initialisation"))), 31.0f64)); old };
}
else {
if (id == 2.0f64) {
let _ = { old = crate::m_native__word::word(ctx, numbers::and(numbers::ushr(crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_FCSR.expect("global initialisation"))), 5.0f64), 7.0f64)); old };
}
else {
if (id == 3.0f64) {
let _ = { old = crate::m_native__word::word(ctx, numbers::and(crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_FCSR.expect("global initialisation"))), 255.0f64)); old };
}
else {
return false;
}
}
}
}
}
if ((((((funct3 != 1.0f64) && (funct3 != 2.0f64)) && (funct3 != 3.0f64)) && (funct3 != 5.0f64)) && (funct3 != 6.0f64)) && (funct3 != 7.0f64)) {
return false;
}
let value: i128 = (if (funct3 >= 5.0f64) { crate::m_native__word::word(ctx, source) } else { crate::m_native__word::sx(ctx, left, 64.0f64) });
let write: i128 = (if ((funct3 == 1.0f64) || (funct3 == 5.0f64)) { value } else { (if ((funct3 == 2.0f64) || (funct3 == 6.0f64)) { (old | value) } else { (old & (!value)) }) });
let hit: bool = (((funct3 == 1.0f64) || (funct3 == 5.0f64)) || (value != 0i128));
if (hit && (!writable)) {
return false;
}
if hit {
{
let current: f64 = crate::m_native__memory::loadU32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_FCSR.expect("global initialisation")));
if (id == 1.0f64) {
let _ = crate::m_native__memory::storeU32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_FCSR.expect("global initialisation")), Numeric::Number(numbers::or(numbers::and(current, numbers::not(31.0f64)), crate::m_native__word::wordToU32(ctx, (write & 31i128)))));
}
else {
if (id == 2.0f64) {
let _ = crate::m_native__memory::storeU32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_FCSR.expect("global initialisation")), Numeric::Number(numbers::or(numbers::and(current, 31.0f64), numbers::shl(crate::m_native__word::wordToU32(ctx, (write & 7i128)), 5.0f64))));
}
else {
let _ = crate::m_native__memory::storeU32(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_FCSR.expect("global initialisation")), Numeric::Number(crate::m_native__word::wordToU32(ctx, (write & 255i128))));
}
}
}
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, register, old);
return true;
}


pub fn atomicInstruction<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut register: f64, mut funct3: f64, mut instruction: f64, mut address: i128, mut value: i128) -> f64 {
if ((funct3 != 2.0f64) && (funct3 != 3.0f64)) {
return ctx.globals.m_native__abi__TETO_BATCH_FAULT.expect("global initialisation");
}
let size: f64 = (if (funct3 == 2.0f64) { 4.0f64 } else { 8.0f64 });
let ts_fn: f64 = numbers::ushr(instruction, 27.0f64);
let permission: f64 = (if (ts_fn == 2.0f64) { ctx.globals.m_native__abi__TETO_SEGMENT_READ.expect("global initialisation") } else { numbers::or(ctx.globals.m_native__abi__TETO_SEGMENT_READ.expect("global initialisation"), ctx.globals.m_native__abi__TETO_SEGMENT_WRITE.expect("global initialisation")) });
if (!crate::m_native__kernel::guestAllowed(ctx, memory, hart, crate::m_native__word::ux(ctx, address), size, permission)) {
return ctx.globals.m_native__abi__TETO_BATCH_FAULT.expect("global initialisation");
}
let pointer: f64 = crate::m_native__kernel::guestPointer(ctx, memory, hart, crate::m_native__word::ux(ctx, address), size, true);
if ((pointer == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) || (pointer == ctx.globals.m_native__kernel__UNMAPPED_POINTER.expect("global initialisation"))) {
return ctx.globals.m_native__abi__TETO_BATCH_FAULT.expect("global initialisation");
}
let lockAt: f64 = (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_ATOMIC_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, lockAt)) {
return ctx.globals.m_native__abi__TETO_BATCH_CONTENDED.expect("global initialisation");
}
let at: f64 = crate::m_native__kernel::hartAt(ctx, hart);
let old: i128 = (if (funct3 == 2.0f64) { crate::m_native__word::word(ctx, crate::m_native__memory::loadI32(ctx, memory, pointer)) } else { crate::m_native__memory::loadI64(ctx, memory, pointer) });
if (ts_fn == 2.0f64) {
{
if ((numbers::and(numbers::ushr(instruction, 20.0f64), 31.0f64)) != 0.0f64) {
{
let _ = crate::m_native__kernel::release(ctx, memory, lockAt);
return ctx.globals.m_native__abi__TETO_BATCH_FAULT.expect("global initialisation");
}
}
let _ = crate::m_native__memory::storeI64(ctx, memory, (at + ctx.globals.m_native__abi__H_RESERVATION.expect("global initialisation")), address);
let _ = crate::m_native__kernel::setX(ctx, memory, hart, register, old);
let _ = crate::m_native__kernel::release(ctx, memory, lockAt);
return ctx.globals.m_native__abi__TETO_BATCH_BUDGET.expect("global initialisation");
}
}
if (ts_fn == 3.0f64) {
{
let success: bool = (crate::m_native__memory::loadI64(ctx, memory, (at + ctx.globals.m_native__abi__H_RESERVATION.expect("global initialisation"))) == address);
let _ = crate::m_native__memory::storeI64(ctx, memory, (at + ctx.globals.m_native__abi__H_RESERVATION.expect("global initialisation")), ctx.globals.m_native__kernel__NO_RESERVATION.expect("global initialisation"));
if success {
{
if (funct3 == 2.0f64) {
let _ = crate::m_native__memory::storeU32(ctx, memory, pointer, Numeric::BigInt(value));
}
else {
let _ = crate::m_native__memory::storeI64(ctx, memory, pointer, value);
}
}
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, register, (if success { 0i128 } else { 1i128 }));
let _ = crate::m_native__kernel::release(ctx, memory, lockAt);
return ctx.globals.m_native__abi__TETO_BATCH_BUDGET.expect("global initialisation");
}
}
let left: i128 = (if (funct3 == 2.0f64) { crate::m_native__word::sx(ctx, old, 32.0f64) } else { old });
let right: i128 = (if (funct3 == 2.0f64) { crate::m_native__word::sx(ctx, value, 32.0f64) } else { value });
let leftUnsigned: i128 = (if (funct3 == 2.0f64) { crate::m_native__word::u32word(ctx, Numeric::BigInt(old)) } else { crate::m_native__word::ux(ctx, old) });
let rightUnsigned: i128 = (if (funct3 == 2.0f64) { crate::m_native__word::u32word(ctx, Numeric::BigInt(value)) } else { crate::m_native__word::ux(ctx, value) });
let mut output: i128;
if (ts_fn == 0.0f64) {
let _ = { output = (left).checked_add(right).expect("BigInt arithmetic"); output };
}
else {
if (ts_fn == 1.0f64) {
let _ = { output = right; output };
}
else {
if (ts_fn == 4.0f64) {
let _ = { output = (left ^ right); output };
}
else {
if (ts_fn == 8.0f64) {
let _ = { output = (left | right); output };
}
else {
if (ts_fn == 12.0f64) {
let _ = { output = (left & right); output };
}
else {
if (ts_fn == 16.0f64) {
let _ = { output = (if (left < right) { left } else { right }); output };
}
else {
if (ts_fn == 20.0f64) {
let _ = { output = (if (left > right) { left } else { right }); output };
}
else {
if (ts_fn == 24.0f64) {
let _ = { output = crate::m_native__word::sx(ctx, (if (leftUnsigned < rightUnsigned) { leftUnsigned } else { rightUnsigned }), 64.0f64); output };
}
else {
if (ts_fn == 28.0f64) {
let _ = { output = crate::m_native__word::sx(ctx, (if (leftUnsigned > rightUnsigned) { leftUnsigned } else { rightUnsigned }), 64.0f64); output };
}
else {
{
let _ = crate::m_native__kernel::release(ctx, memory, lockAt);
return ctx.globals.m_native__abi__TETO_BATCH_FAULT.expect("global initialisation");
}
}
}
}
}
}
}
}
}
}
if (funct3 == 2.0f64) {
let _ = crate::m_native__memory::storeU32(ctx, memory, pointer, Numeric::BigInt(output));
}
else {
let _ = crate::m_native__memory::storeI64(ctx, memory, pointer, output);
}
let _ = crate::m_native__memory::storeI64(ctx, memory, (at + ctx.globals.m_native__abi__H_RESERVATION.expect("global initialisation")), ctx.globals.m_native__kernel__NO_RESERVATION.expect("global initialisation"));
let _ = crate::m_native__kernel::setX(ctx, memory, hart, register, old);
let _ = crate::m_native__kernel::release(ctx, memory, lockAt);
return ctx.globals.m_native__abi__TETO_BATCH_BUDGET.expect("global initialisation");
}


pub fn procId<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut proc: f64, mut field: f64) -> i128 {
return crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, crate::m_native__word::wordToU32(ctx, crate::m_native__word::word(ctx, crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + field))))));
}


pub fn keepId<P: Platform>(ctx: &Context<P>, mut value: i128) -> bool {
return ((crate::m_native__word::ux(ctx, value) == 4294967295i128) || (crate::m_native__word::ux(ctx, value) == 18446744073709551615i128));
}


pub fn validId<P: Platform>(ctx: &Context<P>, mut value: i128) -> bool {
return ((value >= 0i128) && (crate::m_native__word::ux(ctx, value) < 4294967295i128));
}


pub fn canSetUid<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut proc: f64, mut value: i128) -> bool {
return ((((crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_EUID.expect("global initialisation")) == 0i128) || (value == crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_RUID.expect("global initialisation")))) || (value == crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_EUID.expect("global initialisation")))) || (value == crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_SUID.expect("global initialisation"))));
}


pub fn canSetGid<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut proc: f64, mut value: i128) -> bool {
return ((((crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_EUID.expect("global initialisation")) == 0i128) || (value == crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_RGID.expect("global initialisation")))) || (value == crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_EGID.expect("global initialisation")))) || (value == crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_SGID.expect("global initialisation"))));
}


pub fn storeProcId<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut proc: f64, mut field: f64, mut value: i128) -> () {
return crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + field), crate::m_native__word::wordToI32(ctx, value));
}


pub fn setResUid<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut proc: f64) -> i128 {
let r: i128 = crate::m_native__kernel::x(ctx, memory, hart, 10.0f64);
let e: i128 = crate::m_native__kernel::x(ctx, memory, hart, 11.0f64);
let s: i128 = crate::m_native__kernel::x(ctx, memory, hart, 12.0f64);
if (((((!crate::m_native__kernel::keepId(ctx, r)) && (((!crate::m_native__kernel::validId(ctx, r)) || (!crate::m_native__kernel::canSetUid(ctx, memory, proc, crate::m_native__word::ux(ctx, r))))))) || (((!crate::m_native__kernel::keepId(ctx, e)) && (((!crate::m_native__kernel::validId(ctx, e)) || (!crate::m_native__kernel::canSetUid(ctx, memory, proc, crate::m_native__word::ux(ctx, e)))))))) || (((!crate::m_native__kernel::keepId(ctx, s)) && (((!crate::m_native__kernel::validId(ctx, s)) || (!crate::m_native__kernel::canSetUid(ctx, memory, proc, crate::m_native__word::ux(ctx, s)))))))) {
return (1i128).checked_neg().expect("BigInt negation");
}
let nextR: i128 = (if crate::m_native__kernel::keepId(ctx, r) { crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_RUID.expect("global initialisation")) } else { crate::m_native__word::ux(ctx, r) });
let nextE: i128 = (if crate::m_native__kernel::keepId(ctx, e) { crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_EUID.expect("global initialisation")) } else { crate::m_native__word::ux(ctx, e) });
let nextS: i128 = (if crate::m_native__kernel::keepId(ctx, s) { crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_SUID.expect("global initialisation")) } else { crate::m_native__word::ux(ctx, s) });
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_RUID.expect("global initialisation"), nextR);
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_EUID.expect("global initialisation"), nextE);
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_SUID.expect("global initialisation"), nextS);
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_FSUID.expect("global initialisation"), nextE);
return 0i128;
}


pub fn setResGid<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut proc: f64) -> i128 {
let r: i128 = crate::m_native__kernel::x(ctx, memory, hart, 10.0f64);
let e: i128 = crate::m_native__kernel::x(ctx, memory, hart, 11.0f64);
let s: i128 = crate::m_native__kernel::x(ctx, memory, hart, 12.0f64);
if (((((!crate::m_native__kernel::keepId(ctx, r)) && (((!crate::m_native__kernel::validId(ctx, r)) || (!crate::m_native__kernel::canSetGid(ctx, memory, proc, crate::m_native__word::ux(ctx, r))))))) || (((!crate::m_native__kernel::keepId(ctx, e)) && (((!crate::m_native__kernel::validId(ctx, e)) || (!crate::m_native__kernel::canSetGid(ctx, memory, proc, crate::m_native__word::ux(ctx, e)))))))) || (((!crate::m_native__kernel::keepId(ctx, s)) && (((!crate::m_native__kernel::validId(ctx, s)) || (!crate::m_native__kernel::canSetGid(ctx, memory, proc, crate::m_native__word::ux(ctx, s)))))))) {
return (1i128).checked_neg().expect("BigInt negation");
}
let nextR: i128 = (if crate::m_native__kernel::keepId(ctx, r) { crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_RGID.expect("global initialisation")) } else { crate::m_native__word::ux(ctx, r) });
let nextE: i128 = (if crate::m_native__kernel::keepId(ctx, e) { crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_EGID.expect("global initialisation")) } else { crate::m_native__word::ux(ctx, e) });
let nextS: i128 = (if crate::m_native__kernel::keepId(ctx, s) { crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_SGID.expect("global initialisation")) } else { crate::m_native__word::ux(ctx, s) });
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_RGID.expect("global initialisation"), nextR);
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_EGID.expect("global initialisation"), nextE);
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_SGID.expect("global initialisation"), nextS);
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_FSGID.expect("global initialisation"), nextE);
return 0i128;
}


pub fn setReUid<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut proc: f64) -> i128 {
let r: i128 = crate::m_native__kernel::x(ctx, memory, hart, 10.0f64);
let e: i128 = crate::m_native__kernel::x(ctx, memory, hart, 11.0f64);
let oldR: i128 = crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_RUID.expect("global initialisation"));
if ((((!crate::m_native__kernel::keepId(ctx, r)) && (((!crate::m_native__kernel::validId(ctx, r)) || (!crate::m_native__kernel::canSetUid(ctx, memory, proc, crate::m_native__word::ux(ctx, r))))))) || (((!crate::m_native__kernel::keepId(ctx, e)) && (((!crate::m_native__kernel::validId(ctx, e)) || (!crate::m_native__kernel::canSetUid(ctx, memory, proc, crate::m_native__word::ux(ctx, e)))))))) {
return (1i128).checked_neg().expect("BigInt negation");
}
let nextR: i128 = (if crate::m_native__kernel::keepId(ctx, r) { oldR } else { crate::m_native__word::ux(ctx, r) });
let nextE: i128 = (if crate::m_native__kernel::keepId(ctx, e) { crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_EUID.expect("global initialisation")) } else { crate::m_native__word::ux(ctx, e) });
let nextS: i128 = (if ((!crate::m_native__kernel::keepId(ctx, r)) || (((!crate::m_native__kernel::keepId(ctx, e)) && (crate::m_native__word::ux(ctx, e) != oldR)))) { nextE } else { crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_SUID.expect("global initialisation")) });
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_RUID.expect("global initialisation"), nextR);
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_EUID.expect("global initialisation"), nextE);
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_SUID.expect("global initialisation"), nextS);
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_FSUID.expect("global initialisation"), nextE);
return 0i128;
}


pub fn setReGid<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut proc: f64) -> i128 {
let r: i128 = crate::m_native__kernel::x(ctx, memory, hart, 10.0f64);
let e: i128 = crate::m_native__kernel::x(ctx, memory, hart, 11.0f64);
let oldR: i128 = crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_RGID.expect("global initialisation"));
if ((((!crate::m_native__kernel::keepId(ctx, r)) && (((!crate::m_native__kernel::validId(ctx, r)) || (!crate::m_native__kernel::canSetGid(ctx, memory, proc, crate::m_native__word::ux(ctx, r))))))) || (((!crate::m_native__kernel::keepId(ctx, e)) && (((!crate::m_native__kernel::validId(ctx, e)) || (!crate::m_native__kernel::canSetGid(ctx, memory, proc, crate::m_native__word::ux(ctx, e)))))))) {
return (1i128).checked_neg().expect("BigInt negation");
}
let nextR: i128 = (if crate::m_native__kernel::keepId(ctx, r) { oldR } else { crate::m_native__word::ux(ctx, r) });
let nextE: i128 = (if crate::m_native__kernel::keepId(ctx, e) { crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_EGID.expect("global initialisation")) } else { crate::m_native__word::ux(ctx, e) });
let nextS: i128 = (if ((!crate::m_native__kernel::keepId(ctx, r)) || (((!crate::m_native__kernel::keepId(ctx, e)) && (crate::m_native__word::ux(ctx, e) != oldR)))) { nextE } else { crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_SGID.expect("global initialisation")) });
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_RGID.expect("global initialisation"), nextR);
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_EGID.expect("global initialisation"), nextE);
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_SGID.expect("global initialisation"), nextS);
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_FSGID.expect("global initialisation"), nextE);
return 0i128;
}


pub fn setUid<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut proc: f64) -> i128 {
let value: i128 = crate::m_native__kernel::x(ctx, memory, hart, 10.0f64);
if (!crate::m_native__kernel::validId(ctx, value)) {
return (22i128).checked_neg().expect("BigInt negation");
}
let id: i128 = crate::m_native__word::ux(ctx, value);
if (!crate::m_native__kernel::canSetUid(ctx, memory, proc, id)) {
return (1i128).checked_neg().expect("BigInt negation");
}
if (crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_EUID.expect("global initialisation")) == 0i128) {
{
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_RUID.expect("global initialisation"), id);
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_SUID.expect("global initialisation"), id);
}
}
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_EUID.expect("global initialisation"), id);
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_FSUID.expect("global initialisation"), id);
return 0i128;
}


pub fn setGid<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut proc: f64) -> i128 {
let value: i128 = crate::m_native__kernel::x(ctx, memory, hart, 10.0f64);
if (!crate::m_native__kernel::validId(ctx, value)) {
return (22i128).checked_neg().expect("BigInt negation");
}
let id: i128 = crate::m_native__word::ux(ctx, value);
if (!crate::m_native__kernel::canSetGid(ctx, memory, proc, id)) {
return (1i128).checked_neg().expect("BigInt negation");
}
if (crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_EUID.expect("global initialisation")) == 0i128) {
{
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_RGID.expect("global initialisation"), id);
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_SGID.expect("global initialisation"), id);
}
}
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_EGID.expect("global initialisation"), id);
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, ctx.globals.m_native__abi__P_FSGID.expect("global initialisation"), id);
return 0i128;
}


pub fn getResId<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut proc: f64, mut group: bool) -> i128 {
let a: i128 = crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 10.0f64));
let b: i128 = crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 11.0f64));
let c: i128 = crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 12.0f64));
if (((((((((a == 0i128) || (b == 0i128)) || (c == 0i128)) || (!crate::m_native__kernel::guestAllowed(ctx, memory, hart, a, 4.0f64, ctx.globals.m_native__abi__TETO_SEGMENT_WRITE.expect("global initialisation")))) || (!crate::m_native__kernel::guestAllowed(ctx, memory, hart, b, 4.0f64, ctx.globals.m_native__abi__TETO_SEGMENT_WRITE.expect("global initialisation")))) || (!crate::m_native__kernel::guestAllowed(ctx, memory, hart, c, 4.0f64, ctx.globals.m_native__abi__TETO_SEGMENT_WRITE.expect("global initialisation")))) || (!crate::m_native__kernel::guestWritable(ctx, memory, hart, a, 4.0f64))) || (!crate::m_native__kernel::guestWritable(ctx, memory, hart, b, 4.0f64))) || (!crate::m_native__kernel::guestWritable(ctx, memory, hart, c, 4.0f64))) {
return (14i128).checked_neg().expect("BigInt negation");
}
let first: i128 = crate::m_native__kernel::procId(ctx, memory, proc, (if group { ctx.globals.m_native__abi__P_RGID.expect("global initialisation") } else { ctx.globals.m_native__abi__P_RUID.expect("global initialisation") }));
let second: i128 = crate::m_native__kernel::procId(ctx, memory, proc, (if group { ctx.globals.m_native__abi__P_EGID.expect("global initialisation") } else { ctx.globals.m_native__abi__P_EUID.expect("global initialisation") }));
let third: i128 = crate::m_native__kernel::procId(ctx, memory, proc, (if group { ctx.globals.m_native__abi__P_SGID.expect("global initialisation") } else { ctx.globals.m_native__abi__P_SUID.expect("global initialisation") }));
if (((!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, a, first, 4.0f64)) || (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, b, second, 4.0f64))) || (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, c, third, 4.0f64))) {
return (12i128).checked_neg().expect("BigInt negation");
}
return 0i128;
}


pub fn getGroups<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut proc: f64) -> i128 {
let wideCount: i128 = crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 10.0f64));
if (wideCount > 65536i128) {
return (22i128).checked_neg().expect("BigInt negation");
}
let count: f64 = crate::m_native__word::wordToU32(ctx, crate::m_native__word::word(ctx, crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_GROUP_COUNT.expect("global initialisation")))));
if (wideCount == 0i128) {
return crate::m_native__word::word(ctx, count);
}
if (wideCount < crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, count))) {
return (22i128).checked_neg().expect("BigInt negation");
}
let address: i128 = crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 11.0f64));
if (((address == 0i128) || (!crate::m_native__kernel::guestAllowed(ctx, memory, hart, address, (count * 4.0f64), ctx.globals.m_native__abi__TETO_SEGMENT_WRITE.expect("global initialisation")))) || (!crate::m_native__kernel::guestWritable(ctx, memory, hart, address, (count * 4.0f64)))) {
return (14i128).checked_neg().expect("BigInt negation");
}
let mut index: f64 = 0.0f64;
'__crust_v50: while (index < count) {
{
let gid: i128 = crate::m_native__kernel::procId(ctx, memory, proc, (ctx.globals.m_native__abi__P_GROUPS.expect("global initialisation") + (index * 4.0f64)));
if (!crate::m_native__kernel::guestStoreU64(ctx, memory, hart, (address).checked_add(crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, (index * 4.0f64)))).expect("BigInt arithmetic"), gid, 4.0f64)) {
return (12i128).checked_neg().expect("BigInt negation");
}
let _ = { index = (index + 1.0f64); index };
}
}
return crate::m_native__word::word(ctx, count);
}


pub fn setGroups<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut proc: f64) -> i128 {
if (crate::m_native__kernel::procId(ctx, memory, proc, ctx.globals.m_native__abi__P_EUID.expect("global initialisation")) != 0i128) {
return (1i128).checked_neg().expect("BigInt negation");
}
let wideCount: i128 = crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 10.0f64));
if (wideCount > crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, ctx.globals.m_native__abi__TETO_GROUP_CAPACITY.expect("global initialisation")))) {
return (22i128).checked_neg().expect("BigInt negation");
}
let count: f64 = crate::m_native__word::wordToU32(ctx, wideCount);
let address: i128 = crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 11.0f64));
if ((count != 0.0f64) && (((address == 0i128) || (!crate::m_native__kernel::guestAllowed(ctx, memory, hart, address, (count * 4.0f64), ctx.globals.m_native__abi__TETO_SEGMENT_READ.expect("global initialisation")))))) {
return (14i128).checked_neg().expect("BigInt negation");
}
let mut index: f64 = 0.0f64;
'__crust_v51: while (index < count) {
{
let value: i128 = crate::m_native__kernel::guestLoadU64(ctx, memory, hart, (address).checked_add(crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, (index * 4.0f64)))).expect("BigInt arithmetic"), 4.0f64);
if (value >= 4294967295i128) {
return (22i128).checked_neg().expect("BigInt negation");
}
let _ = { index = (index + 1.0f64); index };
}
}
let _ = { index = 0.0f64; index };
'__crust_v52: while (index < count) {
{
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, (ctx.globals.m_native__abi__P_GROUPS.expect("global initialisation") + (index * 4.0f64)), crate::m_native__kernel::guestLoadU64(ctx, memory, hart, (address).checked_add(crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, (index * 4.0f64)))).expect("BigInt arithmetic"), 4.0f64));
let _ = { index = (index + 1.0f64); index };
}
}
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_GROUP_COUNT.expect("global initialisation")), crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, count)));
return 0i128;
}


pub fn credentialSyscall<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut proc: f64, mut number: f64) -> i128 {
if (number == 143.0f64) {
return crate::m_native__kernel::setReGid(ctx, memory, hart, proc);
}
if (number == 144.0f64) {
return crate::m_native__kernel::setGid(ctx, memory, hart, proc);
}
if (number == 145.0f64) {
return crate::m_native__kernel::setReUid(ctx, memory, hart, proc);
}
if (number == 146.0f64) {
return crate::m_native__kernel::setUid(ctx, memory, hart, proc);
}
if (number == 147.0f64) {
return crate::m_native__kernel::setResUid(ctx, memory, hart, proc);
}
if (number == 148.0f64) {
return crate::m_native__kernel::getResId(ctx, memory, hart, proc, false);
}
if (number == 149.0f64) {
return crate::m_native__kernel::setResGid(ctx, memory, hart, proc);
}
if (number == 150.0f64) {
return crate::m_native__kernel::getResId(ctx, memory, hart, proc, true);
}
if ((number == 151.0f64) || (number == 152.0f64)) {
{
let field: f64 = (if (number == 151.0f64) { ctx.globals.m_native__abi__P_FSUID.expect("global initialisation") } else { ctx.globals.m_native__abi__P_FSGID.expect("global initialisation") });
let old: i128 = crate::m_native__kernel::procId(ctx, memory, proc, field);
let value: i128 = crate::m_native__kernel::x(ctx, memory, hart, 10.0f64);
if (!crate::m_native__kernel::validId(ctx, value)) {
return (22i128).checked_neg().expect("BigInt negation");
}
if (if (number == 151.0f64) { crate::m_native__kernel::canSetUid(ctx, memory, proc, crate::m_native__word::ux(ctx, value)) } else { crate::m_native__kernel::canSetGid(ctx, memory, proc, crate::m_native__word::ux(ctx, value)) }) {
{
let _ = crate::m_native__kernel::storeProcId(ctx, memory, proc, field, crate::m_native__word::ux(ctx, value));
}
}
return old;
}
}
if (number == 158.0f64) {
return crate::m_native__kernel::getGroups(ctx, memory, hart, proc);
}
if (number == 159.0f64) {
return crate::m_native__kernel::setGroups(ctx, memory, hart, proc);
}
return (38i128).checked_neg().expect("BigInt negation");
}


pub fn internalSyscall<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut result: i128) -> f64 {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, 10.0f64, result);
let _ = crate::m_native__kernel::bump(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_INTERNAL_SYSCALLS.expect("global initialisation")));
return ctx.globals.m_native__abi__TETO_BATCH_BUDGET.expect("global initialisation");
}


pub fn programBreak<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut proc: f64) -> f64 {
let lock: f64 = (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, lock)) {
{
let _ = crate::m_native__kernel::setPc(ctx, memory, hart, (crate::m_native__memory::loadU64(ctx, memory, (crate::m_native__kernel::hartAt(ctx, hart) + ctx.globals.m_native__abi__H_PC.expect("global initialisation")))).checked_sub(4i128).expect("BigInt arithmetic"));
return ctx.globals.m_native__abi__TETO_BATCH_CONTENDED.expect("global initialisation");
}
}
let current: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_BRK.expect("global initialisation")));
let requested: i128 = crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 10.0f64));
if (requested == 0i128) {
{
let _ = crate::m_native__kernel::release(ctx, memory, lock);
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, crate::m_native__word::sx(ctx, current, 64.0f64));
}
}
let base: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_BRK_BASE.expect("global initialisation")));
let stack: i128 = crate::m_native__memory::atomicLoadU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_STACK_BOTTOM.expect("global initialisation")));
let mut collision: bool = false;
let mapCount: f64 = crate::m_native__memory::loadU32(ctx, memory, (proc + ctx.globals.m_native__abi__P_MAP_COUNT.expect("global initialisation")));
let mut mapIndex: f64 = 0.0f64;
'__crust_v53: while (mapIndex < mapCount) {
{
let mapping: f64 = crate::m_native__kernel::mapAt(ctx, memory, proc, mapIndex);
let start: i128 = crate::m_native__memory::loadU64(ctx, memory, (mapping + ctx.globals.m_native__abi__M_ADDRESS.expect("global initialisation")));
if ((start <= ctx.globals.m_native__kernel__MAP_GUARD.expect("global initialisation")) || (requested >= (start).checked_sub(ctx.globals.m_native__kernel__MAP_GUARD.expect("global initialisation")).expect("BigInt arithmetic"))) {
let _ = { collision = true; collision };
}
let _ = { mapIndex = (mapIndex + 1.0f64); mapIndex };
}
}
if (((((requested < base) || (stack == 0i128)) || (requested >= stack)) || ((stack).checked_sub(requested).expect("BigInt arithmetic") <= ctx.globals.m_native__kernel__MAP_GUARD.expect("global initialisation"))) || collision) {
{
let _ = crate::m_native__kernel::release(ctx, memory, lock);
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, crate::m_native__word::sx(ctx, current, 64.0f64));
}
}
if (requested < current) {
let _ = crate::m_native__kernel::clearMappedRange(ctx, memory, proc, requested, current);
}
let _ = crate::m_native__memory::atomicStoreU64(ctx, memory, (proc + ctx.globals.m_native__abi__P_BRK.expect("global initialisation")), requested);
let _ = crate::m_native__kernel::release(ctx, memory, lock);
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, crate::m_native__word::sx(ctx, requested, 64.0f64));
}


pub fn fallbackSyscall<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> f64 {
let at: f64 = crate::m_native__kernel::hartAt(ctx, hart);
let proc: f64 = crate::m_native__kernel::processForHart(ctx, memory, hart);
let _ = crate::m_native__kernel::bump(ctx, memory, (at + ctx.globals.m_native__abi__H_FALLBACK_SYSCALLS.expect("global initialisation")));
if (proc != ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_STATE.expect("global initialisation")), ctx.globals.m_native__abi__TETO_PROCESS_WAITING.expect("global initialisation"));
}
let _ = crate::m_native__memory::storeI32(ctx, memory, (at + ctx.globals.m_native__abi__H_EVENT.expect("global initialisation")), ctx.globals.m_native__abi__TETO_EVENT_SYSCALL.expect("global initialisation"));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (at + ctx.globals.m_native__abi__H_STATUS.expect("global initialisation")), ctx.globals.m_native__abi__TETO_HART_WAITING.expect("global initialisation"));
return ctx.globals.m_native__abi__TETO_BATCH_SYSCALL.expect("global initialisation");
}


pub fn kernelSyscall<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64) -> f64 {
let at: f64 = crate::m_native__kernel::hartAt(ctx, hart);
let proc: f64 = crate::m_native__kernel::processForHart(ctx, memory, hart);
let number: f64 = crate::m_native__word::wordToU32(ctx, crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 17.0f64)));
let _ = crate::m_native__kernel::bump(ctx, memory, (at + ctx.globals.m_native__abi__H_KERNEL_SYSCALLS.expect("global initialisation")));
if (crate::m_native__kernel::sharedProcessThread(ctx, memory, hart) && (number == 93.0f64)) {
return crate::m_native__kernel::fallbackSyscall(ctx, memory, hart);
}
if ((number == 93.0f64) || (number == 94.0f64)) {
{
let code: f64 = crate::m_native__word::wordToI32(ctx, crate::m_native__kernel::x(ctx, memory, hart, 10.0f64));
let _ = crate::m_native__memory::storeI32(ctx, memory, (at + ctx.globals.m_native__abi__H_EXIT_CODE.expect("global initialisation")), code);
if (proc != ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
{
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_EXIT_CODE.expect("global initialisation")), code);
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_STATE.expect("global initialisation")), ctx.globals.m_native__abi__TETO_PROCESS_ZOMBIE.expect("global initialisation"));
}
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, 10.0f64, crate::m_native__word::word(ctx, code));
let _ = crate::m_native__kernel::bump(ctx, memory, (at + ctx.globals.m_native__abi__H_INTERNAL_SYSCALLS.expect("global initialisation")));
let _ = crate::m_native__memory::storeI32(ctx, memory, (at + ctx.globals.m_native__abi__H_EVENT.expect("global initialisation")), ctx.globals.m_native__abi__TETO_EVENT_NONE.expect("global initialisation"));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (at + ctx.globals.m_native__abi__H_STATUS.expect("global initialisation")), ctx.globals.m_native__abi__TETO_HART_EXITED.expect("global initialisation"));
return ctx.globals.m_native__abi__TETO_BATCH_EXITED.expect("global initialisation");
}
}
if (number == 64.0f64) {
{
let descriptor: f64 = crate::m_native__word::wordToI32(ctx, crate::m_native__kernel::x(ctx, memory, hart, 10.0f64));
let address: i128 = crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 11.0f64));
let wideLength: i128 = crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 12.0f64));
if ((descriptor != 1.0f64) && (descriptor != 2.0f64)) {
return crate::m_native__kernel::fallbackSyscall(ctx, memory, hart);
}
if (wideLength > 1048576i128) {
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, (22i128).checked_neg().expect("BigInt negation"));
}
let length: f64 = crate::m_native__word::wordToU32(ctx, wideLength);
if (!crate::m_native__kernel::guestAllowed(ctx, memory, hart, address, length, ctx.globals.m_native__abi__TETO_SEGMENT_READ.expect("global initialisation"))) {
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, (14i128).checked_neg().expect("BigInt negation"));
}
if (length == 0.0f64) {
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, 0i128);
}
let _ = crate::m_native__memory::storeI32(ctx, memory, (at + ctx.globals.m_native__abi__H_HOST_OPERATION.expect("global initialisation")), ctx.globals.m_native__abi__TETO_HOST_WRITE.expect("global initialisation"));
let _ = crate::m_native__memory::storeI32(ctx, memory, (at + ctx.globals.m_native__abi__H_HOST_DESCRIPTOR.expect("global initialisation")), descriptor);
let _ = crate::m_native__memory::storeU64(ctx, memory, (at + ctx.globals.m_native__abi__H_HOST_ADDRESS.expect("global initialisation")), address);
let _ = crate::m_native__memory::storeU32(ctx, memory, (at + ctx.globals.m_native__abi__H_HOST_LENGTH.expect("global initialisation")), Numeric::Number(length));
let _ = crate::m_native__kernel::bump(ctx, memory, (at + ctx.globals.m_native__abi__H_HOST_REQUESTS.expect("global initialisation")));
let _ = crate::m_native__memory::storeI32(ctx, memory, (at + ctx.globals.m_native__abi__H_EVENT.expect("global initialisation")), ctx.globals.m_native__abi__TETO_EVENT_HOST.expect("global initialisation"));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (at + ctx.globals.m_native__abi__H_STATUS.expect("global initialisation")), ctx.globals.m_native__abi__TETO_HART_WAITING.expect("global initialisation"));
if (proc != ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_STATE.expect("global initialisation")), ctx.globals.m_native__abi__TETO_PROCESS_WAITING.expect("global initialisation"));
}
return ctx.globals.m_native__abi__TETO_BATCH_HOST.expect("global initialisation");
}
}
if (number == 124.0f64) {
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, 0i128);
}
if (number == 214.0f64) {
{
if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, (3i128).checked_neg().expect("BigInt negation"));
}
return crate::m_native__kernel::programBreak(ctx, memory, hart, proc);
}
}
if ((number == 222.0f64) && (((crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 13.0f64)) & 32i128)) != 0i128)) {
{
if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, (3i128).checked_neg().expect("BigInt negation"));
}
let lock: f64 = (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, lock)) {
{
let _ = crate::m_native__kernel::setPc(ctx, memory, hart, (crate::m_native__memory::loadU64(ctx, memory, (at + ctx.globals.m_native__abi__H_PC.expect("global initialisation")))).checked_sub(4i128).expect("BigInt arithmetic"));
return ctx.globals.m_native__abi__TETO_BATCH_CONTENDED.expect("global initialisation");
}
}
let result: i128 = crate::m_native__kernel::anonymousMap(ctx, memory, proc, crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 10.0f64)), crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 11.0f64)), crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 12.0f64)), crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 13.0f64)));
let _ = crate::m_native__kernel::traceMappingOperation(ctx, memory, number, crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 10.0f64)), crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 11.0f64)), crate::m_native__word::wordToU32(ctx, crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 13.0f64))), crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 12.0f64)), result);
let _ = crate::m_native__kernel::release(ctx, memory, lock);
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, result);
}
}
if (number == 215.0f64) {
{
if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, (3i128).checked_neg().expect("BigInt negation"));
}
let lock: f64 = (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, lock)) {
{
let _ = crate::m_native__kernel::setPc(ctx, memory, hart, (crate::m_native__memory::loadU64(ctx, memory, (at + ctx.globals.m_native__abi__H_PC.expect("global initialisation")))).checked_sub(4i128).expect("BigInt arithmetic"));
return ctx.globals.m_native__abi__TETO_BATCH_CONTENDED.expect("global initialisation");
}
}
let result: i128 = crate::m_native__kernel::anonymousUnmap(ctx, memory, proc, crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 10.0f64)), crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 11.0f64)));
let _ = crate::m_native__kernel::traceMappingOperation(ctx, memory, number, crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 10.0f64)), crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 11.0f64)), 0.0f64, 0i128, result);
let _ = crate::m_native__kernel::release(ctx, memory, lock);
if (result == ctx.globals.m_native__kernel__MAP_NOT_OWNED.expect("global initialisation")) {
return crate::m_native__kernel::fallbackSyscall(ctx, memory, hart);
}
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, result);
}
}
if (((number == 216.0f64) || (number == 226.0f64)) || (number == 227.0f64)) {
{
if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, (3i128).checked_neg().expect("BigInt negation"));
}
let lock: f64 = (proc + ctx.globals.m_native__abi__P_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, lock)) {
{
let _ = crate::m_native__kernel::setPc(ctx, memory, hart, (crate::m_native__memory::loadU64(ctx, memory, (at + ctx.globals.m_native__abi__H_PC.expect("global initialisation")))).checked_sub(4i128).expect("BigInt arithmetic"));
return ctx.globals.m_native__abi__TETO_BATCH_CONTENDED.expect("global initialisation");
}
}
let mut result: i128;
if (number == 216.0f64) {
{
let _ = { result = crate::m_native__kernel::remapAnonymous(ctx, memory, hart, proc, crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 10.0f64)), crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 11.0f64)), crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 12.0f64)), crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 13.0f64)), crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 14.0f64))); result };
}
}
else {
if (number == 226.0f64) {
{
let _ = { result = crate::m_native__kernel::protectMappings(ctx, memory, proc, crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 10.0f64)), crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 11.0f64)), crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 12.0f64))); result };
}
}
else {
{
let _ = { result = crate::m_native__kernel::syncAnonymous(ctx, memory, proc, crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 10.0f64)), crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 11.0f64)), crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 12.0f64))); result };
}
}
}
let _ = crate::m_native__kernel::traceMappingOperation(ctx, memory, number, crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 10.0f64)), crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 11.0f64)), crate::m_native__word::wordToU32(ctx, crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 12.0f64))), crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, 13.0f64)), result);
let _ = crate::m_native__kernel::release(ctx, memory, lock);
if (result == ctx.globals.m_native__kernel__MAP_NOT_OWNED.expect("global initialisation")) {
return crate::m_native__kernel::fallbackSyscall(ctx, memory, hart);
}
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, result);
}
}
if (((((number >= 143.0f64) && (number <= 152.0f64))) || (number == 158.0f64)) || (number == 159.0f64)) {
{
if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, (3i128).checked_neg().expect("BigInt negation"));
}
let lock: f64 = (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_CREDENTIAL_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, lock)) {
{
let _ = crate::m_native__kernel::setPc(ctx, memory, hart, (crate::m_native__memory::loadU64(ctx, memory, (at + ctx.globals.m_native__abi__H_PC.expect("global initialisation")))).checked_sub(4i128).expect("BigInt arithmetic"));
return ctx.globals.m_native__abi__TETO_BATCH_CONTENDED.expect("global initialisation");
}
}
let result: i128 = crate::m_native__kernel::credentialSyscall(ctx, memory, hart, proc, number);
let _ = crate::m_native__kernel::release(ctx, memory, lock);
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, result);
}
}
if (crate::m_native__kernel::sharedProcessThread(ctx, memory, hart) && (number == 178.0f64)) {
return crate::m_native__kernel::fallbackSyscall(ctx, memory, hart);
}
if ((number >= 172.0f64) && (number <= 178.0f64)) {
{
if (proc == ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, (3i128).checked_neg().expect("BigInt negation"));
}
if ((number == 172.0f64) || (number == 178.0f64)) {
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, crate::m_native__word::word(ctx, crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PID.expect("global initialisation")))));
}
if (number == 173.0f64) {
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, crate::m_native__word::word(ctx, crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_PPID.expect("global initialisation")))));
}
if (number == 174.0f64) {
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, crate::m_native__word::word(ctx, crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_RUID.expect("global initialisation")))));
}
if (number == 175.0f64) {
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, crate::m_native__word::word(ctx, crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_EUID.expect("global initialisation")))));
}
if (number == 176.0f64) {
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, crate::m_native__word::word(ctx, crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_RGID.expect("global initialisation")))));
}
if (number == 177.0f64) {
return crate::m_native__kernel::internalSyscall(ctx, memory, hart, crate::m_native__word::word(ctx, crate::m_native__memory::atomicLoadI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_EGID.expect("global initialisation")))));
}
}
}
return crate::m_native__kernel::fallbackSyscall(ctx, memory, hart);
}


pub fn compressedStep<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut instruction: f64, mut atPc: i128) -> f64 {
let quadrant: f64 = numbers::and(instruction, 3.0f64);
let funct: f64 = numbers::ushr(instruction, 13.0f64);
let rd: f64 = numbers::and(numbers::ushr(instruction, 7.0f64), 31.0f64);
let rs: f64 = numbers::and(numbers::ushr(instruction, 2.0f64), 31.0f64);
let primeRd: f64 = (8.0f64 + (numbers::and(numbers::ushr(instruction, 7.0f64), 7.0f64)));
let primeRs: f64 = (8.0f64 + (numbers::and(numbers::ushr(instruction, 2.0f64), 7.0f64)));
let immediate: i128 = crate::m_native__word::sx(ctx, crate::m_native__word::word(ctx, numbers::or((numbers::and(numbers::ushr(instruction, 7.0f64), 32.0f64)), (numbers::and(numbers::ushr(instruction, 2.0f64), 31.0f64)))), 6.0f64);
if (quadrant == 0.0f64) {
{
if (funct == 0.0f64) {
{
let value: f64 = numbers::or(numbers::or(numbers::or(numbers::and(numbers::ushr(instruction, 1.0f64), 960.0f64), numbers::and(numbers::ushr(instruction, 7.0f64), 48.0f64)), numbers::and(numbers::ushr(instruction, 2.0f64), 8.0f64)), numbers::and(numbers::ushr(instruction, 4.0f64), 4.0f64));
if (value == 0.0f64) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, primeRs, (crate::m_native__kernel::x(ctx, memory, hart, 2.0f64)).checked_add(crate::m_native__word::word(ctx, value)).expect("BigInt arithmetic"));
}
}
else {
if (funct == 1.0f64) {
{
let offset: f64 = numbers::or(numbers::and(numbers::shl(instruction, 1.0f64), 192.0f64), numbers::and(numbers::ushr(instruction, 7.0f64), 56.0f64));
if (!crate::m_native__kernel::loadFloating(ctx, memory, hart, primeRs, 3.0f64, (crate::m_native__kernel::x(ctx, memory, hart, primeRd)).checked_add(crate::m_native__word::word(ctx, offset)).expect("BigInt arithmetic"))) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), instruction);
}
}
}
else {
if (funct == 2.0f64) {
{
let offset: f64 = numbers::or(numbers::or(numbers::and(numbers::shl(instruction, 1.0f64), 64.0f64), numbers::and(numbers::ushr(instruction, 7.0f64), 56.0f64)), numbers::and(numbers::ushr(instruction, 4.0f64), 4.0f64));
if (!crate::m_native__kernel::loadInteger(ctx, memory, hart, primeRs, 2.0f64, (crate::m_native__kernel::x(ctx, memory, hart, primeRd)).checked_add(crate::m_native__word::word(ctx, offset)).expect("BigInt arithmetic"))) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), instruction);
}
}
}
else {
if (funct == 3.0f64) {
{
let offset: f64 = numbers::or(numbers::and(numbers::shl(instruction, 1.0f64), 192.0f64), numbers::and(numbers::ushr(instruction, 7.0f64), 56.0f64));
if (!crate::m_native__kernel::loadInteger(ctx, memory, hart, primeRs, 3.0f64, (crate::m_native__kernel::x(ctx, memory, hart, primeRd)).checked_add(crate::m_native__word::word(ctx, offset)).expect("BigInt arithmetic"))) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), instruction);
}
}
}
else {
if (funct == 5.0f64) {
{
let offset: f64 = numbers::or(numbers::and(numbers::shl(instruction, 1.0f64), 192.0f64), numbers::and(numbers::ushr(instruction, 7.0f64), 56.0f64));
if (!crate::m_native__kernel::storeFloating(ctx, memory, hart, primeRs, 3.0f64, (crate::m_native__kernel::x(ctx, memory, hart, primeRd)).checked_add(crate::m_native__word::word(ctx, offset)).expect("BigInt arithmetic"))) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), instruction);
}
}
}
else {
if (funct == 6.0f64) {
{
let offset: f64 = numbers::or(numbers::or(numbers::and(numbers::shl(instruction, 1.0f64), 64.0f64), numbers::and(numbers::ushr(instruction, 7.0f64), 56.0f64)), numbers::and(numbers::ushr(instruction, 4.0f64), 4.0f64));
if (!crate::m_native__kernel::storeInteger(ctx, memory, hart, 2.0f64, (crate::m_native__kernel::x(ctx, memory, hart, primeRd)).checked_add(crate::m_native__word::word(ctx, offset)).expect("BigInt arithmetic"), crate::m_native__kernel::x(ctx, memory, hart, primeRs))) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), instruction);
}
}
}
else {
if (funct == 7.0f64) {
{
let offset: f64 = numbers::or(numbers::and(numbers::shl(instruction, 1.0f64), 192.0f64), numbers::and(numbers::ushr(instruction, 7.0f64), 56.0f64));
if (!crate::m_native__kernel::storeInteger(ctx, memory, hart, 3.0f64, (crate::m_native__kernel::x(ctx, memory, hart, primeRd)).checked_add(crate::m_native__word::word(ctx, offset)).expect("BigInt arithmetic"), crate::m_native__kernel::x(ctx, memory, hart, primeRs))) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), instruction);
}
}
}
else {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
}
}
}
}
}
}
return ctx.globals.m_native__abi__TETO_BATCH_BUDGET.expect("global initialisation");
}
}
if (quadrant == 1.0f64) {
{
if (funct == 0.0f64) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, rd, (crate::m_native__kernel::x(ctx, memory, hart, rd)).checked_add(immediate).expect("BigInt arithmetic"));
}
else {
if (funct == 1.0f64) {
{
if (rd == 0.0f64) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, rd, crate::m_native__word::sx(ctx, (crate::m_native__kernel::x(ctx, memory, hart, rd)).checked_add(immediate).expect("BigInt arithmetic"), 32.0f64));
}
}
else {
if (funct == 2.0f64) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, rd, immediate);
}
else {
if (funct == 3.0f64) {
{
if (rd == 2.0f64) {
{
let value: f64 = numbers::or(numbers::or(numbers::or(numbers::or(numbers::and(numbers::ushr(instruction, 3.0f64), 512.0f64), numbers::and(numbers::ushr(instruction, 2.0f64), 16.0f64)), numbers::and(numbers::shl(instruction, 1.0f64), 64.0f64)), numbers::and(numbers::shl(instruction, 4.0f64), 384.0f64)), numbers::and(numbers::shl(instruction, 3.0f64), 32.0f64));
let offset: i128 = crate::m_native__word::sx(ctx, crate::m_native__word::word(ctx, value), 10.0f64);
if (offset == 0i128) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, 2.0f64, (crate::m_native__kernel::x(ctx, memory, hart, 2.0f64)).checked_add(offset).expect("BigInt arithmetic"));
}
}
else {
{
let value: i128 = crate::m_native__word::sx(ctx, crate::m_native__word::word(ctx, numbers::or(numbers::and(numbers::shl(instruction, 5.0f64), 131072.0f64), numbers::and(numbers::shl(instruction, 10.0f64), 126976.0f64))), 18.0f64);
if ((rd == 0.0f64) || (value == 0i128)) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, rd, value);
}
}
}
}
else {
if (funct == 4.0f64) {
{
let funct2: f64 = numbers::and(numbers::ushr(instruction, 10.0f64), 3.0f64);
let shift: i128 = crate::m_native__word::word(ctx, numbers::or((numbers::and(numbers::ushr(instruction, 7.0f64), 32.0f64)), (numbers::and(numbers::ushr(instruction, 2.0f64), 31.0f64))));
if (funct2 == 0.0f64) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, primeRd, numbers::big_shr(crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, primeRd)), shift));
}
else {
if (funct2 == 1.0f64) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, primeRd, numbers::big_shr(crate::m_native__kernel::x(ctx, memory, hart, primeRd), shift));
}
else {
if (funct2 == 2.0f64) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, primeRd, (crate::m_native__kernel::x(ctx, memory, hart, primeRd) & immediate));
}
else {
{
let high: f64 = numbers::and(numbers::ushr(instruction, 12.0f64), 1.0f64);
let low: f64 = numbers::and(numbers::ushr(instruction, 5.0f64), 3.0f64);
let left: i128 = crate::m_native__kernel::x(ctx, memory, hart, primeRd);
let right: i128 = crate::m_native__kernel::x(ctx, memory, hart, primeRs);
if ((high == 0.0f64) && (low == 0.0f64)) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, primeRd, (left).checked_sub(right).expect("BigInt arithmetic"));
}
else {
if ((high == 0.0f64) && (low == 1.0f64)) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, primeRd, (left ^ right));
}
else {
if ((high == 0.0f64) && (low == 2.0f64)) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, primeRd, (left | right));
}
else {
if ((high == 0.0f64) && (low == 3.0f64)) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, primeRd, (left & right));
}
else {
if ((high == 1.0f64) && (low == 0.0f64)) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, primeRd, crate::m_native__word::sx(ctx, (left).checked_sub(right).expect("BigInt arithmetic"), 32.0f64));
}
else {
if ((high == 1.0f64) && (low == 1.0f64)) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, primeRd, crate::m_native__word::sx(ctx, (left).checked_add(right).expect("BigInt arithmetic"), 32.0f64));
}
else {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
}
}
}
}
}
}
}
}
}
}
}
else {
if (funct == 5.0f64) {
let _ = crate::m_native__kernel::setPc(ctx, memory, hart, (atPc).checked_add(crate::m_native__word::ux(ctx, crate::m_native__kernel::compressedJump(ctx, instruction))).expect("BigInt arithmetic"));
}
else {
if ((funct == 6.0f64) || (funct == 7.0f64)) {
{
let value: f64 = numbers::or(numbers::or(numbers::or(numbers::or(numbers::and(numbers::ushr(instruction, 4.0f64), 256.0f64), numbers::and(numbers::shl(instruction, 1.0f64), 192.0f64)), numbers::and(numbers::shl(instruction, 3.0f64), 32.0f64)), numbers::and(numbers::ushr(instruction, 7.0f64), 24.0f64)), numbers::and(numbers::ushr(instruction, 2.0f64), 6.0f64));
if (if (funct == 6.0f64) { (crate::m_native__kernel::x(ctx, memory, hart, primeRd) == 0i128) } else { (crate::m_native__kernel::x(ctx, memory, hart, primeRd) != 0i128) }) {
{
let _ = crate::m_native__kernel::setPc(ctx, memory, hart, (atPc).checked_add(crate::m_native__word::ux(ctx, crate::m_native__word::sx(ctx, crate::m_native__word::word(ctx, value), 9.0f64))).expect("BigInt arithmetic"));
}
}
}
}
else {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
}
}
}
}
}
}
return ctx.globals.m_native__abi__TETO_BATCH_BUDGET.expect("global initialisation");
}
}
if (quadrant == 2.0f64) {
{
if (funct == 0.0f64) {
{
let shift: i128 = crate::m_native__word::word(ctx, numbers::or((numbers::and(numbers::ushr(instruction, 7.0f64), 32.0f64)), (numbers::and(numbers::ushr(instruction, 2.0f64), 31.0f64))));
if (rd == 0.0f64) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, rd, numbers::big_shl(crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, rd)), shift));
}
}
else {
if (funct == 1.0f64) {
{
let offset: f64 = numbers::or(numbers::or(numbers::and(numbers::shl(instruction, 4.0f64), 448.0f64), numbers::and(numbers::ushr(instruction, 7.0f64), 32.0f64)), numbers::and(numbers::ushr(instruction, 2.0f64), 24.0f64));
if ((rd == 0.0f64) || (!crate::m_native__kernel::loadFloating(ctx, memory, hart, rd, 3.0f64, (crate::m_native__kernel::x(ctx, memory, hart, 2.0f64)).checked_add(crate::m_native__word::word(ctx, offset)).expect("BigInt arithmetic")))) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), instruction);
}
}
}
else {
if (funct == 2.0f64) {
{
let offset: f64 = numbers::or(numbers::or(numbers::and(numbers::shl(instruction, 4.0f64), 192.0f64), numbers::and(numbers::ushr(instruction, 7.0f64), 32.0f64)), numbers::and(numbers::ushr(instruction, 2.0f64), 28.0f64));
if ((rd == 0.0f64) || (!crate::m_native__kernel::loadInteger(ctx, memory, hart, rd, 2.0f64, (crate::m_native__kernel::x(ctx, memory, hart, 2.0f64)).checked_add(crate::m_native__word::word(ctx, offset)).expect("BigInt arithmetic")))) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), instruction);
}
}
}
else {
if (funct == 3.0f64) {
{
let offset: f64 = numbers::or(numbers::or(numbers::and(numbers::shl(instruction, 4.0f64), 448.0f64), numbers::and(numbers::ushr(instruction, 7.0f64), 32.0f64)), numbers::and(numbers::ushr(instruction, 2.0f64), 24.0f64));
if ((rd == 0.0f64) || (!crate::m_native__kernel::loadInteger(ctx, memory, hart, rd, 3.0f64, (crate::m_native__kernel::x(ctx, memory, hart, 2.0f64)).checked_add(crate::m_native__word::word(ctx, offset)).expect("BigInt arithmetic")))) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), instruction);
}
}
}
else {
if (funct == 4.0f64) {
{
let high: f64 = numbers::and(numbers::ushr(instruction, 12.0f64), 1.0f64);
if ((high == 0.0f64) && (rs == 0.0f64)) {
{
if (rd == 0.0f64) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
let _ = crate::m_native__kernel::setPc(ctx, memory, hart, crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, rd)));
}
}
else {
if (high == 0.0f64) {
{
if (rd == 0.0f64) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, rd, crate::m_native__kernel::x(ctx, memory, hart, rs));
}
}
else {
if ((rs == 0.0f64) && (rd == 0.0f64)) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_BREAKPOINT.expect("global initialisation"), instruction);
}
else {
if (rs == 0.0f64) {
{
let target: i128 = crate::m_native__word::ux(ctx, crate::m_native__kernel::x(ctx, memory, hart, rd));
let _ = crate::m_native__kernel::setX(ctx, memory, hart, 1.0f64, crate::m_native__word::sx(ctx, (atPc).checked_add(2i128).expect("BigInt arithmetic"), 64.0f64));
let _ = crate::m_native__kernel::setPc(ctx, memory, hart, target);
}
}
else {
{
if (rd == 0.0f64) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, rd, (crate::m_native__kernel::x(ctx, memory, hart, rd)).checked_add(crate::m_native__kernel::x(ctx, memory, hart, rs)).expect("BigInt arithmetic"));
}
}
}
}
}
}
}
else {
if (funct == 5.0f64) {
{
let offset: f64 = numbers::or(numbers::and(numbers::ushr(instruction, 1.0f64), 448.0f64), numbers::and(numbers::ushr(instruction, 7.0f64), 56.0f64));
if (!crate::m_native__kernel::storeFloating(ctx, memory, hart, rs, 3.0f64, (crate::m_native__kernel::x(ctx, memory, hart, 2.0f64)).checked_add(crate::m_native__word::word(ctx, offset)).expect("BigInt arithmetic"))) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), instruction);
}
}
}
else {
if (funct == 6.0f64) {
{
let offset: f64 = numbers::or(numbers::and(numbers::ushr(instruction, 1.0f64), 192.0f64), numbers::and(numbers::ushr(instruction, 7.0f64), 60.0f64));
if (!crate::m_native__kernel::storeInteger(ctx, memory, hart, 2.0f64, (crate::m_native__kernel::x(ctx, memory, hart, 2.0f64)).checked_add(crate::m_native__word::word(ctx, offset)).expect("BigInt arithmetic"), crate::m_native__kernel::x(ctx, memory, hart, rs))) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), instruction);
}
}
}
else {
if (funct == 7.0f64) {
{
let offset: f64 = numbers::or(numbers::and(numbers::ushr(instruction, 1.0f64), 448.0f64), numbers::and(numbers::ushr(instruction, 7.0f64), 56.0f64));
if (!crate::m_native__kernel::storeInteger(ctx, memory, hart, 3.0f64, (crate::m_native__kernel::x(ctx, memory, hart, 2.0f64)).checked_add(crate::m_native__word::word(ctx, offset)).expect("BigInt arithmetic"), crate::m_native__kernel::x(ctx, memory, hart, rs))) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), instruction);
}
}
}
else {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
}
}
}
}
}
}
}
return ctx.globals.m_native__abi__TETO_BATCH_BUDGET.expect("global initialisation");
}
}
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}


pub fn step<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut nowMicros: i128) -> f64 {
let base: f64 = crate::m_native__kernel::hartAt(ctx, hart);
let atPc: i128 = crate::m_native__memory::loadU64(ctx, memory, (base + ctx.globals.m_native__abi__H_PC.expect("global initialisation")));
if (!crate::m_native__kernel::guestAllowed(ctx, memory, hart, atPc, 2.0f64, ctx.globals.m_native__abi__TETO_SEGMENT_EXECUTE.expect("global initialisation"))) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), 0.0f64);
}
let half: f64 = crate::m_native__word::wordToU32(ctx, crate::m_native__kernel::guestLoadU64(ctx, memory, hart, atPc, 2.0f64));
if ((numbers::and(half, 3.0f64)) != 3.0f64) {
{
let _ = crate::m_native__kernel::setPc(ctx, memory, hart, (atPc).checked_add(2i128).expect("BigInt arithmetic"));
return crate::m_native__kernel::compressedStep(ctx, memory, hart, half, atPc);
}
}
if (!crate::m_native__kernel::guestAllowed(ctx, memory, hart, atPc, 4.0f64, ctx.globals.m_native__abi__TETO_SEGMENT_EXECUTE.expect("global initialisation"))) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), 0.0f64);
}
let instruction: f64 = crate::m_native__word::wordToU32(ctx, crate::m_native__kernel::guestLoadU64(ctx, memory, hart, atPc, 4.0f64));
let _ = crate::m_native__memory::storeU32(ctx, memory, (base + ctx.globals.m_native__abi__H_LAST_INSTRUCTION.expect("global initialisation")), Numeric::Number(instruction));
let _ = crate::m_native__kernel::setPc(ctx, memory, hart, (atPc).checked_add(4i128).expect("BigInt arithmetic"));
let opcode: f64 = numbers::and(instruction, 127.0f64);
let destination: f64 = numbers::and(numbers::ushr(instruction, 7.0f64), 31.0f64);
let funct3: f64 = numbers::and(numbers::ushr(instruction, 12.0f64), 7.0f64);
let sourceA: f64 = numbers::and(numbers::ushr(instruction, 15.0f64), 31.0f64);
let sourceB: f64 = numbers::and(numbers::ushr(instruction, 20.0f64), 31.0f64);
let top: f64 = numbers::ushr(instruction, 25.0f64);
let left: i128 = crate::m_native__kernel::x(ctx, memory, hart, sourceA);
let right: i128 = crate::m_native__kernel::x(ctx, memory, hart, sourceB);
if (opcode == 3.0f64) {
{
if (!crate::m_native__kernel::loadInteger(ctx, memory, hart, destination, funct3, (left).checked_add(crate::m_native__kernel::immediateI(ctx, instruction)).expect("BigInt arithmetic"))) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), instruction);
}
}
}
else {
if (opcode == 7.0f64) {
{
if (!crate::m_native__kernel::loadFloating(ctx, memory, hart, destination, funct3, (left).checked_add(crate::m_native__kernel::immediateI(ctx, instruction)).expect("BigInt arithmetic"))) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), instruction);
}
}
}
else {
if (opcode == 15.0f64) {
{
return ctx.globals.m_native__abi__TETO_BATCH_BUDGET.expect("global initialisation");
}
}
else {
if (opcode == 19.0f64) {
{
if (!crate::m_native__kernel::opImmediate(ctx, memory, hart, destination, funct3, instruction, left)) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
}
}
else {
if (opcode == 23.0f64) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, destination, crate::m_native__word::sx(ctx, (atPc).checked_add(crate::m_native__word::ux(ctx, crate::m_native__kernel::immediateU(ctx, instruction))).expect("BigInt arithmetic"), 64.0f64));
}
else {
if (opcode == 27.0f64) {
{
if (!crate::m_native__kernel::opImmediateWord(ctx, memory, hart, destination, funct3, instruction, left)) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
}
}
else {
if (opcode == 35.0f64) {
{
if (!crate::m_native__kernel::storeInteger(ctx, memory, hart, funct3, (left).checked_add(crate::m_native__kernel::immediateS(ctx, instruction)).expect("BigInt arithmetic"), right)) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), instruction);
}
}
}
else {
if (opcode == 39.0f64) {
{
if (!crate::m_native__kernel::storeFloating(ctx, memory, hart, sourceB, funct3, (left).checked_add(crate::m_native__kernel::immediateS(ctx, instruction)).expect("BigInt arithmetic"))) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_MEMORY.expect("global initialisation"), instruction);
}
}
}
else {
if (opcode == 47.0f64) {
{
let result: f64 = crate::m_native__kernel::atomicInstruction(ctx, memory, hart, destination, funct3, instruction, left, right);
if (result == ctx.globals.m_native__abi__TETO_BATCH_CONTENDED.expect("global initialisation")) {
{
let _ = crate::m_native__kernel::setPc(ctx, memory, hart, atPc);
return result;
}
}
if (result == ctx.globals.m_native__abi__TETO_BATCH_FAULT.expect("global initialisation")) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
}
}
else {
if (opcode == 51.0f64) {
{
if (!crate::m_native__kernel::opRegister(ctx, memory, hart, destination, funct3, top, left, right)) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
}
}
else {
if (opcode == 55.0f64) {
let _ = crate::m_native__kernel::setX(ctx, memory, hart, destination, crate::m_native__kernel::immediateU(ctx, instruction));
}
else {
if (opcode == 59.0f64) {
{
if (!crate::m_native__kernel::opRegisterWord(ctx, memory, hart, destination, funct3, top, left, right)) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
}
}
else {
if ((((opcode == 67.0f64) || (opcode == 71.0f64)) || (opcode == 75.0f64)) || (opcode == 79.0f64)) {
{
if (!crate::m_native__kernel::floatingMultiplyAdd(ctx, memory, hart, opcode, destination, instruction, sourceA, sourceB)) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
}
}
else {
if (opcode == 83.0f64) {
{
if (!crate::m_native__kernel::floatingOperation(ctx, memory, hart, destination, funct3, top, sourceA, sourceB, instruction)) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
}
}
else {
if (opcode == 99.0f64) {
{
let take: f64 = crate::m_native__kernel::branch(ctx, funct3, left, right);
if (take < 0.0f64) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
if (take != 0.0f64) {
let _ = crate::m_native__kernel::setPc(ctx, memory, hart, (atPc).checked_add(crate::m_native__word::ux(ctx, crate::m_native__kernel::immediateB(ctx, instruction))).expect("BigInt arithmetic"));
}
}
}
else {
if (opcode == 103.0f64) {
{
if (funct3 != 0.0f64) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
let target: i128 = (crate::m_native__word::ux(ctx, (left).checked_add(crate::m_native__kernel::immediateI(ctx, instruction)).expect("BigInt arithmetic")) & (!1i128));
let _ = crate::m_native__kernel::setX(ctx, memory, hart, destination, crate::m_native__word::sx(ctx, (atPc).checked_add(4i128).expect("BigInt arithmetic"), 64.0f64));
let _ = crate::m_native__kernel::setPc(ctx, memory, hart, target);
}
}
else {
if (opcode == 111.0f64) {
{
let _ = crate::m_native__kernel::setX(ctx, memory, hart, destination, crate::m_native__word::sx(ctx, (atPc).checked_add(4i128).expect("BigInt arithmetic"), 64.0f64));
let _ = crate::m_native__kernel::setPc(ctx, memory, hart, (atPc).checked_add(crate::m_native__word::ux(ctx, crate::m_native__kernel::immediateJ(ctx, instruction))).expect("BigInt arithmetic"));
}
}
else {
if (opcode == 115.0f64) {
{
if (instruction == 115.0f64) {
{
return crate::m_native__kernel::kernelSyscall(ctx, memory, hart);
}
}
if (instruction == 1048691.0f64) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_BREAKPOINT.expect("global initialisation"), instruction);
}
if (!crate::m_native__kernel::csr(ctx, memory, hart, destination, funct3, instruction, left, sourceA, nowMicros)) {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
}
}
else {
return crate::m_native__kernel::fault(ctx, memory, hart, ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION.expect("global initialisation"), instruction);
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
return ctx.globals.m_native__abi__TETO_BATCH_BUDGET.expect("global initialisation");
}


pub fn runHartLocked<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut budget: f64, mut nowMicros: i128, mut worker: f64) -> f64 {
let at: f64 = crate::m_native__kernel::hartAt(ctx, hart);
let _ = crate::m_native__kernel::bump(ctx, memory, (at + ctx.globals.m_native__abi__H_HOST_TO_WASM.expect("global initialisation")));
let previousWorker: f64 = crate::m_native__memory::loadI32(ctx, memory, (at + ctx.globals.m_native__abi__H_LAST_WORKER.expect("global initialisation")));
if ((previousWorker >= 0.0f64) && (previousWorker != worker)) {
let _ = crate::m_native__kernel::bump(ctx, memory, (at + ctx.globals.m_native__abi__H_MIGRATIONS.expect("global initialisation")));
}
let _ = crate::m_native__memory::storeI32(ctx, memory, (at + ctx.globals.m_native__abi__H_LAST_WORKER.expect("global initialisation")), worker);
let _ = crate::m_native__memory::atomicAddI32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_ACTIVE_WORKERS.expect("global initialisation")), 1.0f64);
let mut result: f64 = ctx.globals.m_native__abi__TETO_BATCH_BUDGET.expect("global initialisation");
let mut count: f64 = 0.0f64;
let status: f64 = crate::m_native__memory::atomicLoadI32(ctx, memory, (at + ctx.globals.m_native__abi__H_STATUS.expect("global initialisation")));
if (status == ctx.globals.m_native__abi__TETO_HART_EXITED.expect("global initialisation")) {
let _ = { result = ctx.globals.m_native__abi__TETO_BATCH_EXITED.expect("global initialisation"); result };
}
else {
if (status == ctx.globals.m_native__abi__TETO_HART_FAULTED.expect("global initialisation")) {
let _ = { result = ctx.globals.m_native__abi__TETO_BATCH_FAULT.expect("global initialisation"); result };
}
else {
if (status != ctx.globals.m_native__abi__TETO_HART_RUNNABLE.expect("global initialisation")) {
let _ = { result = ctx.globals.m_native__abi__TETO_BATCH_BUSY.expect("global initialisation"); result };
}
else {
{
'__crust_v54: while ((count < budget) && (result == ctx.globals.m_native__abi__TETO_BATCH_BUDGET.expect("global initialisation"))) {
{
let _ = { result = crate::m_native__kernel::step(ctx, memory, hart, nowMicros); result };
let _ = crate::m_native__kernel::bump(ctx, memory, (at + ctx.globals.m_native__abi__H_INSTRUCTIONS.expect("global initialisation")));
let _ = { count = (count + 1.0f64); count };
}
}
}
}
}
}
if (result == ctx.globals.m_native__abi__TETO_BATCH_CONTENDED.expect("global initialisation")) {
let _ = crate::m_native__kernel::bump(ctx, memory, (at + ctx.globals.m_native__abi__H_SYNC_WAITS.expect("global initialisation")));
}
let _ = crate::m_native__memory::atomicAddI32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_ACTIVE_WORKERS.expect("global initialisation")), (-1.0f64));
let _ = crate::m_native__kernel::bump(ctx, memory, (at + ctx.globals.m_native__abi__H_WASM_TO_HOST.expect("global initialisation")));
if ((worker >= 0.0f64) && (crate::m_native__word::wordToU32(ctx, crate::m_native__word::word(ctx, worker)) < crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_WORKER_CAPACITY.expect("global initialisation"))))) {
{
let workerState: f64 = crate::m_native__kernel::workerAt(ctx, crate::m_native__word::wordToU32(ctx, crate::m_native__word::word(ctx, worker)));
let _ = crate::m_native__kernel::bump(ctx, memory, (workerState + ctx.globals.m_native__abi__W_BATCHES.expect("global initialisation")));
let _ = crate::m_native__memory::atomicAddU64(ctx, memory, (workerState + ctx.globals.m_native__abi__W_INSTRUCTIONS.expect("global initialisation")), crate::m_native__word::ux(ctx, crate::m_native__word::word(ctx, count)));
}
}
let _ = crate::m_native__kernel::release(ctx, memory, (at + ctx.globals.m_native__abi__H_LOCK.expect("global initialisation")));
return result;
}


pub fn tetoRunRv64Batch<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut budget: f64, mut nowMicros: i128, mut worker: f64) -> f64 {
if (((!crate::m_native__kernel::tetoKernelValid(ctx, memory)) || (!crate::m_native__kernel::validHart(ctx, memory, hart))) || (budget < 1.0f64)) {
return ctx.globals.m_native__abi__TETO_BATCH_FAULT.expect("global initialisation");
}
let at: f64 = crate::m_native__kernel::hartAt(ctx, hart);
if (!crate::m_native__kernel::acquire(ctx, memory, (at + ctx.globals.m_native__abi__H_LOCK.expect("global initialisation")))) {
{
let _ = crate::m_native__kernel::bump(ctx, memory, (at + ctx.globals.m_native__abi__H_LOCK_CONTENTION.expect("global initialisation")));
return ctx.globals.m_native__abi__TETO_BATCH_BUSY.expect("global initialisation");
}
}
return crate::m_native__kernel::runHartLocked(ctx, memory, hart, budget, nowMicros, worker);
}


pub fn tetoRunSchedulerBatch<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut budget: f64, mut nowMicros: i128, mut worker: f64) -> f64 {
if (((!crate::m_native__kernel::tetoKernelValid(ctx, memory)) || (budget < 1.0f64)) || (worker < 0.0f64)) {
return ctx.globals.m_native__abi__TETO_BATCH_FAULT.expect("global initialisation");
}
let workerId: f64 = crate::m_native__word::wordToU32(ctx, crate::m_native__word::word(ctx, worker));
let workerCapacity: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_WORKER_CAPACITY.expect("global initialisation")));
if (workerId >= workerCapacity) {
return ctx.globals.m_native__abi__TETO_BATCH_FAULT.expect("global initialisation");
}
let lock: f64 = (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_SCHED_LOCK.expect("global initialisation"));
if (!crate::m_native__kernel::acquire(ctx, memory, lock)) {
{
let _ = crate::m_native__kernel::bump(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_SCHED_CONTENTION.expect("global initialisation")));
return ctx.globals.m_native__abi__TETO_BATCH_CONTENDED.expect("global initialisation");
}
}
let maxHarts: f64 = crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_MAX_HARTS.expect("global initialisation")));
let start: f64 = (crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_SCHED_CURSOR.expect("global initialisation"))) % maxHarts);
let mut scan: f64 = 0.0f64;
'__crust_v55: while (scan < maxHarts) {
{
let hart: f64 = (((start + scan)) % maxHarts);
let at: f64 = crate::m_native__kernel::hartAt(ctx, hart);
if (crate::m_native__memory::atomicLoadI32(ctx, memory, (at + ctx.globals.m_native__abi__H_STATUS.expect("global initialisation"))) == ctx.globals.m_native__abi__TETO_HART_RUNNABLE.expect("global initialisation")) {
{
if crate::m_native__kernel::acquire(ctx, memory, (at + ctx.globals.m_native__abi__H_LOCK.expect("global initialisation"))) {
{
let _ = crate::m_native__memory::storeU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_SCHED_CURSOR.expect("global initialisation")), Numeric::Number((((hart + 1.0f64)) % maxHarts)));
let _ = crate::m_native__kernel::release(ctx, memory, lock);
let _ = crate::m_native__kernel::bump(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_SCHED_RUNS.expect("global initialisation")));
let _ = crate::m_native__kernel::bump(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_SCHED_CLAIMS.expect("global initialisation")));
let result: f64 = crate::m_native__kernel::runHartLocked(ctx, memory, hart, budget, nowMicros, worker);
return crate::m_native__word::wordToI32(ctx, crate::m_native__word::word(ctx, ((((hart + 1.0f64)) * 256.0f64) + (numbers::and(crate::m_native__word::wordToU32(ctx, crate::m_native__word::word(ctx, result)), 255.0f64)))));
}
}
let _ = crate::m_native__kernel::bump(ctx, memory, (at + ctx.globals.m_native__abi__H_LOCK_CONTENTION.expect("global initialisation")));
}
}
let _ = { scan = (scan + 1.0f64); scan };
}
}
let _ = crate::m_native__kernel::release(ctx, memory, lock);
let _ = crate::m_native__kernel::bump(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_SCHED_IDLE.expect("global initialisation")));
let _ = crate::m_native__kernel::bump(ctx, memory, (crate::m_native__kernel::workerAt(ctx, workerId) + ctx.globals.m_native__abi__W_IDLE.expect("global initialisation")));
return 0.0f64;
}


pub fn tetoWorkerMetric<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut worker: f64, mut offset: f64) -> i128 {
if ((((!crate::m_native__kernel::tetoKernelValid(ctx, memory)) || (worker >= crate::m_native__memory::loadU32(ctx, memory, (ctx.globals.m_native__abi__TETO_CONTROL_BASE.expect("global initialisation") + ctx.globals.m_native__abi__C_WORKER_CAPACITY.expect("global initialisation"))))) || ((offset + 8.0f64) > ctx.globals.m_native__abi__TETO_WORKER_STRIDE.expect("global initialisation"))) || ((numbers::and(offset, 7.0f64)) != 0.0f64)) {
return 0i128;
}
return crate::m_native__memory::atomicLoadU64(ctx, memory, (crate::m_native__kernel::workerAt(ctx, worker) + offset));
}


pub fn tetoResumeSyscall<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut result: i128) -> f64 {
if (!crate::m_native__kernel::validHart(ctx, memory, hart)) {
return ctx.globals.m_native__abi__TETO_FAULT_BAD_STATE.expect("global initialisation");
}
let at: f64 = crate::m_native__kernel::hartAt(ctx, hart);
if (!crate::m_native__kernel::acquire(ctx, memory, (at + ctx.globals.m_native__abi__H_LOCK.expect("global initialisation")))) {
return ctx.globals.m_native__abi__TETO_BATCH_BUSY.expect("global initialisation");
}
if ((crate::m_native__memory::atomicLoadI32(ctx, memory, (at + ctx.globals.m_native__abi__H_STATUS.expect("global initialisation"))) != ctx.globals.m_native__abi__TETO_HART_WAITING.expect("global initialisation")) || (crate::m_native__memory::loadI32(ctx, memory, (at + ctx.globals.m_native__abi__H_EVENT.expect("global initialisation"))) != ctx.globals.m_native__abi__TETO_EVENT_SYSCALL.expect("global initialisation"))) {
{
let _ = crate::m_native__kernel::release(ctx, memory, (at + ctx.globals.m_native__abi__H_LOCK.expect("global initialisation")));
return ctx.globals.m_native__abi__TETO_FAULT_BAD_STATE.expect("global initialisation");
}
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, 10.0f64, result);
let _ = crate::m_native__memory::storeI32(ctx, memory, (at + ctx.globals.m_native__abi__H_EVENT.expect("global initialisation")), ctx.globals.m_native__abi__TETO_EVENT_NONE.expect("global initialisation"));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (at + ctx.globals.m_native__abi__H_STATUS.expect("global initialisation")), ctx.globals.m_native__abi__TETO_HART_RUNNABLE.expect("global initialisation"));
let proc: f64 = crate::m_native__kernel::processForHart(ctx, memory, hart);
if (proc != ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_STATE.expect("global initialisation")), ctx.globals.m_native__abi__TETO_PROCESS_RUNNABLE.expect("global initialisation"));
}
let _ = crate::m_native__kernel::release(ctx, memory, (at + ctx.globals.m_native__abi__H_LOCK.expect("global initialisation")));
return 0.0f64;
}


pub fn tetoResumeHost<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut result: i128) -> f64 {
if (!crate::m_native__kernel::validHart(ctx, memory, hart)) {
return ctx.globals.m_native__abi__TETO_FAULT_BAD_STATE.expect("global initialisation");
}
let at: f64 = crate::m_native__kernel::hartAt(ctx, hart);
if (!crate::m_native__kernel::acquire(ctx, memory, (at + ctx.globals.m_native__abi__H_LOCK.expect("global initialisation")))) {
return ctx.globals.m_native__abi__TETO_BATCH_BUSY.expect("global initialisation");
}
if ((crate::m_native__memory::atomicLoadI32(ctx, memory, (at + ctx.globals.m_native__abi__H_STATUS.expect("global initialisation"))) != ctx.globals.m_native__abi__TETO_HART_WAITING.expect("global initialisation")) || (crate::m_native__memory::loadI32(ctx, memory, (at + ctx.globals.m_native__abi__H_EVENT.expect("global initialisation"))) != ctx.globals.m_native__abi__TETO_EVENT_HOST.expect("global initialisation"))) {
{
let _ = crate::m_native__kernel::release(ctx, memory, (at + ctx.globals.m_native__abi__H_LOCK.expect("global initialisation")));
return ctx.globals.m_native__abi__TETO_FAULT_BAD_STATE.expect("global initialisation");
}
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, 10.0f64, result);
let _ = crate::m_native__memory::storeI32(ctx, memory, (at + ctx.globals.m_native__abi__H_HOST_OPERATION.expect("global initialisation")), ctx.globals.m_native__abi__TETO_HOST_NONE.expect("global initialisation"));
let _ = crate::m_native__memory::storeI32(ctx, memory, (at + ctx.globals.m_native__abi__H_HOST_DESCRIPTOR.expect("global initialisation")), (-1.0f64));
let _ = crate::m_native__memory::storeU64(ctx, memory, (at + ctx.globals.m_native__abi__H_HOST_ADDRESS.expect("global initialisation")), 0i128);
let _ = crate::m_native__memory::storeU32(ctx, memory, (at + ctx.globals.m_native__abi__H_HOST_LENGTH.expect("global initialisation")), Numeric::Number(0.0f64));
let _ = crate::m_native__memory::storeI32(ctx, memory, (at + ctx.globals.m_native__abi__H_EVENT.expect("global initialisation")), ctx.globals.m_native__abi__TETO_EVENT_NONE.expect("global initialisation"));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (at + ctx.globals.m_native__abi__H_STATUS.expect("global initialisation")), ctx.globals.m_native__abi__TETO_HART_RUNNABLE.expect("global initialisation"));
let proc: f64 = crate::m_native__kernel::processForHart(ctx, memory, hart);
if (proc != ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) {
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_STATE.expect("global initialisation")), ctx.globals.m_native__abi__TETO_PROCESS_RUNNABLE.expect("global initialisation"));
}
let _ = crate::m_native__kernel::release(ctx, memory, (at + ctx.globals.m_native__abi__H_LOCK.expect("global initialisation")));
return 0.0f64;
}


pub fn tetoExitHart<P: Platform>(ctx: &Context<P>, mut memory: crate::m_native__memory::TetoMemory, mut hart: f64, mut code: f64) -> f64 {
if (!crate::m_native__kernel::validHart(ctx, memory, hart)) {
return ctx.globals.m_native__abi__TETO_FAULT_BAD_STATE.expect("global initialisation");
}
let at: f64 = crate::m_native__kernel::hartAt(ctx, hart);
if (!crate::m_native__kernel::acquire(ctx, memory, (at + ctx.globals.m_native__abi__H_LOCK.expect("global initialisation")))) {
return ctx.globals.m_native__abi__TETO_BATCH_BUSY.expect("global initialisation");
}
let _ = crate::m_native__kernel::setX(ctx, memory, hart, 10.0f64, crate::m_native__word::word(ctx, code));
let proc: f64 = crate::m_native__kernel::processForHart(ctx, memory, hart);
if ((proc != ctx.globals.m_native__kernel__INVALID_POINTER.expect("global initialisation")) && (!crate::m_native__kernel::sharedProcessThread(ctx, memory, hart))) {
{
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_EXIT_CODE.expect("global initialisation")), code);
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (proc + ctx.globals.m_native__abi__P_STATE.expect("global initialisation")), ctx.globals.m_native__abi__TETO_PROCESS_ZOMBIE.expect("global initialisation"));
}
}
let _ = crate::m_native__memory::storeI32(ctx, memory, (at + ctx.globals.m_native__abi__H_EVENT.expect("global initialisation")), ctx.globals.m_native__abi__TETO_EVENT_NONE.expect("global initialisation"));
let _ = crate::m_native__memory::atomicStoreI32(ctx, memory, (at + ctx.globals.m_native__abi__H_STATUS.expect("global initialisation")), ctx.globals.m_native__abi__TETO_HART_EXITED.expect("global initialisation"));
let _ = crate::m_native__kernel::release(ctx, memory, (at + ctx.globals.m_native__abi__H_LOCK.expect("global initialisation")));
return 0.0f64;
}

