use libc;

extern {
    pub fn hello();
    pub fn lazperf_decompress_points(
        compressed_points_buffer: *const libc::c_char,
        buffer_size: libc::size_t,
        laszip_vlr_data: *const libc::c_char,
        num_points: libc::size_t,
        point_size: libc::size_t
    ) -> *mut libc::c_char;
}