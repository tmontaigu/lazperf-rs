use libc;

mod ffi;

#[derive(Debug)]
pub enum Error {
    Failed,
}

#[derive(Debug)]
pub struct VlrDecompressor {
    decompressor: ffi::LazPerf_VlrDecompressorPtr
}


impl VlrDecompressor {
    pub fn new(compressed_points: &[u8], point_size: usize, laszip_vlr_data: &[u8]) -> Self {
        unsafe {
            let decompressor = ffi::lazperf_new_vlr_decompressor(
                compressed_points.as_ptr() as *const libc::c_char,
                compressed_points.len() as libc::size_t,
                point_size as libc::size_t,
                laszip_vlr_data.as_ptr() as *const libc::c_char,
            );
            VlrDecompressor { decompressor }
        }
    }

    pub fn decompress_one_to(&self, out: &mut [u8]) {
        unsafe {
            ffi::lazperf_vlr_decompressor_decompress_one_to(
                self.decompressor,
                out.as_mut_ptr() as *mut libc::c_char,
            );
        }
    }

    pub fn decompress_points(
        raw_compressed_points: &[u8],
        laszip_vlr_data: &[u8],
        num_points: usize,
        point_size: usize,
    ) -> Vec<u8> {
        let mut out_buffer = vec![0u8; num_points * point_size];
        unsafe {
            ffi::lazperf_decompress_points_into(
                raw_compressed_points.as_ptr() as *const libc::c_char,
                raw_compressed_points.len(),
                laszip_vlr_data.as_ptr() as *const libc::c_char,
                num_points,
                point_size,
                out_buffer.as_mut_ptr() as *mut libc::uint8_t,
            );
            out_buffer
        }
    }
}

impl Drop for VlrDecompressor {
    fn drop(&mut self) {
        unsafe { ffi::lazperf_delete_vlr_decompressor(self.decompressor); }
    }
}


pub struct RecordSchema {
    record_schema: ffi::LazPerf_RecordSchemaPtr
}

impl RecordSchema {
    pub fn new() -> Self {
        let record_schema = unsafe { ffi::lazperf_new_record_schema() };
        RecordSchema { record_schema }
    }

    pub fn push_point(&mut self) {
        unsafe { ffi::lazperf_record_schema_push_point(self.record_schema); }
    }

    pub fn push_gpstime(&mut self) {
        unsafe { ffi::lazperf_record_schema_push_gpstime(self.record_schema); }
    }
    pub fn push_rgb(&mut self) {
        unsafe { ffi::lazperf_record_schema_push_rgb(self.record_schema); }
    }
    pub fn push_extrabytes(&mut self, count: usize) {
        unsafe { ffi::lazperf_record_schema_push_extrabytes(self.record_schema, count as libc::size_t); }
    }

    pub fn size_in_bytes(&self) -> i32 {
        unsafe { ffi::lazperf_record_schema_size_in_bytes(self.record_schema) as i32 }
    }
}

impl Drop for RecordSchema {
    fn drop(&mut self) {
        unsafe { ffi::lazperf_delete_record_schema(self.record_schema) };
    }
}

#[derive(Debug)]
pub struct VlrCompressor {
    compressor: ffi::LazPerfVlr_CompressorPtr
}

impl VlrCompressor {
    pub fn new(record_schema: &RecordSchema) -> Self {
        let compressor = unsafe { ffi::lazperf_new_vlr_compressor(record_schema.record_schema) };
        VlrCompressor { compressor }
    }

    pub fn compress_one(&mut self, raw_point: &[u8]) -> usize {
        //TODO assert_eq!(raw_point.len() >= compressor.schema.size)
        let compressed_size = unsafe {
            ffi::lazperf_vlr_compressor_compress(
                self.compressor, raw_point.as_ptr() as *const libc::c_char)
        };
        compressed_size as usize
    }

    pub fn done(&mut self) -> usize {
        unsafe {
            return ffi::lazperf_vlr_compressor_done(self.compressor) as usize;
        };
    }

    pub fn write_chunk_table(&mut self) -> usize {
        let size = unsafe {
            ffi::lazperf_vlr_compressor_write_chunk_table(self.compressor)
        };

        size as usize
    }

    pub fn extract_data_to(&mut self) -> usize {
        0
    }

    pub fn reset_size(&mut self) {
        unsafe {
            ffi::lazperf_vlr_compressor_reset_size(self.compressor);
        }
    }

    pub fn internal_data(&self) -> &[u8] {
        unsafe {
            let ptr = ffi::lazperf_vlr_compressor_internal_buffer(self.compressor);
            let size = ffi::lazperf_vlr_compressor_internal_buffer_size(self.compressor);
            std::slice::from_raw_parts(ptr, size)
        }
    }

    pub fn laszip_vlr_data(&self) -> Vec<u8> {
        unsafe {
            let vlr_data = ffi::lazperf_vlr_compressor_vlr_data(self.compressor);
            let mut data = vec![0u8; vlr_data.size];
            (vlr_data.data as *const u8).copy_to(data.as_mut_ptr(), vlr_data.size);
            ffi::lazperf_delete_sized_buffer(vlr_data);
            data
        }
    }
}

impl Drop for VlrCompressor {
    fn drop(&mut self) {
        unsafe {
            ffi::lazperf_delete_vlr_compressor(self.compressor);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_record_schema() {
        let mut record_schema = super::RecordSchema::new();
        assert_eq!(record_schema.size_in_bytes(), 0);

        record_schema.push_point();
        assert_eq!(record_schema.size_in_bytes(), 20);
        record_schema.push_gpstime();

        assert_eq!(record_schema.size_in_bytes(), 28);
        record_schema.push_rgb();
        assert_eq!(record_schema.size_in_bytes(), 34);

        record_schema.push_extrabytes(6);
        assert_eq!(record_schema.size_in_bytes(), 40);
    }

    #[test]
    fn point_vlr_data_not_empty() {
        {
            let mut record_schema = super::RecordSchema::new();
            record_schema.push_point();

            let compressor = super::VlrCompressor::new(&record_schema);
            assert!(!compressor.laszip_vlr_data().is_empty());
        }
    }
    #[test]
    fn point_gps_vlr_data_not_empty() {
        let mut record_schema = super::RecordSchema::new();
        record_schema.push_point();
        record_schema.push_gpstime();

        let compressor = super::VlrCompressor::new(&record_schema);
        assert!(!compressor.laszip_vlr_data().is_empty());
    }

    #[test]
    fn point_gps_rgb_vlr_data_not_empty() {
        let mut record_schema = super::RecordSchema::new();
        record_schema.push_point();
        record_schema.push_gpstime();
        record_schema.push_rgb();

        let compressor = super::VlrCompressor::new(&record_schema);
        assert!(!compressor.laszip_vlr_data().is_empty());
    }

    #[test]
    fn point_rgb_vlr_data_not_empty() {
        let mut record_schema = super::RecordSchema::new();
        record_schema.push_point();
        record_schema.push_rgb();

        let compressor = super::VlrCompressor::new(&record_schema);
        assert!(!compressor.laszip_vlr_data().is_empty());
    }
}
