import hashlib

def hash_data(data):
    out = []

    for set in data:
        m = hashlib.sha256()
        m.update(data)
        out.append(m.hexdigest())

    return out
