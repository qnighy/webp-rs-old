// Copyright 2015 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Definitions and macros common to encoding and decoding
//
// Author: Skal (pascal.massimino@gmail.com)
// Port to Rust: Masaki Hara (ackie.h.gmai@gmail.com)

#![allow(dead_code)]

use std::os::raw::*;

// intra prediction modes
pub(crate) const B_DC_PRED: c_int = 0; // 4x4 modes
pub(crate) const B_TM_PRED: c_int = 1;
pub(crate) const B_VE_PRED: c_int = 2;
pub(crate) const B_HE_PRED: c_int = 3;
pub(crate) const B_RD_PRED: c_int = 4;
pub(crate) const B_VR_PRED: c_int = 5;
pub(crate) const B_LD_PRED: c_int = 6;
pub(crate) const B_VL_PRED: c_int = 7;
pub(crate) const B_HD_PRED: c_int = 8;
pub(crate) const B_HU_PRED: c_int = 9;
pub(crate) const NUM_BMODES: c_int = B_HU_PRED + 1 - B_DC_PRED; // = 10

// Luma16 or UV modes
pub(crate) const DC_PRED: c_int = B_DC_PRED;
pub(crate) const V_PRED: c_int = B_VE_PRED;
pub(crate) const H_PRED: c_int = B_HE_PRED;
pub(crate) const TM_PRED: c_int = B_TM_PRED;
pub(crate) const B_PRED: c_int = NUM_BMODES; // refined I4x4 mode
pub(crate) const NUM_PRED_MODES: c_int = 4;

// special modes
pub(crate) const B_DC_PRED_NOTOP: c_int = 4;
pub(crate) const B_DC_PRED_NOLEFT: c_int = 5;
pub(crate) const B_DC_PRED_NOTOPLEFT: c_int = 6;
pub(crate) const NUM_B_DC_MODES: c_int = 7;

pub(crate) const MB_FEATURE_TREE_PROBS: c_int = 3;
pub(crate) const NUM_MB_SEGMENTS: c_int = 4;
pub(crate) const NUM_REF_LF_DELTAS: c_int = 4;
pub(crate) const NUM_MODE_LF_DELTAS: c_int = 4; // I4x4, ZERO, *, SPLIT
pub(crate) const MAX_NUM_PARTITIONS: c_int = 8;
// Probabilities
pub(crate) const NUM_TYPES: c_int = 4; // 0: i16-AC,  1: i16-DC,  2:chroma-AC,  3:i4-AC
pub(crate) const NUM_BANDS: c_int = 8;
pub(crate) const NUM_CTX: c_int = 3;
pub(crate) const NUM_PROBAS: c_int = 11;
