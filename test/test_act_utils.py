import unittest

from nari.ext.act.utils import validate_checksum
from nari.ext.act.exceptions import InvalidActChecksum


class TestLineChecksum(unittest.TestCase):

    def test_sha256_line(self):
        """
        Tests ACT SHA256 checksum validation for lines
        """
        test_line = "253|2022-02-09T20:09:52.6303877-06:00|FFXIV_ACT_Plugin Version: 2.6.4.1 (50BCD605C50A749F)|5401dc333f466389"
        self.assertEqual(validate_checksum(test_line, 1), True)

    def test_old_checksum(self):
        """
        Tests ACT checksum validation against an old version
        """
        test_line = "253|2020-09-10T22:36:46.6756722-04:00|FFXIV PLUGIN VERSION: 2.0.6.8|4b16c21ba358b9543c75ad2f090cac53"
        self.assertEqual(validate_checksum(test_line, 1), False)


if __name__ == '__main__':
    unittest.main()
