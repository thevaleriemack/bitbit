from bitcoin import *

class Person:
    """A person in the Cryptogym"""
    public_key = None
    private_key = None
    address = None

    def __init__(self, first_name, last_name):
        """Customize person construction with first and last name"""
        # It would be better to hash something random that no one would be able
        # to guess
        # bitcoin.random_key() # a random hex string
        full_name = " ".join([first_name, last_name])
        self.private_key = sha256(full_name)
        self.public_key = privtopub(self.private_key)
        self.address = pubtoaddr(self.public_key)
