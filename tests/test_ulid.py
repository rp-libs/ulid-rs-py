import uuid
import pytest
from datetime import datetime, timezone
from ulid import (
    new,
    PyUlid,
    from_string,
    from_uuid,
    from_parts,
    from_timestamp,
    DecodeError,
    from_datetime,
)


def test_new():
    py_ulid = new()
    assert repr(py_ulid)
    assert type(py_ulid) is PyUlid
    assert py_ulid.str()
    assert py_ulid.timestamp()
    assert py_ulid.randomness()
    assert py_ulid.bytes()
    assert py_ulid.increment()
    assert py_ulid.randomness() + 1 == py_ulid.increment().randomness()


def test_from_string():
    py_ulid = from_string("01D39ZY06FGSCTVN4T2V9PKHFZ")
    assert repr(py_ulid) == "<ULID('01D39ZY06FGSCTVN4T2V9PKHFZ')>"
    assert type(py_ulid) is PyUlid
    print(py_ulid.bytes())
    assert py_ulid.bytes() == b"\x01h\xd3\xff\x00\xcf\x86Y\xad\xd4\x9a\x16\xd3i\xc5\xff"
    assert py_ulid.randomness() == 634451394732979059803647
    assert py_ulid.timestamp() == 1549744931.0
    assert py_ulid.str() == "01D39ZY06FGSCTVN4T2V9PKHFZ"
    assert py_ulid.increment().str() == "01D39ZY06FGSCTVN4T2V9PKHG0"


def test_from_uuid():
    py_ulid = from_uuid(uuid.uuid4())
    assert repr(py_ulid)
    assert type(py_ulid) is PyUlid
    assert py_ulid.str()
    assert py_ulid.timestamp()
    assert py_ulid.randomness()
    assert py_ulid.bytes()
    assert py_ulid.increment()
    assert py_ulid.randomness() + 1 == py_ulid.increment().randomness()


def test_from_timestamp():
    timestamp = datetime(1999, 1, 1).timestamp()
    py_ulid = from_timestamp(timestamp)
    assert py_ulid.str()
    assert type(py_ulid) is PyUlid
    assert py_ulid.timestamp() == timestamp
    assert py_ulid.randomness()
    assert py_ulid.bytes()
    assert py_ulid.increment()


def test_from_datetime():
    datetime_value = datetime(2023, 7, 28, hour=1, minute=20, tzinfo=timezone.utc)
    py_ulid = from_datetime(datetime_value)
    assert py_ulid.str()
    assert py_ulid.datetime() == datetime(2023, 7, 28, hour=1, minute=20)
    assert py_ulid.timestamp() == datetime_value.timestamp()


def test_from_parts():
    a = new()
    py_ulid = from_parts(a.timestamp(), a.randomness())
    assert py_ulid.str()
    assert type(py_ulid) is PyUlid
    assert py_ulid.timestamp()
    assert py_ulid.randomness()
    assert py_ulid.bytes()
    assert py_ulid.increment()


def test_from_string_decode_error():
    with pytest.raises(DecodeError):
        from_string("q")


def test_from_uuid_invalid_uuid_error():
    with pytest.raises(TypeError):
        from_uuid("q")
