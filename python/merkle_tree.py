import hashlib


def generateMerkleRoot(items):
    if (len(items) % 2) != 0:
        items.append(items[-1])
    index = 0
    for n in items:
        try:
            m = hashlib.md5()
            m.update(n)
            items[index] = m.digest()
            index += 1
        except TypeError as e:
            print("Invalid argument:", e)
            return
    while len(items) > 1:
        for k in range(len(items) // 2):
            items.insert(k, hashTogether(items.pop(k), items.pop(k)))
    return items


def hashTogether(a, b):
    m = hashlib.md5()
    m.update(a)
    m.update(b)
    return m.digest()
