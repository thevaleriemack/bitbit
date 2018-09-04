import unittest
from hypothesis import given, example
import hypothesis.strategies as st
from mining_simplified import *
import hashlib

class TestConcatenate(unittest.TestCase):

    @given(n=st.integers(), p=st.text(), t=st.lists(st.text()))
    def test_expected_concatenation(self, n, p, t):
        ex = 'IamSatoshi'
        self.assertEqual(ex, concatenate('I', 'am', 'Satoshi'))
        ex1 = "".join([str(n), p, "".join(t)])
        self.assertEqual(ex1, concatenate(n, p, t))

class TestHashSha256(unittest.TestCase):

    def test_expected_hash(self):
        ex = '2d3ba8456fb4b275cfc0e5a0e189b4603174ddbbb55d743853564e2f7b24155a'
        self.assertEqual(ex, hash_sha256('Alice Nakamoto'))

    @given(c=st.text())
    @example(c='Alice Nakamoto')
    def test_consistent_hash(self, c):
        ex = hash_sha256(c)
        self.assertEqual(ex, hash_sha256(c))

    @given(n=st.integers())
    @example(n=-1)
    def test_string_conversion(self, n):
        ex = '5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9'
        self.assertEqual(ex, hash_sha256(0))
        self.assertIsNotNone(hash_sha256(n), 'message')

if __name__ == '__main__':
    unittest.main()
