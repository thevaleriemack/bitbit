from bitcoin import *

priv = sha256('big long brainwallet pw')
print(priv)

"""
def privkey_to_pubkey(privkey):
    f = get_privkey_format(privkey)
    privkey = decode_privkey(privkey, f)
    if privkey >= N:
        raise Exception("Invalid privkey")
    if f in ['bin', 'bin_compressed', 'hex', 'hex_compressed', 'decimal']:
        return encode_pubkey(fast_multiply(G, privkey), f)
    else:
        return encode_pubkey(fast_multiply(G, privkey), f.replace('wif', 'hex'))
"""
pub = privtopub(priv)

print(pub)
addr = pubtoaddr(pub)
print(addr)

# Gets the transaction output history of a given set of addresses,
# including whether or not they have been spent
h = history(addr)
print(h)
