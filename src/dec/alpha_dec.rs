// Copyright 2011 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Alpha-plane decompression.
//
// Author: Skal (pascal.massimino@gmail.com)
// Port to Rust: Masaki Hara (ackie.h.gmai@gmail.com)

use std::mem;
use std::os::raw::*;
use std::ptr;
use std::slice;

use dec::io_dec::WebPInitCustomIo;
use dec::vp8_dec::{VP8InitIo, VP8Io};
use dec::vp8i_dec::VP8Decoder;
use dec::vp8l_dec::{VP8LDecodeAlphaHeader, VP8LDecodeAlphaImageStream};
use dec::vp8li_dec::{VP8LDecoder, VP8LDelete};
use dsp::dsp::{WEBP_FILTER_LAST, WEBP_FILTER_NONE, WEBP_FILTER_TYPE};
use dsp::filters::{VP8FiltersInit, WebPUnfilters};
use format_constants::{
    ALPHA_HEADER_LEN, ALPHA_LOSSLESS_COMPRESSION, ALPHA_NO_COMPRESSION, ALPHA_PREPROCESSED_LEVELS,
};
use utils::quant_levels_dec_utils::WebPDequantizeLevels;
use utils::utils::{WebPSafeCalloc, WebPSafeFree, WebPSafeMalloc};

#[repr(C)]
pub(crate) struct ALPHDecoder {
    pub(crate) width_: c_int,
    pub(crate) height_: c_int,
    pub(crate) method_: c_int,
    pub(crate) filter_: WEBP_FILTER_TYPE,
    pub(crate) pre_processing_: c_int,
    pub(crate) vp8l_dec_: *mut VP8LDecoder,
    pub(crate) io_: VP8Io,
    pub(crate) use_8b_decode_: c_int, // Although alpha channel requires only 1 byte per
    // pixel, sometimes VP8LDecoder may need to allocate
    // 4 bytes per pixel internally during decode.
    pub(crate) output_: *mut u8,
    pub(crate) prev_line_: *const u8, // last output row (or NULL)
}

//------------------------------------------------------------------------------
// ALPHDecoder object.

// Allocates a new alpha decoder instance.
#[allow(non_snake_case)]
fn ALPHNew() -> *mut ALPHDecoder {
    let dec: *mut ALPHDecoder =
        (unsafe { WebPSafeCalloc(1, mem::size_of::<ALPHDecoder>()) }) as *mut ALPHDecoder;
    return dec;
}

// Clears and deallocates an alpha decoder instance.
#[allow(non_snake_case)]
unsafe fn ALPHDelete(dec: *mut ALPHDecoder) {
    if !dec.is_null() {
        VP8LDelete((*dec).vp8l_dec_);
        (*dec).vp8l_dec_ = ptr::null_mut();
        WebPSafeFree(dec as *mut c_void);
    }
}

//------------------------------------------------------------------------------
// Decoding.

// Initialize alpha decoding by parsing the alpha header and decoding the image
// header for alpha data stored using lossless compression.
// Returns false in case of error in alpha header (data too short, invalid
// compression method or filter, error in lossless header data etc).
#[allow(non_snake_case)]
unsafe fn ALPHInit(
    dec: *mut ALPHDecoder,
    data: *const u8,
    data_size: usize,
    src_io: *const VP8Io,
    output: *mut u8,
) -> c_int {
    let copy_of_dec = dec;
    let dec: &mut ALPHDecoder = &mut *dec;

    let alpha_data: *const u8 = data.offset(ALPHA_HEADER_LEN as isize);
    let alpha_data_size: usize = data_size - ALPHA_HEADER_LEN;

    c_assert!(!data.is_null() && !output.is_null() && !src_io.is_null());
    let src_io: &VP8Io = &*src_io;
    let data: &[u8] = slice::from_raw_parts(data, data_size);

    VP8FiltersInit();
    dec.output_ = output;
    dec.width_ = src_io.width;
    dec.height_ = src_io.height;
    c_assert!(dec.width_ > 0 && dec.height_ > 0);

    if data_size <= ALPHA_HEADER_LEN {
        return 0;
    }

    dec.method_ = ((data[0] >> 0) & 0x03) as i32;
    dec.filter_ = WEBP_FILTER_TYPE::new(((data[0] >> 2) & 0x03) as i32);
    dec.pre_processing_ = ((data[0] >> 4) & 0x03) as i32;
    let rsrv: i32 = ((data[0] >> 6) & 0x03) as i32;
    if dec.method_ < ALPHA_NO_COMPRESSION
        || dec.method_ > ALPHA_LOSSLESS_COMPRESSION
        || dec.filter_ as isize >= WEBP_FILTER_LAST as isize
        || dec.pre_processing_ > ALPHA_PREPROCESSED_LEVELS
        || rsrv != 0
    {
        return 0;
    }

    // Copy the necessary parameters from src_io to io
    {
        let io: &mut VP8Io = &mut dec.io_;
        VP8InitIo(io);
        WebPInitCustomIo(ptr::null_mut(), io);
        io.opaque = copy_of_dec as *mut c_void;
        io.width = src_io.width;
        io.height = src_io.height;

        io.use_cropping = src_io.use_cropping;
        io.crop_left = src_io.crop_left;
        io.crop_right = src_io.crop_right;
        io.crop_top = src_io.crop_top;
        io.crop_bottom = src_io.crop_bottom;
        // No need to copy the scaling parameters.
    }

    let ok: c_int;
    if dec.method_ == ALPHA_NO_COMPRESSION {
        let alpha_decoded_size = dec.width_ as usize * dec.height_ as usize;
        ok = (alpha_data_size >= alpha_decoded_size) as c_int;
    } else {
        c_assert!(dec.method_ == ALPHA_LOSSLESS_COMPRESSION);
        ok = VP8LDecodeAlphaHeader(dec, alpha_data, alpha_data_size);
    }

    return ok;
}

// Decodes, unfilters and dequantizes *at least* 'num_rows' rows of alpha
// starting from row number 'row'. It assumes that rows up to (row - 1) have
// already been decoded.
// Returns false in case of bitstream error.
#[allow(non_snake_case)]
unsafe fn ALPHDecode(dec: *mut VP8Decoder, row: c_int, num_rows: c_int) -> c_int {
    let dec: &mut VP8Decoder = &mut *dec;

    let alph_dec: &mut ALPHDecoder = &mut *dec.alph_dec_;
    let width: c_int = alph_dec.width_;
    let height: c_int = alph_dec.io_.crop_bottom;
    if alph_dec.method_ == ALPHA_NO_COMPRESSION {
        let mut prev_line: *const u8 = dec.alpha_prev_line_;
        let mut deltas: *const u8 = dec.alpha_data_
            .offset(ALPHA_HEADER_LEN as isize + row as isize * width as isize);
        let mut dst: *mut u8 = dec.alpha_plane_.offset(row as isize * width as isize);
        c_assert!(deltas <= dec.alpha_data_.offset(dec.alpha_data_size_ as isize));
        if alph_dec.filter_ != WEBP_FILTER_NONE {
            c_assert!(WebPUnfilters[alph_dec.filter_ as usize].is_some());
            for _ in 0..num_rows {
                WebPUnfilters[alph_dec.filter_ as usize].unwrap()(prev_line, deltas, dst, width);
                prev_line = dst;
                dst = dst.offset(width as isize);
                deltas = deltas.offset(width as isize);
            }
        } else {
            for _ in 0..num_rows {
                ptr::copy_nonoverlapping(deltas, dst, width as usize);
                prev_line = dst;
                dst = dst.offset(width as isize);
                deltas = deltas.offset(width as isize);
            }
        }
        dec.alpha_prev_line_ = prev_line;
    } else {
        // alph_dec->method_ == ALPHA_LOSSLESS_COMPRESSION
        c_assert!(!alph_dec.vp8l_dec_.is_null());
        if !(VP8LDecodeAlphaImageStream(alph_dec, row + num_rows) != 0) {
            return 0;
        }
    }

    if row + num_rows >= height {
        dec.is_alpha_decoded_ = 1;
    }
    return 1;
}

unsafe fn AllocateAlphaPlane(dec: *mut VP8Decoder, io: *const VP8Io) -> c_int {
    let dec: &mut VP8Decoder = &mut *dec;
    let io: &VP8Io = &*io;

    let stride: c_int = io.width;
    let height: c_int = io.crop_bottom;
    let alpha_size = stride as u64 * height as u64;
    c_assert!(dec.alpha_plane_mem_.is_null());
    dec.alpha_plane_mem_ = WebPSafeMalloc(alpha_size, mem::size_of::<u8>()) as *mut u8;
    if dec.alpha_plane_mem_.is_null() {
        return 0;
    }
    dec.alpha_plane_ = dec.alpha_plane_mem_;
    dec.alpha_prev_line_ = ptr::null_mut();
    return 1;
}

#[no_mangle]
pub unsafe extern "C" fn WebPDeallocateAlphaMemory(dec: *mut VP8Decoder) {
    c_assert!(!dec.is_null());
    let dec: &mut VP8Decoder = &mut *dec;

    WebPSafeFree(dec.alpha_plane_mem_ as *mut c_void);
    dec.alpha_plane_mem_ = ptr::null_mut();
    dec.alpha_plane_ = ptr::null_mut();
    ALPHDelete(dec.alph_dec_);
    dec.alph_dec_ = ptr::null_mut();
}

//------------------------------------------------------------------------------
// Main entry point.

#[no_mangle]
pub unsafe extern "C" fn VP8DecompressAlphaRows(
    dec: *mut VP8Decoder,
    io: *const VP8Io,
    row: c_int,
    mut num_rows: c_int,
) -> *const u8 {
    c_assert!(!dec.is_null() && !io.is_null());
    let dec: &mut VP8Decoder = &mut *dec;
    let io: &VP8Io = &*io;

    let width: c_int = io.width;
    let height: c_int = io.crop_bottom;

    if row < 0 || num_rows <= 0 || row + num_rows > height {
        return ptr::null(); // sanity check.
    }

    if !(dec.is_alpha_decoded_ != 0) {
        if dec.alph_dec_.is_null() {
            // Initialize decoder.
            dec.alph_dec_ = ALPHNew();
            if dec.alph_dec_.is_null() {
                return ptr::null();
            }
            if !(AllocateAlphaPlane(dec, io) != 0) {
                WebPDeallocateAlphaMemory(dec);
                return ptr::null();
            }
            if !(ALPHInit(
                dec.alph_dec_,
                dec.alpha_data_,
                dec.alpha_data_size_,
                io,
                dec.alpha_plane_,
            ) != 0)
            {
                WebPDeallocateAlphaMemory(dec);
                return ptr::null();
            }
            // if we allowed use of alpha dithering, check whether it's needed at all
            if (*dec.alph_dec_).pre_processing_ != ALPHA_PREPROCESSED_LEVELS {
                dec.alpha_dithering_ = 0; // disable dithering
            } else {
                num_rows = height - row; // decode everything in one pass
            }
        }

        c_assert!(!dec.alph_dec_.is_null());
        c_assert!(row + num_rows <= height);
        if !(ALPHDecode(dec, row, num_rows) != 0) {
            WebPDeallocateAlphaMemory(dec);
            return ptr::null();
        }

        if dec.is_alpha_decoded_ != 0 {
            // finished?
            ALPHDelete(dec.alph_dec_);
            dec.alph_dec_ = ptr::null_mut();
            if dec.alpha_dithering_ > 0 {
                let alpha: *mut u8 = dec.alpha_plane_
                    .offset(io.crop_top as isize * width as isize + io.crop_left as isize);
                if !(WebPDequantizeLevels(
                    alpha,
                    io.crop_right - io.crop_left,
                    io.crop_bottom - io.crop_top,
                    width,
                    dec.alpha_dithering_,
                ) != 0)
                {
                    WebPDeallocateAlphaMemory(dec);
                    return ptr::null();
                }
            }
        }
    }

    // Return a pointer to the current decoded row.
    return dec.alpha_plane_.offset(row as isize * width as isize);
}
