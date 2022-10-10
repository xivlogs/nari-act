"""Helpful utilities"""
from datetime import datetime
from hashlib import sha256
from typing import Callable

from nari.types import Timestamp


DEFAULT_DATE_FORMAT: str = '%Y-%m-%dT%H:%M:%S.%f%z'


try:
    from nari_act_rust.utils import date_from_cs_string, validate_checksum_internal as validate_checksum_internal_rs
except ImportError:
    def date_from_cs_string(datestr: str) -> Timestamp:
        """Look, this is dirty. This is wrong. Please someone find a better way to do this."""
        return int(datetime.strptime(f'{datestr[:26]}{datestr[-6:]}', DEFAULT_DATE_FORMAT).timestamp() * 1000)


def validate_checksum_internal_py(line: str, index: int) -> bool:
    """Validates an ACT log line internal function"""
    parts = line.split('|')
    check_hash = parts[-1]
    to_hash = f'{"|".join(parts[:-1])}|{index}'.encode('utf-8')

    return sha256(to_hash).hexdigest()[:16] == check_hash

validate_checksum_internal: Callable[[str, int], bool] = validate_checksum_internal_rs or validate_checksum_internal_py

def date_from_act_timestamp(datestr: str) -> Timestamp:
    """Parse timestamp from ACT log into a Timestamp"""
    return date_from_cs_string(datestr)

def validate_checksum(line: str, index: int) -> bool:
    """Validates an ACT log line
    Given some line 1|foo|bar|baz|a823425f532c540667195f641dd3649b, and an index of 1, then the md5sum of
    1|foo|bar|baz|1 (where 1 is the index) should be a823425f532c540667195f641dd3649b (which is the checksum value)
    """
    return validate_checksum_internal(line, index)
