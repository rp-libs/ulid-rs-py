import pytest
import ulid
import ulid_py

num = 10000


# pip install ulid-py~=1.1.0
# Since the package names are the same, you need to go to site-package to change the ulid of ulid-py to ulid_py.
# pip install pytest-benchmark~=4.0.0 ulid-rs-py
# pytest tests/benchmarks/benchmark.py  --benchmark-histogram


@pytest.mark.benchmark(group="new")
def test_rs_new(benchmark):
    def rs_new():
        for _ in range(num):
            ulid.new()

    benchmark(rs_new)


@pytest.mark.benchmark(group="new")
def test_py_new(benchmark):
    def py_new():
        for _ in range(num):
            ulid_py.new()

    benchmark(py_new)


@pytest.mark.benchmark(group="from_str")
def test_rs_from_str(benchmark):
    def rs_from_str():
        for _ in range(num):
            ulid.from_string("01H6D6M1HWY1KNND0FKB8PRR87")

    benchmark(rs_from_str)


@pytest.mark.benchmark(group="from_str")
def test_py_from_str(benchmark):
    def py_from_str():
        for _ in range(num):
            ulid_py.from_str("01H6D6M1HWY1KNND0FKB8PRR87")

    benchmark(py_from_str)


@pytest.mark.benchmark(group="from_uuid")
def test_rs_from_uuid(benchmark):
    import uuid

    a = uuid.uuid4().hex

    def rs_from_uuid():
        for _ in range(num):
            ulid.from_uuid(a)

    benchmark(rs_from_uuid)


@pytest.mark.benchmark(group="from_uuid")
def test_py_from_uuid(benchmark):
    import uuid

    a = uuid.uuid4()

    def py_from_uuid():
        for _ in range(num):
            ulid_py.from_uuid(a)

    benchmark(py_from_uuid)


@pytest.mark.benchmark(group="from_timestamp")
def test_rs_from_timestamp(benchmark):
    from datetime import datetime

    datetime_value = datetime(2023, 7, 28)

    def rs_from_timestamp():
        for _ in range(num):
            ulid.from_timestamp(datetime_value)

    benchmark(rs_from_timestamp)


@pytest.mark.benchmark(group="from_timestamp")
def test_py_from_timestamp(benchmark):
    from datetime import datetime

    datetime_value = datetime(2023, 7, 28)

    def py_from_timestamp():
        for _ in range(num):
            ulid_py.from_timestamp(datetime_value)

    benchmark(py_from_timestamp)
