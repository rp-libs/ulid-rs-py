## Benchmarks
These benchmarks are intended only as a guide for the simplest scenarios.

### Test Environment
1. M1pro Mac 10core 32gb
2. Python3.9

### Test Comparison Package
1. [ulid-py](https://github.com/ahawker/ulid) v1.1.0

### Test Result:
**1. Generate ulid 10x performance improvement:**
![new](./result/benchmark_20230728_090826-new.svg)
**2. From str 21x performance improvement:**
![str](./result/benchmark_20230728_090826-from_str.svg)
**3. From timestamp 16x performance improvement:**
![timestamp](./result/benchmark_20230728_090826-from_timestamp.svg)
**4. From uuid 2.5x performance improvement:**
![uuid](./result/benchmark_20230728_090826-from_uuid.svg)
