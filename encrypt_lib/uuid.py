name = 'uuid'
description = 'Interpret binary as sequence of UUIDs (16-byte blocks) with leading sha256 and length'

import uuid
import hashlib

def sha256_bytes(b):
	sha = hashlib.sha256()
	sha.update(b)
	return sha.digest()


def process(data, args):
    # pad to 16-byte multiple
    pad_len = (16 - (len(data) % 16)) % 16
    if pad_len:
        data = data + (b'\x00' * pad_len)
    uuids = []
    for i in range(0, len(data), 16):
        block = data[i:i+16]
        u = uuid.UUID(bytes=block)
        uuids.append(str(u))
    hash1 = sha256_bytes(data)
    len_bytes = len(data).to_bytes(4, 'little')
    final = hash1 + len_bytes + ','.join(uuids).encode()
    return final
