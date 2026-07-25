use super::*;
use crate::for_gen::{
    for_linsearch16, for_linsearch32, for_linsearch8, for_linsearchx, for_pack16, for_pack32,
    for_pack8, for_packx, for_unpack16, for_unpack32, for_unpack8, for_unpackx,
};

///Returns the size required to compress a sequence of |length| ints,
///each compressed with |bits| bits
///
///This function will NOT include any overhead required by
///for_compress_sorted() and for_compress_unsorted().
///
///Invariant: bits <= 32
#[allow(unused_doc_comments)]
pub(crate) extern "C" fn for_compressed_size_bits(mut length: u32, bits: u32) -> u32 {
    let mut c: u32 = 0 as u32;
    let mut b: u32 = 0 as u32;
    if !(bits <= 32 as u32) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"for_compressed_size_bits".as_ptr() as *const i8,
                c"for.c".as_ptr() as *mut i8 as *const i8,
                72,
                c"bits <= 32".as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if length >= 32 as u32 {
        b = length / 32 as u32;
        c = c.wrapping_add(
            b.wrapping_mul(32 as u32)
                .wrapping_mul(bits)
                .wrapping_add(7 as u32)
                / 8 as u32,
        );
        length %= 32 as u32;
    }
    if length >= 16 as u32 {
        b = length / 16 as u32;
        c = c.wrapping_add(
            b.wrapping_mul(16 as u32)
                .wrapping_mul(bits)
                .wrapping_add(7 as u32)
                / 8 as u32,
        );

        /// VERIFY_ARRAY(in, tmp, length);
        (length %= 16 as u32);
    }
    if length >= 8 as u32 {
        b = length / 8 as u32;
        c = c.wrapping_add(
            b.wrapping_mul(8 as u32)
                .wrapping_mul(bits)
                .wrapping_add(7 as u32)
                / 8 as u32,
        );
        length %= 8 as u32;
    }

    /// 10 mb
    return c.wrapping_add(length.wrapping_mul(bits).wrapping_add(7 as u32) / 8 as u32);
}

#[inline]
extern "C" fn required_bits(v: u32) -> u32 {
    return if v as u32 == 0 as u32 {
        0
    } else {
        32 - unsafe { __builtin_clz(v) }
    } as u32;
}

///Returns the size required to compress an unsorted sequence of |length| ints.
///
///This routine scans |in| for the min/max values and then calls
///for_compressed_size_bits().
///
///The returned size will include the overhead required for
///for_compress_sorted() and for_compressed_unsorted().
pub(crate) extern "C" fn for_compressed_size_unsorted(in_: *const u32, length: u32) -> u32 {
    let mut i: u32 = 0 as u32;
    let mut b: u32 = 0 as u32;
    let mut m: u32 = 0 as u32;
    let mut m: u32 = 0 as u32;
    if length == 0 as u32 {
        return 0 as u32;
    }
    m = unsafe { *in_.offset(0 as isize) } as u32;
    m = m;
    {
        i = 1 as u32;
        '__b3: loop {
            if !(i < length) {
                break '__b3;
            }
            '__c3: loop {
                if (unsafe { *in_.add(i as usize) } as u32) < m {
                    m = unsafe { *in_.add(i as usize) } as u32;
                }
                if unsafe { *in_.add(i as usize) } as u32 > m {
                    m = unsafe { *in_.add(i as usize) } as u32;
                }
                break '__c3;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p = (*__p).wrapping_add(1);
                __t
            };
        }
    }
    b = required_bits(m.wrapping_sub(m) as u32);
    return 5 as u32 + for_compressed_size_bits(length, b);
}

///Returns the size required to compress a sorted sequence of |length| ints.
///
///This routine extracts min/max values at the beginning and end of
///the sequence, then calls for_compressed_size_bits(). It is therefore
///slightly faster than for_compressed_size_unsorted().
///
///The returned size will include the overhead required for
///for_compress_sorted() and for_compressed_unsorted().
pub(crate) extern "C" fn for_compressed_size_sorted(in_: *const u32, length: u32) -> u32 {
    let mut b: u32 = 0 as u32;
    let mut m: u32 = 0 as u32;
    let mut m: u32 = 0 as u32;
    if length == 0 as u32 {
        return 0 as u32;
    }
    m = unsafe { *in_.offset(0 as isize) } as u32;
    m = unsafe { *in_.add(length.wrapping_sub(1 as u32) as usize) } as u32;
    b = required_bits(m.wrapping_sub(m) as u32);
    return 5 as u32 + for_compressed_size_bits(length, b);
}

pub(crate) type ForPackfuncT = unsafe extern "C" fn(u32, *const u32, *mut u8) -> u32;

pub(crate) type ForPackxfuncT = unsafe extern "C" fn(u32, *const u32, *mut u8, u32) -> u32;

///Compresses a sequence of |length| ints at |in| and stores the result
///in |out|.
///
///|base| is the "offset" (or common delta value) of all ints. It is usually
///set to the minimum value of the uncompressed sequence.
///
///|bits| are the bits required to store a single integer.
///
///Returns the number of bytes used for compression.
///
///This is for advanced users who opt for storing |base| and |bits| on their
///own. This function is called by for_compress_sorted() and
///for_compress_unsorted().
///
///Invariant: bits <= 32
pub(crate) extern "C" fn for_compress_bits(
    mut in_: *const u32,
    out: *mut u8,
    length: u32,
    base: u32,
    bits: u32,
) -> u32 {
    unsafe {
        let mut i: u32 = 0 as u32;
        let mut written: u32 = 0 as u32;
        if !(bits <= 32 as u32) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(
                    c"for_compress_bits".as_ptr() as *const i8,
                    c"for.c".as_ptr() as *mut i8 as *const i8,
                    146,
                    c"bits <= 32".as_ptr() as *mut i8 as *const i8,
                )
            }
        } else {
            {
                let _ = 0;
            }
        };
        {
            '__b39: loop {
                if !(i.wrapping_add(32 as u32) <= length) {
                    break '__b39;
                }
                '__c39: loop {
                    written = written.wrapping_add(unsafe {
                        for_pack32[bits as usize](base, in_, unsafe { out.add(written as usize) })
                    });
                    break '__c39;
                }
                {
                    i = i.wrapping_add(32 as u32);
                    {
                        let __n = 32;
                        let __p = &mut in_;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    }
                };
            }
        }
        {
            '__b40: loop {
                if !(i.wrapping_add(16 as u32) <= length) {
                    break '__b40;
                }
                '__c40: loop {
                    written = written.wrapping_add(unsafe {
                        for_pack16[bits as usize](base, in_, unsafe { out.add(written as usize) })
                    });
                    break '__c40;
                }
                {
                    i = i.wrapping_add(16 as u32);
                    {
                        let __n = 16;
                        let __p = &mut in_;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    }
                };
            }
        }
        {
            '__b41: loop {
                if !(i.wrapping_add(8 as u32) <= length) {
                    break '__b41;
                }
                '__c41: loop {
                    written = written.wrapping_add(unsafe {
                        for_pack8[bits as usize](base, in_, unsafe { out.add(written as usize) })
                    });
                    break '__c41;
                }
                {
                    i = i.wrapping_add(8 as u32);
                    {
                        let __n = 8;
                        let __p = &mut in_;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    }
                };
            }
        }
        return written.wrapping_add(unsafe {
            for_packx[bits as usize](
                base,
                in_,
                unsafe { out.add(written as usize) },
                length.wrapping_sub(i),
            )
        });
    }
}

///Compresses an unsorted sequence of |length| ints at |in| and stores the
///result in |out|.
///
///This routine scans |in| for the min/max values and then calls
///for_compress_bits().
///
///The minimun value and the bits are stored as metadata in |out|.
pub(crate) extern "C" fn for_compress_unsorted(in_: *const u32, out: *mut u8, length: u32) -> u32 {
    let mut i: u32 = 0 as u32;
    let mut b: u32 = 0 as u32;
    let mut m: u32 = 0 as u32;
    let mut m: u32 = 0 as u32;
    if length == 0 as u32 {
        return 0 as u32;
    }
    m = unsafe { *in_.offset(0 as isize) } as u32;
    m = m;
    {
        i = 1 as u32;
        '__b42: loop {
            if !(i < length) {
                break '__b42;
            }
            '__c42: loop {
                if (unsafe { *in_.add(i as usize) } as u32) < m {
                    m = unsafe { *in_.add(i as usize) } as u32;
                }
                if unsafe { *in_.add(i as usize) } as u32 > m {
                    m = unsafe { *in_.add(i as usize) } as u32;
                }
                break '__c42;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p = (*__p).wrapping_add(1);
                __t
            };
        }
    }
    b = required_bits(m.wrapping_sub(m) as u32);
    unsafe { *(unsafe { out.offset(0 as isize) } as *mut u32) = m };
    unsafe { *(unsafe { out.offset(4 as isize) } as *mut u8) = b as u8 };
    return 5 as u32 + for_compress_bits(in_, unsafe { out.offset(5 as isize) }, length, m, b);
}

///Compresses a sorted sequence of |length| ints at |in| and stores the
///result in |out|.
///
///This routine extracts min/max values at the beginning and end of
///the sequence, then calls for_compress_bits().
///
///The minimun value and the bits are stored as metadata in |out|.
pub(crate) extern "C" fn for_compress_sorted(in_: *const u32, out: *mut u8, length: u32) -> u32 {
    let mut m: u32 = 0 as u32;
    let mut m: u32 = 0 as u32;
    let mut b: u32 = 0 as u32;
    if length == 0 as u32 {
        return 0 as u32;
    }
    m = unsafe { *in_.offset(0 as isize) } as u32;
    m = unsafe { *in_.add(length.wrapping_sub(1 as u32) as usize) } as u32;
    b = required_bits(m.wrapping_sub(m) as u32);
    unsafe { *(unsafe { out.offset(0 as isize) } as *mut u32) = m };
    unsafe { *(unsafe { out.offset(4 as isize) } as *mut u8) = b as u8 };
    return 5 as u32 + for_compress_bits(in_, unsafe { out.offset(5 as isize) }, length, m, b);
}

pub(crate) type ForUnpackfuncT = unsafe extern "C" fn(u32, *const u8, *mut u32) -> u32;

pub(crate) type ForUnpackxfuncT = unsafe extern "C" fn(u32, *const u8, *mut u32, u32) -> u32;

///Uncompresses a sequence of |length| ints at |in| and stores the
///result in |out|.
///
///|base| is the "offset" (or common delta value) of all ints. It is usually
///set to the minimum value of the uncompressed sequence.
///
///|bits| are the bits required to store a single integer.
///
///Returns the number of compressed bytes processed.
///
///This function is for advanced users. It is the counterpart of
///for_compress_bits().
///
///Invariant: bits <= 32
pub(crate) extern "C" fn for_uncompress_bits(
    mut in_: *const u8,
    mut out: *mut u32,
    length: u32,
    base: u32,
    bits: u32,
) -> u32 {
    unsafe {
        let mut i: u32 = 0 as u32;
        let bin: *const u8 = in_;
        if !(bits <= 32 as u32) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(
                    c"for_uncompress_bits".as_ptr() as *const i8,
                    c"for.c".as_ptr() as *mut i8 as *const i8,
                    217,
                    c"bits <= 32".as_ptr() as *mut i8 as *const i8,
                )
            }
        } else {
            {
                let _ = 0;
            }
        };
        {
            '__b82: loop {
                if !(i.wrapping_add(32 as u32) <= length) {
                    break '__b82;
                }
                '__c82: loop {
                    {
                        let __n = unsafe { for_unpack32[bits as usize](base, in_, out) };
                        let __p = &mut in_;
                        *__p = unsafe { (*__p).add(__n as usize) };
                    };
                    break '__c82;
                }
                {
                    i = i.wrapping_add(32 as u32);
                    {
                        let __n = 32;
                        let __p = &mut out;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    }
                };
            }
        }
        {
            '__b83: loop {
                if !(i.wrapping_add(16 as u32) <= length) {
                    break '__b83;
                }
                '__c83: loop {
                    {
                        let __n = unsafe { for_unpack16[bits as usize](base, in_, out) };
                        let __p = &mut in_;
                        *__p = unsafe { (*__p).add(__n as usize) };
                    };
                    break '__c83;
                }
                {
                    i = i.wrapping_add(16 as u32);
                    {
                        let __n = 16;
                        let __p = &mut out;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    }
                };
            }
        }
        {
            '__b84: loop {
                if !(i.wrapping_add(8 as u32) <= length) {
                    break '__b84;
                }
                '__c84: loop {
                    {
                        let __n = unsafe { for_unpack8[bits as usize](base, in_, out) };
                        let __p = &mut in_;
                        *__p = unsafe { (*__p).add(__n as usize) };
                    };
                    break '__c84;
                }
                {
                    i = i.wrapping_add(8 as u32);
                    {
                        let __n = 8;
                        let __p = &mut out;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    }
                };
            }
        }
        return (unsafe { in_.offset_from(bin) } as i64
            + unsafe { for_unpackx[bits as usize](base, in_, out, length.wrapping_sub(i)) } as i64)
            as u32;
    }
}

///Uncompresses a sequence of |length| ints at |in| and stores the
///result in |out|.
///
///This function is a convenience wrapper for for_uncompress_bits(). It
///expects metadata at the beginning of |in|. Use in combination with
///for_compress_sorted() and for_compress_unsorted().
///
///Returns the number of compressed bytes processed.
#[allow(unused_doc_comments)]
pub(crate) extern "C" fn for_uncompress(in_: *const u8, out: *mut u32, length: u32) -> u32 {
    let mut m: u32 = 0 as u32;
    let mut b: u32 = 0 as u32;
    if length == 0 as u32 {
        return 0 as u32;
    }

    ///Compresses an unsorted sequence of |length| ints at |in| and stores the
    ///result in |out|.
    ///
    ///This routine scans |in| for the min/max values and then calls
    ///for_compress_bits().
    ///
    ///The minimun value and the bits are stored as metadata in |out|.
    (m = unsafe { *(unsafe { in_.offset(0 as isize) } as *mut u32) });
    b = unsafe { *unsafe { in_.offset(4 as isize) } } as u32;
    return 5 as u32 + for_uncompress_bits(unsafe { in_.offset(5 as isize) }, out, length, m, b);
}

pub(crate) type AppendImpl = unsafe extern "C" fn(*const u32, *mut u8, u32) -> u32;

///Appends a |value| to a compressed integer sequence.
///
///|base| is the "offset" (or common delta value) of all ints. It is usually
///set to the minimum value of the uncompressed sequence.
///
///|bits| are the bits required to store a single integer.
///
///Returns the size (in bytes) of the compressed data.
///
///Invariant: bits <= 32
///Invariant: the new |value| (more precisely: |value - base|) can be stored
///     in |bits| bits. Details can be found in the implementation of
///     for_append() in for.c.
#[allow(unused_doc_comments)]
pub(crate) extern "C" fn for_append_bits(
    mut in_: *mut u8,
    mut length: u32,
    base: u32,
    bits: u32,
    mut value: u32,
) -> u32 {
    let mut b: u32 = 0 as u32;
    let mut start: u32 = 0 as u32;
    let initin: *const u8 = in_ as *const u8;
    let mut in32: *mut u32 = in_ as *mut u32;
    if !(bits <= 32 as u32) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"for_append_bits".as_ptr() as *const i8,
                c"for.c".as_ptr() as *mut i8 as *const i8,
                254,
                c"bits <= 32".as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(required_bits((value - base) as u32) <= bits) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"for_append_bits".as_ptr() as *const i8,
                c"for.c".as_ptr() as *mut i8 as *const i8,
                255,
                c"required_bits(value - base) <= bits".as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(value >= base) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"for_append_bits".as_ptr() as *const i8,
                c"for.c".as_ptr() as *mut i8 as *const i8,
                256,
                c"value >= base".as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if bits == 32 as u32 {
        unsafe { *in32.add(length as usize) = value.wrapping_sub(base) };
        return (length.wrapping_add(1 as u32) as u64)
            .wrapping_mul(core::mem::size_of::<u32>() as u64) as u32;
    }
    if length > 32 as u32 {
        b = length / 32 as u32;
        {
            let __n = b.wrapping_mul(32 as u32).wrapping_mul(bits) / 8 as u32;
            let __p = &mut in_;
            *__p = unsafe { (*__p).add(__n as usize) };
        };
        length %= 32 as u32;
    }
    if length > 16 as u32 {
        b = length / 16 as u32;
        {
            let __n = b.wrapping_mul(16 as u32).wrapping_mul(bits) / 8 as u32;
            let __p = &mut in_;
            *__p = unsafe { (*__p).add(__n as usize) };
        };
        length %= 16 as u32;
    }
    if length > 8 as u32 {
        b = length / 8 as u32;
        {
            let __n = b.wrapping_mul(8 as u32).wrapping_mul(bits) / 8 as u32;
            let __p = &mut in_;
            *__p = unsafe { (*__p).add(__n as usize) };
        };
        length %= 8 as u32;
    }
    start = length.wrapping_mul(bits);
    {
        let __n = start / 8 as u32;
        let __p = &mut in_;
        *__p = unsafe { (*__p).add(__n as usize) };
    };
    start %= 8 as u32;
    in32 = in_ as *mut u32;
    value = value.wrapping_sub(base);
    if start.wrapping_add(bits) < 32 as u32 {
        let mask: u32 = ((1 << bits) - 1) as u32;
        unsafe { *in32 &= !(mask << start) };
        unsafe { *in32 |= value << start };
    } else {
        ///Uncompresses a sequence of |length| ints at |in| and stores the
        ///result in |out|.
        ///
        ///This function is a convenience wrapper for for_uncompress_bits(). It
        ///expects metadata at the beginning of |in|. Use in combination with
        ///for_compress_sorted() and for_compress_unsorted().
        ///
        ///Returns the number of compressed bytes processed.
        let mask1: u32 = ((1 << bits) - 1) as u32;
        let mask2: u32 = ((1 << bits.wrapping_sub((32 as u32).wrapping_sub(start))) - 1) as u32;
        unsafe { *unsafe { in32.offset(0 as isize) } &= !(mask1 << start) };
        unsafe { *unsafe { in32.offset(0 as isize) } |= (value & mask1) << start };
        unsafe { *unsafe { in32.offset(1 as isize) } &= !mask2 };
        unsafe { *unsafe { in32.offset(1 as isize) } |= value >> (32 as u32).wrapping_sub(start) };
    }
    return (unsafe { in_.offset_from(initin) } as i64
        + (start.wrapping_add(bits).wrapping_add(7 as u32) / 8 as u32) as i64) as u32;
}

extern "C" fn for_append_impl(
    in__1: *mut u8,
    length: u32,
    mut value: u32,
    impl__1: Option<unsafe extern "C" fn(*const u32, *mut u8, u32) -> u32>,
) -> u32 {
    let mut m: u32 = 0 as u32;
    let mut b: u32 = 0 as u32;
    let mut bnew: u32 = 0 as u32;
    let mut s: u32 = 0 as u32;
    if length == 0 as u32 {
        return unsafe { impl__1.unwrap()(&raw mut value as *const u32, in__1, 1) };
    }
    m = unsafe { *(unsafe { in__1.offset(0 as isize) } as *mut u32) };
    b = unsafe { *unsafe { in__1.offset(4 as isize) } } as u32;
    bnew = required_bits(value.wrapping_sub(m) as u32);
    if m > value || bnew > b {
        let tmp: *mut u32 = unsafe {
            malloc(
                (core::mem::size_of::<u32>() as u64)
                    .wrapping_mul(length.wrapping_add(1 as u32) as u64),
            )
        } as *mut u32;
        if (tmp).is_null() as i32 != 0 {
            return 0 as u32;
        }
        for_uncompress(in__1 as *const u8, tmp, length);
        unsafe { *tmp.add(length as usize) = value };
        s = unsafe { impl__1.unwrap()(tmp as *const u32, in__1, length.wrapping_add(1 as u32)) };
        unsafe { free(tmp as *mut ()) };
        return s;
    }
    return 5 as u32 + for_append_bits(unsafe { in__1.offset(5 as isize) }, length, m, b, value);
}

///Appends a |value| to a compressed sequence of unsorted integers.
///
///This function is optimized for appending new values at the end of an
///encoded sequence. This is only possible if the new value (more precisely:
///the delta of the new value) can be stored in the same amount of bits that
///were used to encode the other integers.
///
///If this is not the case then memory is allocated, the whole sequence is
///decoded and re-encoded using more bits. This requires a heap allocation
///with malloc().
///
///Returns the size (in bytes) of the compressed data, or 0 if malloc() fails.
pub(crate) extern "C" fn for_append_unsorted(in_: *mut u8, length: u32, value: u32) -> u32 {
    return for_append_impl(in_, length, value, Some(for_compress_unsorted));
}

///Appends a |value| to a compressed sequence of sorted integers.
///
///This function is optimized for appending new values at the end of an
///encoded sequence. This is only possible if the new value (more precisely:
///the delta of the new value) can be stored in the same amount of bits that
///were used to encode the other integers.
///
///If this is not the case then memory is allocated, the whole sequence is
///decoded and re-encoded using more bits. This requires a heap allocation
///with malloc().
///
///Returns the size (in bytes) of the compressed data, or 0 if malloc() fails.
pub(crate) extern "C" fn for_append_sorted(in_: *mut u8, length: u32, value: u32) -> u32 {
    return for_append_impl(in_, length, value, Some(for_compress_sorted));
}

///Returns the value at the given |index| from a compressed sequence.
///
///Make sure that |index| does not exceed the length of the sequence.
///
///|base| is the "offset" (or common delta value) of all ints. It is usually
///set to the minimum value of the uncompressed sequence.
///
///Invariant: bits <= 32
pub(crate) extern "C" fn for_select_bits(
    mut in_: *const u8,
    base: u32,
    bits: u32,
    mut index: u32,
) -> u32 {
    let mut b: u32 = 0 as u32;
    let mut start: u32 = 0 as u32;
    let mut in32: *const u32 = core::ptr::null();
    if !(bits <= 32 as u32) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"for_select_bits".as_ptr() as *const i8,
                c"for.c".as_ptr() as *mut i8 as *const i8,
                363,
                c"bits <= 32".as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if bits == 32 as u32 {
        in32 = in_ as *mut u32 as *const u32;
        return base.wrapping_add(unsafe { *in32.add(index as usize) } as u32);
    }
    if index > 32 as u32 {
        b = index / 32 as u32;
        {
            let __n = b.wrapping_mul(32 as u32).wrapping_mul(bits) / 8 as u32;
            let __p = &mut in_;
            *__p = unsafe { (*__p).add(__n as usize) };
        };
        index %= 32 as u32;
    }
    if index > 16 as u32 {
        b = index / 16 as u32;
        {
            let __n = b.wrapping_mul(16 as u32).wrapping_mul(bits) / 8 as u32;
            let __p = &mut in_;
            *__p = unsafe { (*__p).add(__n as usize) };
        };
        index %= 16 as u32;
    }
    if index > 8 as u32 {
        b = index / 8 as u32;
        {
            let __n = b.wrapping_mul(8 as u32).wrapping_mul(bits) / 8 as u32;
            let __p = &mut in_;
            *__p = unsafe { (*__p).add(__n as usize) };
        };
        index %= 8 as u32;
    }
    start = index.wrapping_mul(bits);
    {
        let __n = start / 8 as u32;
        let __p = &mut in_;
        *__p = unsafe { (*__p).add(__n as usize) };
    };
    start %= 8 as u32;
    in32 = in_ as *mut u32 as *const u32;
    if start.wrapping_add(bits) < 32 as u32 {
        let mask: u32 = ((1 << bits) - 1) as u32;
        return base.wrapping_add(unsafe { *in32 } >> start & mask);
    } else {
        let mask1: u32 = ((1 << bits) - 1) as u32;
        let mask2: u32 = ((1 << bits.wrapping_sub((32 as u32).wrapping_sub(start))) - 1) as u32;
        let v1: u32 = unsafe { *unsafe { in32.offset(0 as isize) } } >> start & mask1;
        let v2: u32 = unsafe { *unsafe { in32.offset(1 as isize) } } & mask2;
        return base.wrapping_add(v2 << (32 as u32).wrapping_sub(start) | v1);
    }
}

///Returns the value at the given |index| from a compressed sequence.
///
///Make sure that |index| does not exceed the length of the sequence.
///
///This function is a convenience wrapper for for_select_bits(). It
///expects metadata at the beginning of |in|. Use in combination with
///for_compress_sorted() and for_compress_unsorted().
pub(crate) extern "C" fn for_select(in_: *const u8, index: u32) -> u32 {
    let m: u32 = unsafe { *(unsafe { in_.offset(0 as isize) } as *mut u32) };
    let b: u32 = unsafe { *unsafe { in_.offset(4 as isize) } } as u32;
    return for_select_bits(unsafe { in_.offset(5 as isize) }, m, b, index);
}

pub(crate) type ForLinsearchfuncT = unsafe extern "C" fn(u32, *const u8, u32, *mut i32) -> u32;

pub(crate) type ForLinsearchxfuncT =
    unsafe extern "C" fn(u32, *const u8, u32, u32, *mut i32) -> u32;

///Performs a linear search for |value|.
///
///Returns the index of the found element, or |length| if the key was not
///found.
///
///|base| is the "offset" (or common delta value) of all ints. It is usually
///set to the minimum value of the uncompressed sequence.
///
///Invariant: bits <= 32
pub(crate) extern "C" fn for_linear_search_bits(
    mut in_: *const u8,
    length: u32,
    base: u32,
    bits: u32,
    value: u32,
) -> u32 {
    unsafe {
        let mut i: u32 = 0 as u32;
        let mut found: i32 = -1;
        if !(bits <= 32 as u32) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(
                    c"for_linear_search_bits".as_ptr() as *const i8,
                    c"for.c".as_ptr() as *mut i8 as *const i8,
                    440,
                    c"bits <= 32".as_ptr() as *mut i8 as *const i8,
                )
            }
        } else {
            {
                let _ = 0;
            }
        };
        if bits == 0 as u32 {
            return if value == base { 0 as u32 } else { length };
        }
        {
            '__b120: loop {
                if !(i.wrapping_add(32 as u32) <= length) {
                    break '__b120;
                }
                '__c120: loop {
                    {
                        let __n =
                            unsafe { for_linsearch32[bits as usize](base, in_, value, &mut found) };
                        let __p = &mut in_;
                        *__p = unsafe { (*__p).add(__n as usize) };
                    };
                    if found >= 0 {
                        return i.wrapping_add(found as u32);
                    }
                    break '__c120;
                }
                i = i.wrapping_add(32 as u32);
            }
        }
        {
            '__b121: loop {
                if !(i.wrapping_add(16 as u32) <= length) {
                    break '__b121;
                }
                '__c121: loop {
                    {
                        let __n =
                            unsafe { for_linsearch16[bits as usize](base, in_, value, &mut found) };
                        let __p = &mut in_;
                        *__p = unsafe { (*__p).add(__n as usize) };
                    };
                    if found >= 0 {
                        return i.wrapping_add(found as u32);
                    }
                    break '__c121;
                }
                i = i.wrapping_add(16 as u32);
            }
        }
        {
            '__b122: loop {
                if !(i.wrapping_add(8 as u32) <= length) {
                    break '__b122;
                }
                '__c122: loop {
                    {
                        let __n =
                            unsafe { for_linsearch8[bits as usize](base, in_, value, &mut found) };
                        let __p = &mut in_;
                        *__p = unsafe { (*__p).add(__n as usize) };
                    };
                    if found >= 0 {
                        return i.wrapping_add(found as u32);
                    }
                    break '__c122;
                }
                i = i.wrapping_add(8 as u32);
            }
        }
        unsafe {
            for_linsearchx[bits as usize](base, in_, length.wrapping_sub(i), value, &mut found)
        };
        if found >= 0 {
            return i.wrapping_add(found as u32);
        }
        return length;
    }
}

///Performs a linear search for |value|.
///
///Returns the index of the found element, or |length| if the key was not
///found.
///
///This function is a convenience wrapper for for_linear_search_bits(). It
///expects metadata at the beginning of |in|. Use in combination with
///for_compress_sorted() and for_compress_unsorted().
pub(crate) extern "C" fn for_linear_search(in_: *const u8, length: u32, value: u32) -> u32 {
    let m: u32 = unsafe { *(unsafe { in_.offset(0 as isize) } as *mut u32) };
    let b: u32 = unsafe { *unsafe { in_.offset(4 as isize) } } as u32;
    return for_linear_search_bits(unsafe { in_.offset(5 as isize) }, length, m, b, value);
}

///Performs lower bound binary search search for |value|.
///
///A lower bound search returns the first element in the sequence which does
///not compare less than |value|.
///The actual result is stored in |*actual|.
///
///|base| is the "offset" (or common delta value) of all ints. It is usually
///set to the minimum value of the uncompressed sequence.
///
///Invariant: bits <= 32
#[allow(unused_doc_comments)]
pub(crate) extern "C" fn for_lower_bound_search_bits(
    in_: *const u8,
    length: u32,
    base: u32,
    bits: u32,
    value: u32,
    actual: &mut u32,
) -> u32 {
    let mut imid: u32 = 0 as u32;
    let mut imin: u32 = 0 as u32;
    let mut imax: u32 = length.wrapping_sub(1 as u32);
    let mut v: u32 = 0 as u32;
    while imin.wrapping_add(1 as u32) < imax {
        imid = imin.wrapping_add(imax.wrapping_sub(imin) / 2 as u32);
        v = for_select_bits(in_, base, bits, imid);
        if v >= value {
            imax = imid;
        } else if v < value {
            ///Performs lower bound binary search search for |value|.
            ///
            ///A lower bound search returns the first element in the sequence which does
            ///not compare less than |value|.
            ///The actual result is stored in |*actual|.
            ///
            ///|base| is the "offset" (or common delta value) of all ints. It is usually
            ///set to the minimum value of the uncompressed sequence.
            ///
            ///Invariant: bits <= 32
            (imin = imid);
        }
    }
    v = for_select_bits(in_, base, bits, imin);
    if v >= value {
        *actual = v;
        return imin;
    }
    v = for_select_bits(in_, base, bits, imax);
    *actual = v;
    return imax;
}

///Performs lower bound binary search search for |value|.
///
///A lower bound search returns the first element in the sequence which does
///not compare less than |value|.
///The actual result is stored in |*actual|.
///
///This function is a convenience wrapper for for_lower_bound_search_bits(). It
///expects metadata at the beginning of |in|. Use in combination with
///for_compress_sorted() and for_compress_unsorted().
#[allow(unused_doc_comments)]
pub(crate) extern "C" fn for_lower_bound_search(
    in_: *const u8,
    length: u32,
    value: u32,
    actual: *mut u32,
) -> u32 {
    let m: u32 = unsafe { *(unsafe { in_.offset(0 as isize) } as *mut u32) };
    let b: u32 = unsafe { *unsafe { in_.offset(4 as isize) } } as u32;

    ///Performs lower bound binary search search for |value|.
    ///
    ///A lower bound search returns the first element in the sequence which does
    ///not compare less than |value|.
    ///The actual result is stored in |*actual|.
    ///
    ///This function is a convenience wrapper for for_lower_bound_search_bits(). It
    ///expects metadata at the beginning of |in|. Use in combination with
    ///for_compress_sorted() and for_compress_unsorted().
    return for_lower_bound_search_bits(
        unsafe { in_.offset(5 as isize) },
        length,
        m,
        b,
        value,
        unsafe { &mut *actual },
    );
}
