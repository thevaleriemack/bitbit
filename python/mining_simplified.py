import hashlib

# The hash puzzle includes 3 pieces of data:
# A nonce, the hash of the previous block, and a set of transactions

def concatenate(nonce, prev_hash, transactions):
    # We have to stringify it in order to get a concatenated value
    nonce_str = str(nonce)
    transactions_str = ''.join(transactions)
    return ''.join([nonce_str, prev_hash, transactions_str])

# Now we want to hash this concatenated value.

def hash_sha256(concatenated):
    # Bitcoin uses the SHA256 hash algorithm for mining
    return hashlib.sha256(str(concatenated).encode('utf-8')).hexdigest()

# If we get a hash value inside of our defined target space we have found a
# solution to the puzzle. In this case, we will print the hash solution (which
# will be this block's hash) and return true.

def solve_hash_puzzle(nonce, prev_hash, transactions):
    concatenated = concatenate(nonce, prev_hash, transactions)
    proposed_solution = hash_sha256(concatenated)
    # For the sake of example, we will define our target space as any hash
    # output with 2 leading zeros. In Bitcoin it is actually 32 leading zeros.
    if (proposed_solution[0:2] == '00'):
        print(f'Solution found: {proposed_solution}')
        return True
    return False

# Now let's mine!

def mine_simple(max_nonce, prev_hash, transactions):
    print('\nMining...')
    # Note: max_nonce is just used for demonstration purposes to avoid an endless
    # loop
    nonce = 0 # Initalized to zero -- Is this true in Bitcoin?
    while (solve_hash_puzzle(nonce, prev_hash, transactions) == False
            and nonce < max_nonce):
        nonce += 1
    print(f'Nonce that produced solution: {nonce}')

# Uncomment the code below to see the program run
"""
_max_nonce = 100000
prev_hash = hashlib.sha256('Satoshi Nakamoto'.encode('utf-8')).hexdigest()
transactions = ["tx1", "tx2", "tx3"]

mine_simple(_max_nonce, prev_hash, transactions)
"""

# Notice the nonce value tells us how many attempts were required to find a
# solution. This is also an approximation for difficulty.
