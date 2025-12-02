name = 'ecc'
description = 'ECC-based encryption using ECDH key exchange and AES-GCM'

import os
import hashlib
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives.kdf.hkdf import HKDF
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

def sha256_bytes(b):
    sha = hashlib.sha256()
    sha.update(b)
    return sha.digest()

def process(data, args):
    # Generate ECC private key A
    priv_a = ec.generate_private_key(ec.SECP256R1())

    # Generate random ECC private key B
    priv_b = ec.generate_private_key(ec.SECP256R1())
    pub_b = priv_b.public_key()

    # Perform ECDH: shared_secret = priv_a.exchange(ec.ECDH(), pub_b)
    shared_secret = priv_a.exchange(ec.ECDH(), pub_b)

    # Derive AES key using HKDF
    hkdf = HKDF(
        algorithm=hashes.SHA256(),
        length=32,
        salt=None,
        info=b'',
    )
    aes_key = hkdf.derive(shared_secret)

    # Generate nonce
    nonce = os.urandom(12)  # AES-GCM uses 12-byte nonce

    # Encrypt data
    aesgcm = AESGCM(aes_key)
    ciphertext = aesgcm.encrypt(nonce, data, None)

    # Serialize keys
    priv_a_bytes = priv_a.private_numbers().private_value.to_bytes(32, 'big')
    pub_b_bytes = pub_b.public_bytes(
        encoding=serialization.Encoding.X962,
        format=serialization.PublicFormat.CompressedPoint
    )  # 33 bytes: 0x02/0x03 + x

    # Final output: priv_a (32) + pub_b (33) + nonce (12) + ciphertext
    final = priv_a_bytes + pub_b_bytes + nonce + ciphertext
    return final