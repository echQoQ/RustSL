name = 'rc4'
description = 'RC4 encrypt with random 32-byte key, output [key(32)][sha256(orig)][encrypted]'

import os
import hashlib

def sha256_bytes(b):
	sha = hashlib.sha256()
	sha.update(b)
	return sha.digest()


def process(data, args):
    try:
        from Crypto.Cipher import ARC4
    except Exception:
        raise RuntimeError('pycryptodome is required for rc4 plugin')
    key = os.urandom(32)
    cipher = ARC4.new(key)
    encrypted = cipher.encrypt(data)
    hash1 = sha256_bytes(data)
    final = key + hash1 + encrypted
    return final
