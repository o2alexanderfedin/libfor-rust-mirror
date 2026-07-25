use super::*;
use crate::r#for::{for_compress_sorted, for_compressed_size_sorted, for_uncompress};

#[allow(unused_doc_comments)]
extern "C" fn run(length: u32) -> () {
    let mut i: u32 = 0 as u32;
    let mut s1: u32 = 0 as u32;
    let mut s2: u32 = 0 as u32;
    let mut s3: u32 = 0 as u32;
    let out: *mut u8 =
        unsafe { malloc((length as u64).wrapping_mul(core::mem::size_of::<u32>() as u64)) }
            as *mut u8;
    let in_: *mut u32 =
        unsafe { malloc((length as u64).wrapping_mul(core::mem::size_of::<u32>() as u64)) }
            as *mut u32;
    let tmp: *mut u32 =
        unsafe { malloc((length as u64).wrapping_mul(core::mem::size_of::<u32>() as u64)) }
            as *mut u32;
    {
        i = 0 as u32;
        '__b0: loop {
            if !(i < length) {
                break '__b0;
            }
            '__c0: loop {
                unsafe { *in_.add(i as usize) = (33 as u32).wrapping_add(i) };
                break '__c0;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p = (*__p).wrapping_add(1);
                __t
            };
        }
    }
    s1 = for_compress_sorted(in_ as *const u32, out, length);
    s2 = for_uncompress(out as *const u8, tmp, length);
    s3 = for_compressed_size_sorted(in_ as *const u32, length);
    while (s1 == 0) as i32 as u32 == s2 {
        unsafe {
            printf(c"%s:%d: expression failed\n".as_ptr() as *mut i8 as
                    *const i8,
                c"/Users/alexanderfedin/.cache/clang2rust/crust-bench/dataset/CBench/libfor/benchmark.c".as_ptr()
                    as *mut i8, 52)
        };
        unsafe { exit(-1) };
    }
    while (s2 == 0) as i32 as u32 == s3 {
        unsafe {
            printf(c"%s:%d: expression failed\n".as_ptr() as *mut i8 as
                    *const i8,
                c"/Users/alexanderfedin/.cache/clang2rust/crust-bench/dataset/CBench/libfor/benchmark.c".as_ptr()
                    as *mut i8, 53)
        };
        unsafe { exit(-1) };
    }

    /// VERIFY_ARRAY(in, tmp, length);
    unsafe {
        free(in_ as *mut ())
    };
    unsafe { free(out as *mut ()) };
    unsafe { free(tmp as *mut ()) };
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn __main_inner() -> i32 {
    run((1024 * 1024 * 10) as u32);

    /// 10 mb
    return 0;
}
