use libc;

mod ffi;

#[derive(Debug)]
pub enum Error {
    Failed,
}

pub fn decompress_points(raw_compressed_points: &[u8], laszip_vlr_data: &[u8], num_points: usize, point_size: usize) -> Result<Vec<u8>, Error> {
    unsafe {
        let decompressed_points = ffi::lazperf_decompress_points(
            raw_compressed_points.as_ptr() as *const libc::c_char,
            raw_compressed_points.len(),
            laszip_vlr_data.as_ptr() as *const libc::c_char,
            num_points,
            point_size
        );
        if decompressed_points.is_null() {
            Err(Error::Failed)
        } else {
            Ok(Vec::<u8>::from_raw_parts(decompressed_points as *mut u8, num_points * point_size, num_points * point_size))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
