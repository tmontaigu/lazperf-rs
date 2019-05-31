fn main() {
    cc::Build::new()
        .include("./lazperf-c/laz-perf")
        .file("./lazperf-c/lazperf_c.cpp")
        .compile("lazperf_c");
}
