use std::os::raw::c_void;

use libc;

/* Some LazPerf data structs */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LazPerfSizedBuffer {
    pub data: *mut libc::c_char,
    pub size: libc::size_t,
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct LazPerfError {
    pub error_msg: *const libc::c_char
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Union {
    pub error: LazPerfError,
    pub points_buffer: LazPerfSizedBuffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LazPerfResult {
    pub is_error: libc::c_int,
    pub content: Union,
}

impl LazPerfResult {
    pub fn is_error(&self) -> bool {
        self.is_error == 1
    }
}

extern {
    fn lazperf_delete_sized_buffer(buffer: *mut LazPerfSizedBuffer);
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
        out_buffer: *mut libc::uint8_t,
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
pub type LazPerf_VlrCompressorPtr = *mut c_void;

extern {
    pub fn lazperf_new_vlr_compressor(schema: LazPerf_RecordSchemaPtr) -> LazPerf_VlrCompressorPtr;
    pub fn lazperf_vlr_compressor_compress(compressor: LazPerf_VlrCompressorPtr, input: *const libc::c_char) -> libc::size_t;
    pub fn lazperf_vlr_compressor_extract_data_to(compressor: LazPerf_VlrCompressorPtr, dst: *mut libc::uint8_t) -> libc::size_t;
    pub fn lazperf_vlr_compressor_done(compressor: LazPerf_VlrCompressorPtr) -> libc::uint64_t;
    pub fn lazperf_vlr_compressor_write_chunk_table(compressor: LazPerf_VlrCompressorPtr) -> libc::uint64_t;
    pub fn lazperf_vlr_compressor_internal_buffer(compressor: LazPerf_VlrCompressorPtr) -> *const libc::uint8_t;
    pub fn lazperf_vlr_compressor_internal_buffer_size(compressor: LazPerf_VlrCompressorPtr) -> libc::size_t;
    pub fn lazperf_delete_vlr_compressor(compressor: LazPerf_VlrCompressorPtr);
    pub fn lazperf_vlr_compressor_reset_size(compressor: LazPerf_VlrCompressorPtr);
    pub fn lazperf_vlr_compressor_vlr_data(compressor: LazPerf_VlrCompressorPtr) -> LazPerfSizedBuffer;
}