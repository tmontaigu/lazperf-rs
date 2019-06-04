#![allow(dead_code, non_camel_case_types)]

use std::os::raw::c_void;

use libc;

/* Some LazPerf data structs */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LazPerf_SizedBuffer {
    pub data: *mut libc::c_char,
    pub size: libc::size_t,
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct LazPerf_Error {
    pub error_msg: *const libc::c_char
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Union {
    pub error: LazPerf_Error,
    pub points_buffer: LazPerf_SizedBuffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LazPerf_BufferResult {
    pub is_error: libc::c_int,
    pub content: Union,
}

impl LazPerf_BufferResult {
    pub fn is_error(&self) -> bool {
        self.is_error == 1
    }
}

extern {
    pub fn lazperf_delete_sized_buffer(buffer: LazPerf_SizedBuffer);
}


/* Record Schema */
pub type LazPerf_RecordSchemaPtr = *mut c_void;

extern {
    pub fn lazperf_new_record_schema() -> LazPerf_RecordSchemaPtr;
    pub fn lazperf_delete_record_schema(schema: LazPerf_RecordSchemaPtr);
    pub fn lazperf_record_schema_push_point(schema: LazPerf_RecordSchemaPtr);
    pub fn lazperf_record_schema_push_gpstime(schema: LazPerf_RecordSchemaPtr);
    pub fn lazperf_record_schema_push_rgb(schema: LazPerf_RecordSchemaPtr);
    pub fn lazperf_record_schema_push_extrabytes(schema: LazPerf_RecordSchemaPtr, count: libc::size_t);
    pub fn lazperf_record_schema_size_in_bytes(schema: LazPerf_RecordSchemaPtr) -> libc::c_int;
}

/* Decompression API */
pub type LazPerf_VlrDecompressorPtr = *mut c_void;

extern {
    pub fn lazperf_decompress_points_into(
        compressed_points_buffer: *const libc::c_char,
        buffer_size: libc::size_t,
        laszip_vlr_data: *const libc::c_char,
        num_points: libc::size_t,
        point_size: libc::size_t,
        out_buffer: *mut u8,
    );

    pub fn lazperf_new_vlr_decompressor(
        compressed_points_buffer: *const libc::c_char,
        buffer_size: libc::size_t,
        point_size: libc::size_t,
        laszip_vlr_data: *const libc::c_char,
    ) -> LazPerf_VlrDecompressorPtr;

    pub fn lazperf_delete_vlr_decompressor(decompressor: LazPerf_VlrDecompressorPtr);
    pub fn lazperf_vlr_decompressor_decompress_one_to(
        decompressor: LazPerf_VlrDecompressorPtr,
        out: *mut libc::c_char,
    );
}


/* Compression API */
pub type LazPerfVlr_CompressorPtr = *mut c_void;

extern {
    pub fn lazperf_new_vlr_compressor(schema: LazPerf_RecordSchemaPtr) -> LazPerfVlr_CompressorPtr;
    pub fn lazperf_vlr_compressor_compress(compressor: LazPerfVlr_CompressorPtr, input: *const libc::c_char) -> libc::size_t;
    pub fn lazperf_vlr_compressor_done(compressor: LazPerfVlr_CompressorPtr) -> u64;
    pub fn lazperf_vlr_compressor_write_chunk_table(compressor: LazPerfVlr_CompressorPtr) -> u64;
    pub fn lazperf_vlr_compressor_internal_buffer(compressor: LazPerfVlr_CompressorPtr) -> *const u8;
    pub fn lazperf_vlr_compressor_internal_buffer_size(compressor: LazPerfVlr_CompressorPtr) -> libc::size_t;
    pub fn lazperf_delete_vlr_compressor(compressor: LazPerfVlr_CompressorPtr);
    pub fn lazperf_vlr_compressor_reset_size(compressor: LazPerfVlr_CompressorPtr);
    pub fn lazperf_vlr_compressor_vlr_data(compressor: LazPerfVlr_CompressorPtr) -> LazPerf_SizedBuffer;
}