extern crate lazperf;

use std::fs::File;
use std::io::{Seek, SeekFrom, Read};

const LAS_HEADER_SIZE: u64 = 227;
const VLR_HEADER_SIZE: u64 = 54;
const OFFSET_TO_LASZIP_VLR_DATA: u64 = LAS_HEADER_SIZE + VLR_HEADER_SIZE;
const LASZIP_VLR_DATA_SIZE: u64 = 52;
const SIZEOF_CHUNKTABLE: i64 = 8;
const FILE_SIZE: u64 = 18217;
const COMPRESSED_POINTS_DATA_SIZE: u64 = FILE_SIZE - (LAS_HEADER_SIZE + VLR_HEADER_SIZE + LASZIP_VLR_DATA_SIZE + SIZEOF_CHUNKTABLE as u64);
const POINT_SIZE: usize = 34;
const NUM_POINTS: usize = 1065;

#[test]
fn test_decompress_points() {
    let mut laz_file = File::open("./lazperf-c/tests/data/simple.laz").unwrap();

    laz_file.seek(SeekFrom::Start(OFFSET_TO_LASZIP_VLR_DATA)).unwrap();
    let mut laszip_vlr_data = [0u8; LASZIP_VLR_DATA_SIZE as usize];
    laz_file.read_exact(&mut laszip_vlr_data).unwrap();

    laz_file.seek(SeekFrom::Current(SIZEOF_CHUNKTABLE));
    let mut raw_compressed_points = vec![0u8; COMPRESSED_POINTS_DATA_SIZE as usize];
    laz_file.read_exact(raw_compressed_points.as_mut_slice()).unwrap();
    assert_eq!(laz_file.seek(SeekFrom::Current(0)).unwrap(), FILE_SIZE);

    let mut raw_expected_decompressed_point = vec![0u8; NUM_POINTS * POINT_SIZE];
    let mut expected_points_file = File::open("./lazperf-c/tests/data/simple_points_uncompressed.bin").unwrap();
    expected_points_file.read_exact(&mut raw_expected_decompressed_point).unwrap();



    let raw_decompressed_points = lazperf::decompress_points(&raw_compressed_points, &laszip_vlr_data, NUM_POINTS, POINT_SIZE).unwrap();

    assert_eq!(raw_decompressed_points, raw_expected_decompressed_point);
}