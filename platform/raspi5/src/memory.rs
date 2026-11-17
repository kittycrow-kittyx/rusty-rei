#![forbid(unsafe_code)]

use core::cell::RefCell;
use crust_kernel::{
    numbers, Platform, __bake_c_Buffer as Buffer, __bake_c_View as View,
    __bake_c_U8Array as U8Array, __bake_c_I32Array as I32Array,
    __bake_c_U64Array as U64Array,
};

const MAX_BUFFERS: usize = 64;
#[derive(Clone, Copy)]
struct Allocation { start: usize, len: usize }
struct Arena<'a> {
    bytes: &'a mut [u8],
    allocations: [Allocation; MAX_BUFFERS],
    count: usize,
    used: usize,
}

/// Single-hart runtime. No interrupt handler or other core accesses this arena.
pub struct BoardMemory<'a>(RefCell<Arena<'a>>);

fn index(value: f64) -> usize {
    assert!(value.is_finite() && value >= 0.0 && value <= u32::MAX as f64
        && numbers::trunc(value) == value, "invalid buffer index");
    value as usize
}

fn clamp(value: f64, length: usize) -> usize {
    let value = if value.is_nan() { 0.0 } else { numbers::trunc(value) };
    let value = if value < 0.0 { value + length as f64 } else { value };
    if value <= 0.0 { 0 } else if value >= length as f64 { length } else { value as usize }
}

impl Arena<'_> {
    fn range(&self, buffer: Buffer, offset: usize, size: usize) -> core::ops::Range<usize> {
        let handle = usize::try_from(buffer.handle).expect("invalid handle");
        assert!(handle > 0 && handle <= self.count, "invalid handle");
        let allocation = self.allocations[handle - 1];
        assert!(buffer.byteLength == allocation.len as f64, "invalid buffer length");
        assert!(offset <= allocation.len && size <= allocation.len - offset, "buffer range");
        allocation.start + offset..allocation.start + offset + size
    }
}

impl<'a> BoardMemory<'a> {
    pub fn new(bytes: &'a mut [u8]) -> Self {
        Self(RefCell::new(Arena { bytes, allocations: [Allocation { start: 0, len: 0 }; MAX_BUFFERS],
            count: 0, used: 0 }))
    }

    fn read<const N: usize>(&self, buffer: Buffer, offset: usize) -> [u8; N] {
        let arena = self.0.borrow();
        arena.bytes[arena.range(buffer, offset, N)].try_into().expect("buffer width")
    }

    fn write(&self, buffer: Buffer, offset: usize, value: &[u8]) {
        let mut arena = self.0.borrow_mut();
        let range = arena.range(buffer, offset, value.len());
        arena.bytes[range].copy_from_slice(value);
    }

    fn bits<const N: usize>(&self, view: View, at: f64, le: Option<bool>) -> [u8; N] {
        let mut bytes = self.read(view.buffer, index(at));
        if !le.unwrap_or(false) { bytes.reverse(); }
        bytes
    }

    fn store<const N: usize>(&self, view: View, at: f64, mut bytes: [u8; N], le: Option<bool>) {
        if !le.unwrap_or(false) { bytes.reverse(); }
        self.write(view.buffer, index(at), &bytes);
    }

    fn array_offset(length: f64, at: f64, width: usize) -> usize {
        let i = index(at);
        assert!(i < index(length), "array index");
        i.checked_mul(width).expect("array offset")
    }
}

impl Platform for BoardMemory<'_> {
    fn __bake_c_abort_range(&self) -> ! { panic!("buffer range") }

    fn __bake_c_new_buffer(&self, size: f64) -> Buffer {
        let len = index(size);
        let mut arena = self.0.borrow_mut();
        let start = (arena.used + 15) & !15;
        assert!(arena.count < MAX_BUFFERS && start <= arena.bytes.len()
            && len <= arena.bytes.len() - start, "arena exhausted");
        arena.bytes[start..start + len].fill(0);
        let slot = arena.count;
        arena.allocations[slot] = Allocation { start, len };
        arena.count += 1;
        arena.used = start + len;
        Buffer { handle: arena.count as u64, byteLength: size }
    }
    fn __bake_c_new_shared_buffer(&self, size: f64) -> Buffer { self.__bake_c_new_buffer(size) }
    fn __bake_c_new_view(&self, buffer: Buffer) -> View { View { buffer } }
    fn __bake_c_new_u8_array(&self, buffer: Buffer) -> U8Array { U8Array { buffer, length: buffer.byteLength } }
    fn __bake_c_new_i32_array(&self, buffer: Buffer) -> I32Array {
        assert!(index(buffer.byteLength) % 4 == 0);
        I32Array { buffer, length: buffer.byteLength / 4.0 }
    }
    fn __bake_c_new_u64_array(&self, buffer: Buffer) -> U64Array {
        assert!(index(buffer.byteLength) % 8 == 0);
        U64Array { buffer, length: buffer.byteLength / 8.0 }
    }

    fn __bake_c_u8_get(&self, a: U8Array, at: f64) -> f64 {
        self.read::<1>(a.buffer, Self::array_offset(a.length, at, 1))[0] as f64
    }
    fn __bake_c_u8_set(&self, a: U8Array, at: f64, value: f64) -> f64 {
        let v = numbers::to_u32(value) as u8;
        self.write(a.buffer, Self::array_offset(a.length, at, 1), &[v]);
        v as f64
    }
    fn __bake_c_u8_fill(&self, a: U8Array, value: f64, start: Option<f64>, end: Option<f64>) -> U8Array {
        let length = index(a.length);
        let s = clamp(start.unwrap_or(0.0), length);
        let e = clamp(end.unwrap_or(a.length), length);
        let mut arena = self.0.borrow_mut();
        let range = arena.range(a.buffer, s, e.saturating_sub(s));
        arena.bytes[range].fill(numbers::to_u32(value) as u8);
        a
    }
    fn __bake_c_u8_copy_within(&self, a: U8Array, target: f64, start: f64, end: Option<f64>) -> U8Array {
        let length = index(a.length);
        let t = clamp(target, length);
        let s = clamp(start, length);
        let e = clamp(end.unwrap_or(a.length), length);
        let n = e.saturating_sub(s).min(length - t);
        let mut arena = self.0.borrow_mut();
        let source = arena.range(a.buffer, s, n);
        let dest = arena.range(a.buffer, t, n);
        arena.bytes.copy_within(source, dest.start);
        a
    }
    fn __bake_c_i32_get(&self, a: I32Array, at: f64) -> f64 {
        i32::from_le_bytes(self.read(a.buffer, Self::array_offset(a.length, at, 4))) as f64
    }
    fn __bake_c_i32_set(&self, a: I32Array, at: f64, value: f64) -> f64 {
        let value = numbers::to_i32(value);
        self.write(a.buffer, Self::array_offset(a.length, at, 4), &value.to_le_bytes());
        value as f64
    }
    // With one hart and no IRQ access, no other execution can interleave these operations.
    fn __bake_c_atomic_load_i32(&self, a: I32Array, at: f64) -> f64 { self.__bake_c_i32_get(a, at) }
    fn __bake_c_atomic_store_i32(&self, a: I32Array, at: f64, v: f64) -> f64 { self.__bake_c_i32_set(a, at, v) }
    fn __bake_c_atomic_add_i32(&self, a: I32Array, at: f64, v: f64) -> f64 {
        let old = self.__bake_c_i32_get(a, at) as i32;
        self.__bake_c_i32_set(a, at, old.wrapping_add(numbers::to_i32(v)) as f64);
        old as f64
    }
    fn __bake_c_atomic_cmpxchg_i32(&self, a: I32Array, at: f64, e: f64, v: f64) -> f64 {
        let old = self.__bake_c_i32_get(a, at);
        if old == numbers::to_i32(e) as f64 { self.__bake_c_i32_set(a, at, v); }
        old
    }
    fn __bake_c_atomic_load_u64(&self, a: U64Array, at: f64) -> i128 {
        u64::from_le_bytes(self.read(a.buffer, Self::array_offset(a.length, at, 8))) as i128
    }
    fn __bake_c_atomic_store_u64(&self, a: U64Array, at: f64, v: i128) -> i128 {
        let v = v as u64;
        self.write(a.buffer, Self::array_offset(a.length, at, 8), &v.to_le_bytes());
        v as i128
    }
    fn __bake_c_atomic_add_u64(&self, a: U64Array, at: f64, v: i128) -> i128 {
        let old = self.__bake_c_atomic_load_u64(a, at) as u64;
        self.__bake_c_atomic_store_u64(a, at, old.wrapping_add(v as u64) as i128);
        old as i128
    }
    fn __bake_c_f64_sqrt(&self, value: f64) -> f64 { crate::hardware_sqrt(value) }
    fn __bake_c_view_get_i16(&self, v: View, at: f64, le: Option<bool>) -> f64 { i16::from_le_bytes(self.bits(v, at, le)) as f64 }
    fn __bake_c_view_get_i32(&self, v: View, at: f64, le: Option<bool>) -> f64 { i32::from_le_bytes(self.bits(v, at, le)) as f64 }
    fn __bake_c_view_set_i32(&self, v: View, at: f64, value: f64, le: Option<bool>) { self.store(v, at, (numbers::to_i32(value)).to_le_bytes(), le); }
    fn __bake_c_view_get_i64(&self, v: View, at: f64, le: Option<bool>) -> i128 { i64::from_le_bytes(self.bits(v, at, le)) as i128 }
    fn __bake_c_view_set_i64(&self, v: View, at: f64, value: i128, le: Option<bool>) { self.store(v, at, (value as i64).to_le_bytes(), le); }
    fn __bake_c_view_get_u16(&self, v: View, at: f64, le: Option<bool>) -> f64 { u16::from_le_bytes(self.bits(v, at, le)) as f64 }
    fn __bake_c_view_set_u16(&self, v: View, at: f64, value: f64, le: Option<bool>) { self.store(v, at, (numbers::to_u32(value) as u16).to_le_bytes(), le); }
    fn __bake_c_view_get_u32(&self, v: View, at: f64, le: Option<bool>) -> f64 { u32::from_le_bytes(self.bits(v, at, le)) as f64 }
    fn __bake_c_view_set_u32(&self, v: View, at: f64, value: f64, le: Option<bool>) { self.store(v, at, (numbers::to_u32(value)).to_le_bytes(), le); }
    fn __bake_c_view_get_u64(&self, v: View, at: f64, le: Option<bool>) -> i128 { u64::from_le_bytes(self.bits(v, at, le)) as i128 }
    fn __bake_c_view_set_u64(&self, v: View, at: f64, value: i128, le: Option<bool>) { self.store(v, at, (value as u64).to_le_bytes(), le); }
    fn __bake_c_view_get_f32(&self, v: View, at: f64, le: Option<bool>) -> f64 { f32::from_le_bytes(self.bits(v, at, le)) as f64 }
    fn __bake_c_view_set_f32(&self, v: View, at: f64, value: f64, le: Option<bool>) { self.store(v, at, (value as f32).to_le_bytes(), le); }
    fn __bake_c_view_get_f64(&self, v: View, at: f64, le: Option<bool>) -> f64 { f64::from_le_bytes(self.bits(v, at, le)) }
    fn __bake_c_view_set_f64(&self, v: View, at: f64, value: f64, le: Option<bool>) { self.store(v, at, (value).to_le_bytes(), le); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_and_views_are_independent() {
        let mut storage = [0xff; 128];
        let p = BoardMemory::new(&mut storage);
        let a = p.__bake_c_new_buffer(16.0);
        let b = p.__bake_c_new_buffer(16.0);
        assert_ne!(a.handle, b.handle);
        let av = p.__bake_c_new_view(a);
        let bv = p.__bake_c_new_view(b);
        p.__bake_c_view_set_u64(av, 0.0, 0xfedcba9876543210, Some(true));
        assert_eq!(p.__bake_c_view_get_u64(bv, 0.0, Some(true)), 0);
        assert_eq!(p.__bake_c_view_get_i64(av, 0.0, Some(true)), (0xfedcba9876543210u64 as i64) as i128);
        assert_eq!(p.__bake_c_view_get_u16(av, 0.0, None), 0x1032 as f64);
    }

    #[test]
    fn wrapping_atomic_and_overlap() {
        let mut storage = [0; 128];
        let p = BoardMemory::new(&mut storage);
        let buffer = p.__bake_c_new_buffer(16.0);
        let a = p.__bake_c_new_u64_array(buffer);
        p.__bake_c_atomic_store_u64(a, 0.0, u64::MAX as i128);
        assert_eq!(p.__bake_c_atomic_add_u64(a, 0.0, 1), u64::MAX as i128);
        assert_eq!(p.__bake_c_atomic_load_u64(a, 0.0), 0);
        let bytes = p.__bake_c_new_u8_array(buffer);
        for i in 0..8 { p.__bake_c_u8_set(bytes, i as f64, i as f64); }
        p.__bake_c_u8_copy_within(bytes, 2.0, 0.0, Some(8.0));
        assert_eq!(p.__bake_c_u8_get(bytes, 9.0), 7.0);
    }

    #[test]
    #[should_panic(expected = "array index")]
    fn rejects_out_of_range() {
        let mut storage = [0; 16];
        let p = BoardMemory::new(&mut storage);
        let a = p.__bake_c_new_u8_array(p.__bake_c_new_buffer(4.0));
        p.__bake_c_u8_set(a, 4.0, 1.0);
    }
}
