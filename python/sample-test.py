import unittest
# from hypothesis import given, example
# import hypothesis.strategies as st
from sample import *

class MyFirstTest(unittest.TestCase):

    def test_hello(self):
        self.assertEqual(hello(), 'Hello, Unit Test')

if __name__ == '__main__':
    unittest.main()
