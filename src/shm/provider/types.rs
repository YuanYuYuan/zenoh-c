//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

use std::mem::MaybeUninit;

use zenoh::shm::{AllocAlignment, BufAllocResult, ChunkAllocResult, MemoryLayout, ZAllocError};
use zenoh_util::core::zerror;

use crate::{
    errors::{z_error_t, Z_EINVAL, Z_OK},
    transmute::{Inplace, TransmuteRef, TransmuteUninitPtr},
    z_loaned_buf_alloc_result_t, z_loaned_chunk_alloc_result_t, z_loaned_memory_layout_t,
    z_owned_buf_alloc_result_t, z_owned_chunk_alloc_result_t, z_owned_memory_layout_t,
    z_owned_shm_mut_t,
};

use super::chunk::z_allocated_chunk_t;

/// Allocation errors
///
///     - **NEED_DEFRAGMENT**: defragmentation needed
///     - **OUT_OF_MEMORY**: the provider is out of memory
///     - **OTHER**: other error
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_alloc_error_t {
    NEED_DEFRAGMENT,
    OUT_OF_MEMORY,
    OTHER,
}

impl From<ZAllocError> for z_alloc_error_t {
    #[inline]
    fn from(value: ZAllocError) -> Self {
        match value {
            ZAllocError::NeedDefragment => z_alloc_error_t::NEED_DEFRAGMENT,
            ZAllocError::OutOfMemory => z_alloc_error_t::OUT_OF_MEMORY,
            ZAllocError::Other(_) => z_alloc_error_t::OTHER,
        }
    }
}

impl From<z_alloc_error_t> for ZAllocError {
    #[inline]
    fn from(value: z_alloc_error_t) -> Self {
        match value {
            z_alloc_error_t::NEED_DEFRAGMENT => ZAllocError::NeedDefragment,
            z_alloc_error_t::OUT_OF_MEMORY => ZAllocError::OutOfMemory,
            z_alloc_error_t::OTHER => ZAllocError::Other(zerror!("other error").into()),
        }
    }
}

// An AllocAlignment.
#[repr(C)]
pub struct z_alloc_alignment_t {
    pub pow: u8,
}

impl From<z_alloc_alignment_t> for AllocAlignment {
    fn from(value: z_alloc_alignment_t) -> Self {
        Self::new(value.pow)
    }
}

decl_transmute_owned!(Option<MemoryLayout>, z_owned_memory_layout_t);
decl_transmute_handle!(MemoryLayout, z_loaned_memory_layout_t);

/// Creates a new Memory Layout
#[no_mangle]
pub extern "C" fn z_memory_layout_new(
    this: *mut MaybeUninit<z_owned_memory_layout_t>,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    match MemoryLayout::new(size, AllocAlignment::new(alignment.pow)) {
        Ok(layout) => {
            Inplace::init(this.transmute_uninit_ptr(), Some(layout));
            Z_OK
        }
        Err(e) => {
            log::error!("{e}");
            Z_EINVAL
        }
    }
}

/// Constructs Memory Layout in its gravestone value.
#[no_mangle]
pub extern "C" fn z_memory_layout_null(this: *mut MaybeUninit<z_owned_memory_layout_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_memory_layout_check(this: &z_owned_memory_layout_t) -> bool {
    this.transmute_ref().is_some()
}

/// Deletes Memory Layout
#[no_mangle]
pub extern "C" fn z_memory_layout_delete(this: &mut z_owned_memory_layout_t) {
    let _ = this.transmute_mut().extract();
}

decl_transmute_owned!(Option<ChunkAllocResult>, z_owned_chunk_alloc_result_t);

decl_transmute_handle!(ChunkAllocResult, z_loaned_chunk_alloc_result_t);

/// Creates a new Chunk Alloc Result with Ok value
#[no_mangle]
pub extern "C" fn z_chunk_alloc_result_new_ok(
    this: *mut MaybeUninit<z_owned_chunk_alloc_result_t>,
    allocated_chunk: z_allocated_chunk_t,
) {
    Inplace::init(
        this.transmute_uninit_ptr(),
        Some(Ok(allocated_chunk.into())),
    );
}

/// Creates a new Chunk Alloc Result with Error value
#[no_mangle]
pub extern "C" fn z_chunk_alloc_result_new_error(
    this: *mut MaybeUninit<z_owned_chunk_alloc_result_t>,
    alloc_error: z_alloc_error_t,
) {
    Inplace::init(this.transmute_uninit_ptr(), Some(Err(alloc_error.into())));
}

/// Constructs Chunk Alloc Result in its gravestone value.
#[no_mangle]
pub extern "C" fn z_chunk_alloc_result_null(this: *mut MaybeUninit<z_owned_chunk_alloc_result_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_chunk_alloc_result_check(this: &z_owned_chunk_alloc_result_t) -> bool {
    this.transmute_ref().is_some()
}

/// Deletes Chunk Alloc Result
#[no_mangle]
pub extern "C" fn z_chunk_alloc_result_delete(this: &mut z_owned_chunk_alloc_result_t) {
    let _ = this.transmute_mut().extract();
}

decl_transmute_owned!(Option<BufAllocResult>, z_owned_buf_alloc_result_t);

decl_transmute_handle!(BufAllocResult, z_loaned_buf_alloc_result_t);

#[no_mangle]
pub extern "C" fn z_buf_alloc_result_unwrap(
    alloc_result: &mut z_owned_buf_alloc_result_t,
    out_buf: *mut MaybeUninit<z_owned_shm_mut_t>,
    out_error: &mut MaybeUninit<z_alloc_error_t>,
) -> z_error_t {
    match alloc_result.transmute_mut().extract() {
        Some(Ok(val)) => {
            Inplace::init(out_buf.transmute_uninit_ptr(), Some(val));
            Z_OK
        }
        Some(Err(err)) => {
            out_error.write(err.into());
            Z_OK
        }
        None => Z_EINVAL,
    }
}

/// Constructs Buf Alloc Result in its gravestone value.
#[no_mangle]
pub extern "C" fn z_buf_alloc_result_null(this: *mut MaybeUninit<z_owned_buf_alloc_result_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_buf_alloc_result_check(this: &z_owned_buf_alloc_result_t) -> bool {
    this.transmute_ref().is_some()
}

/// Deletes Buf Alloc Result
#[no_mangle]
pub extern "C" fn z_buf_alloc_result_delete(this: &mut z_owned_buf_alloc_result_t) {
    let _ = this.transmute_mut().extract();
}
