name = 'aes'
description = 'AES encrypt with random 32-byte key and 16-byte IV, output [key(32)][iv(16)][sha256(orig)][encrypted]'

import os
import hashlib

def sha256_bytes(b):
    sha = hashlib.sha256()
    sha.update(b)
    return sha.digest()

def process(data, args):
    try:
        from Crypto.Cipher import AES
        from Crypto.Util.Padding import pad
    except Exception:
        raise RuntimeError('pycryptodome is required for aes plugin')
    
    # generate key and iv
    key = os.urandom(32)
    iv = os.urandom(16)
    
    padded_data = pad(data, AES.block_size)    
    cipher = AES.new(key, AES.MODE_CBC, iv)
    encrypted = cipher.encrypt(padded_data)
    hash1 = sha256_bytes(data)
    final = key + iv + hash1 + encrypted
    return final