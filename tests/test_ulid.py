import uuid
import pytest
from datetime import datetime
from ulid import (
    new,
    PyUlid,
    from_string,
    from_uuid,
    from_parts,
    from_timestamp,
    DecodeError,
    InvalidUuidError,
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
    assert py_ulid.bytes() == [
        1,
        104,
        211,
        255,
        0,
        207,
        134,
        89,
        173,
        212,
        154,
        22,
        211,
        105,
        197,
        255,
    ]
    assert py_ulid.randomness() == 634451394732979059803647
    assert py_ulid.timestamp() == 1549744931023
    assert py_ulid.str() == "01D39ZY06FGSCTVN4T2V9PKHFZ"
    assert py_ulid.increment().str() == "01D39ZY06FGSCTVN4T2V9PKHG0"


def test_from_uuid():
    value = uuid.uuid4().hex
    py_ulid = from_uuid(value)
    assert repr(py_ulid)
    assert type(py_ulid) is PyUlid
    assert py_ulid.str()
    assert py_ulid.timestamp()
    assert py_ulid.randomness()
    assert py_ulid.bytes()
    assert py_ulid.increment()
    assert py_ulid.randomness() + 1 == py_ulid.increment().randomness()


def test_from_timestamp():
    py_ulid = from_timestamp(datetime(1999, 1, 1))
    assert py_ulid.str()
    assert type(py_ulid) is PyUlid
    assert py_ulid.timestamp() == 915148800000
    assert py_ulid.randomness()
    assert py_ulid.bytes()
    assert py_ulid.increment()


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
    with pytest.raises(InvalidUuidError):
        from_uuid("q")
