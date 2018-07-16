use std::os::raw::*;

use dec::alpha_dec::ALPHDecoder;
use dec::common_dec::*;
use dec::vp8_dec::VP8Io;
use decode::VP8StatusCode;
use utils::bit_reader_utils::VP8BitReader;
use utils::random_utils::VP8Random;
use utils::thread_utils::WebPWorker;

#[repr(C)]
pub(crate) struct VP8FrameHeader {
    pub(crate) key_frame_: u8,
    pub(crate) profile_: u8,
    pub(crate) show_: u8,
    pub(crate) partition_length_: u32,
}

#[repr(C)]
pub(crate) struct VP8PictureHeader {
    pub(crate) width_: u16,
    pub(crate) height_: u16,
    pub(crate) xscale_: u8,
    pub(crate) yscale_: u8,
    pub(crate) colorspace_: u8, // 0 = YCbCr
    pub(crate) clamp_type_: u8,
}

#[repr(C)]
pub(crate) struct VP8SegmentHeader {
    pub(crate) use_segment_: c_int,
    pub(crate) update_map_: c_int, // whether to update the segment map or not
    pub(crate) absolute_delta_: c_int, // absolute or delta values for quantizer and filter
    pub(crate) quantizer_: [i8; NUM_MB_SEGMENTS as usize], // quantization changes
    pub(crate) filter_strength_: [i8; NUM_MB_SEGMENTS as usize], // filter strength for segments
}

// probas associated to one of the contexts
pub(crate) type VP8ProbaArray = [u8; NUM_PROBAS as usize];

#[repr(C)]
pub(crate) struct VP8BandProbas {
    // all the probas associated to one band
    pub(crate) probas_: [VP8ProbaArray; NUM_CTX as usize],
}

// Struct collecting all frame-persistent probabilities.
#[repr(C)]
pub(crate) struct VP8Proba {
    pub(crate) segments_: [u8; MB_FEATURE_TREE_PROBS as usize],
    // Type: 0:Intra16-AC  1:Intra16-DC   2:Chroma   3:Intra4
    pub(crate) bands_: [[VP8BandProbas; NUM_BANDS as usize]; NUM_TYPES as usize],
    pub(crate) bands_ptr_: [[*const VP8BandProbas; 16 + 1]; NUM_TYPES as usize],
}

#[repr(C)]
pub(crate) struct VP8FilterHeader {
    pub(crate) simple_: c_int,    // 0=complex, 1=simple
    pub(crate) level_: c_int,     // [0..63]
    pub(crate) sharpness_: c_int, // [0..7]
    pub(crate) use_lf_delta_: c_int,
    pub(crate) ref_lf_delta_: [c_int; NUM_REF_LF_DELTAS as usize],
    pub(crate) mode_lf_delta_: [c_int; NUM_MODE_LF_DELTAS as usize],
}

#[repr(C)]
pub(crate) struct VP8FInfo {
    // filter specs
    pub(crate) f_limit_: u8, // filter limit in [3..189], or 0 if no filtering
    pub(crate) f_ilevel_: u8, // inner limit in [1..63]
    pub(crate) f_inner_: u8, // do inner filtering?
    pub(crate) hev_thresh_: u8, // high edge variance threshold in [0..2]
}

#[repr(C)]
pub(crate) struct VP8MB {
    // Top/Left Contexts used for syntax-parsing
    pub(crate) nz_: u8, // non-zero AC/DC coeffs (4bit for luma + 4bit for chroma)
    pub(crate) nz_dc_: u8, // non-zero DC coeff (1bit)
}

// Dequantization matrices
#[allow(non_camel_case_types)]
pub(crate) type quant_t = [c_int; 2]; // [DC / AC].  Can be 'uint16_t[2]' too (~slower).

#[derive(Clone, Copy)]
#[repr(C)]
pub(crate) struct VP8QuantMatrix {
    pub(crate) y1_mat_: quant_t,
    pub(crate) y2_mat_: quant_t,
    pub(crate) uv_mat_: quant_t,

    pub(crate) uv_quant_: c_int, // U/V quantizer value
    pub(crate) dither_: c_int,   // dithering amplitude (0 = off, max=255)
}

// Data needed to reconstruct a macroblock
#[repr(C)]
pub(crate) struct VP8MBData {
    pub(crate) coeffs_: [i16; 384], // 384 coeffs = (16+4+4) * 4*4
    pub(crate) is_i4x4_: u8,        // true if intra4x4
    pub(crate) imodes_: [u8; 16],   // one 16x16 mode (#0) or sixteen 4x4 modes
    pub(crate) uvmode_: u8,         // chroma prediction mode
    // bit-wise info about the content of each sub-4x4 blocks (in decoding order).
    // Each of the 4x4 blocks for y/u/v is associated with a 2b code according to:
    //   code=0 -> no coefficient
    //   code=1 -> only DC
    //   code=2 -> first three coefficients are non-zero
    //   code=3 -> more than three coefficients are non-zero
    // This allows to call specialized transform functions.
    pub(crate) non_zero_y_: u32,
    pub(crate) non_zero_uv_: u32,
    pub(crate) dither_: u8, // local dithering strength (deduced from non_zero_*)
    pub(crate) skip_: u8,
    pub(crate) segment_: u8,
}

#[repr(C)]
pub(crate) struct VP8ThreadContext {
    pub(crate) id_: c_int,               // cache row to process (in [0..2])
    pub(crate) mb_y_: c_int,             // macroblock position of the row
    pub(crate) filter_row_: c_int,       // true if row-filtering is needed
    pub(crate) f_info_: *mut VP8FInfo,   // filter strengths (swapped with dec->f_info_)
    pub(crate) mb_data_: *mut VP8MBData, // reconstruction data (swapped with dec->mb_data_)
    pub(crate) io_: VP8Io,               // copy of the VP8Io to pass to put()
}

// Saved top samples, per macroblock. Fits into a cache-line.
#[repr(C)]
pub(crate) struct VP8TopSamples {
    pub(crate) y: [u8; 16],
    pub(crate) u: [u8; 8],
    pub(crate) v: [u8; 8],
}

#[repr(C)]
pub struct VP8Decoder {
    pub(crate) status_: VP8StatusCode,
    pub(crate) ready_: c_int, // true if ready to decode a picture with VP8Decode()
    pub(crate) error_msg_: *const c_char, // set when status_ is not OK.

    // Main data source
    pub(crate) br_: VP8BitReader,

    // headers
    pub(crate) frm_hdr_: VP8FrameHeader,
    pub(crate) pic_hdr_: VP8PictureHeader,
    pub(crate) filter_hdr_: VP8FilterHeader,
    pub(crate) segment_hdr_: VP8SegmentHeader,

    // Worker
    pub(crate) worker_: WebPWorker,
    pub(crate) mt_method_: c_int, // multi-thread method: 0=off, 1=[parse+recon][filter]
    // 2=[parse][recon+filter]
    pub(crate) cache_id_: c_int,              // current cache row
    pub(crate) num_caches_: c_int,            // number of cached rows of 16 pixels (1, 2 or 3)
    pub(crate) thread_ctx_: VP8ThreadContext, // Thread context

    // dimension, in macroblock units.
    pub(crate) mb_w_: c_int,
    pub(crate) mb_h_: c_int,

    // Macroblock to process/filter, depending on cropping and filter_type.
    pub(crate) tl_mb_x_: c_int, // top-left MB that must be in-loop filtered
    pub(crate) tl_mb_y_: c_int, // top-left MB that must be in-loop filtered
    pub(crate) br_mb_x_: c_int, // last bottom-right MB that must be decoded
    pub(crate) br_mb_y_: c_int, // last bottom-right MB that must be decoded

    // number of partitions minus one.
    pub(crate) num_parts_minus_one_: u32,
    // per-partition boolean decoders.
    pub(crate) parts_: [VP8BitReader; MAX_NUM_PARTITIONS as usize],

    // Dithering strength, deduced from decoding options
    pub(crate) dither_: c_int,           // whether to use dithering or not
    pub(crate) dithering_rg_: VP8Random, // random generator for dithering

    // dequantization (one set of DC/AC dequant factor per segment)
    pub(crate) dqm_: [VP8QuantMatrix; NUM_MB_SEGMENTS as usize],

    // probabilities
    pub(crate) proba_: VP8Proba,
    pub(crate) use_skip_proba_: c_int,
    pub(crate) skip_p_: u8,

    // Boundary data cache and persistent buffers.
    pub(crate) intra_t_: *mut u8, // top intra modes values: 4 * mb_w_
    pub(crate) intra_l_: [u8; 4], // left intra modes values

    pub(crate) yuv_t_: *mut VP8TopSamples, // top y/u/v samples

    pub(crate) mb_info_: *mut VP8MB, // contextual macroblock info (mb_w_ + 1)
    pub(crate) f_info_: *mut VP8FInfo, // filter strength info
    pub(crate) yuv_b_: *mut u8,      // main block for Y/U/V (size = YUV_SIZE)

    pub(crate) cache_y_: *mut u8, // macroblock row for storing unfiltered samples
    pub(crate) cache_u_: *mut u8,
    pub(crate) cache_v_: *mut u8,
    pub(crate) cache_y_stride_: c_int,
    pub(crate) cache_uv_stride_: c_int,

    // main memory chunk for the above data. Persistent.
    pub(crate) mem_: *mut c_void,
    pub(crate) mem_size_: usize,

    // Per macroblock non-persistent infos.
    pub(crate) mb_x_: c_int, // current position, in macroblock units
    pub(crate) mb_y_: c_int, // current position, in macroblock units
    pub(crate) mb_data_: *mut VP8MBData, // parsed reconstruction data

    // Filtering side-info
    pub(crate) filter_type_: c_int, // 0=off, 1=simple, 2=complex
    pub(crate) fstrengths_: [[VP8FInfo; 2]; NUM_MB_SEGMENTS as usize], // precalculated per-segment/type

    // Alpha
    pub(crate) alph_dec_: *mut ALPHDecoder, // alpha-plane decoder object
    pub(crate) alpha_data_: *const u8,      // compressed alpha data (if present)
    pub(crate) alpha_data_size_: usize,
    pub(crate) is_alpha_decoded_: c_int, // true if alpha_data_ is decoded in alpha_plane_
    pub(crate) alpha_plane_mem_: *mut u8, // memory allocated for alpha_plane_
    pub(crate) alpha_plane_: *mut u8,    // output. Persistent, contains the whole data.
    pub(crate) alpha_prev_line_: *const u8, // last decoded alpha row (or NULL)
    pub(crate) alpha_dithering_: c_int,  // derived from decoding options (0=off, 100=full)
}
