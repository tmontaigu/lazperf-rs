extern crate lazperf;

use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

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

    laz_file.seek(SeekFrom::Current(SIZEOF_CHUNKTABLE)).unwrap();
    let mut raw_compressed_points = vec![0u8; COMPRESSED_POINTS_DATA_SIZE as usize];
    laz_file.read_exact(raw_compressed_points.as_mut_slice()).unwrap();
    assert_eq!(laz_file.seek(SeekFrom::Current(0)).unwrap(), FILE_SIZE);

    let mut raw_expected_decompressed_point = vec![0u8; NUM_POINTS * POINT_SIZE];
    let mut expected_points_file = File::open("./lazperf-c/tests/data/simple_points_uncompressed.bin").unwrap();
    expected_points_file.read_exact(&mut raw_expected_decompressed_point).unwrap();


    let raw_decompressed_points = lazperf::VlrDecompressor::decompress_points(
        &raw_compressed_points, &laszip_vlr_data, NUM_POINTS, POINT_SIZE);


    assert_eq!(raw_decompressed_points, raw_expected_decompressed_point);
}

#[test]
fn test_streaming_decompress_points() {
    let mut laz_file = File::open("./lazperf-c/tests/data/simple.laz").unwrap();

    laz_file.seek(SeekFrom::Start(OFFSET_TO_LASZIP_VLR_DATA)).unwrap();
    let mut laszip_vlr_data = [0u8; LASZIP_VLR_DATA_SIZE as usize];
    laz_file.read_exact(&mut laszip_vlr_data).unwrap();

    laz_file.seek(SeekFrom::Current(SIZEOF_CHUNKTABLE)).unwrap();
    let mut raw_compressed_points = vec![0u8; COMPRESSED_POINTS_DATA_SIZE as usize];
    laz_file.read_exact(raw_compressed_points.as_mut_slice()).unwrap();
    assert_eq!(laz_file.seek(SeekFrom::Current(0)).unwrap(), FILE_SIZE);

    let mut raw_expected_decompressed_point = vec![0u8; NUM_POINTS * POINT_SIZE];
    let mut expected_points_file = File::open("./lazperf-c/tests/data/simple_points_uncompressed.bin").unwrap();
    expected_points_file.read_exact(&mut raw_expected_decompressed_point).unwrap();

    let decompressor = lazperf::VlrDecompressor::new(&raw_compressed_points, POINT_SIZE, &laszip_vlr_data);
    let mut raw_decompressed_points = vec![0u8; POINT_SIZE * NUM_POINTS];
    let mut cursor = Cursor::new(&mut raw_decompressed_points);
    let mut tmp_buffer = [0u8; POINT_SIZE];

    for _ in 0..NUM_POINTS {
        decompressor.decompress_one_to(&mut tmp_buffer);
        cursor.write_all(&tmp_buffer).unwrap();
    }

    assert_eq!(raw_decompressed_points, raw_expected_decompressed_point);
}


#[test]
fn test_streaming_compression_points() {
    let mut raw_expected_decompressed_point = vec![0u8; NUM_POINTS * POINT_SIZE];
    let mut expected_points_file = File::open("./lazperf-c/tests/data/simple_points_uncompressed.bin").unwrap();
    expected_points_file.read_exact(&mut raw_expected_decompressed_point).unwrap();

    let mut record_schema = lazperf::RecordSchema::new();
    record_schema.push_point();
    record_schema.push_gpstime();
    record_schema.push_rgb();
    let mut vlr_compressor = lazperf::VlrCompressor::new(&record_schema);

    let mut compression_output = Cursor::new(Vec::<u8>::with_capacity(POINT_SIZE * NUM_POINTS));
    for i in 0..NUM_POINTS {
        let current_point = &raw_expected_decompressed_point[i * POINT_SIZE..(i + 1) * POINT_SIZE];
        let compressed_size = vlr_compressor.compress_one(current_point);
        if compressed_size != 0 {
            compression_output.write_all(vlr_compressor.internal_data()).unwrap();
            vlr_compressor.reset_size();
        }
    }

    vlr_compressor.done();
    compression_output.write_all(vlr_compressor.internal_data()).unwrap();
    vlr_compressor.reset_size();

    compression_output.seek(SeekFrom::Current(0)).unwrap();
    vlr_compressor.write_chunk_table();
    compression_output.write_all(vlr_compressor.internal_data()).unwrap();
    let raw_compressed_points = compression_output.into_inner();

    let laszip_vlr_data = vlr_compressor.laszip_vlr_data();
    // skip the chunk table size when decompressing
    let raw_decompressed_points = lazperf::VlrDecompressor::decompress_points(
        &raw_compressed_points[std::mem::size_of::<u64>()..], &laszip_vlr_data, NUM_POINTS, POINT_SIZE);

    assert_eq!(raw_decompressed_points, raw_expected_decompressed_point);
}
