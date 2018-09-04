import unittest
# from hypothesis import given, example
# import hypothesis.strategies as st
import person
from bitcoin import *

class TestPerson(unittest.TestCase):

    def test_new_person(self):
        alice = person.Person('Alice', 'Nakamoto')
        priv = sha256('Alice Nakamoto')
        pub = privtopub(priv)
        addr = pubtoaddr(pub)
        self.assertEqual(alice.private_key, priv)
        self.assertEqual(alice.public_key, pub)
        self.assertEqual(alice.address, addr)

if __name__ == '__main__':
    unittest.main()
