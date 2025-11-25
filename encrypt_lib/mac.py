name = 'mac'
description = 'Encode binary as MAC addresses (comma separated) with leading sha256 and length'

import hashlib

def sha256_bytes(b):
	sha = hashlib.sha256()
	sha.update(b)
	return sha.digest()

def bytes_to_mac(b):
	return '-'.join(f'{x:02X}' for x in b)

def process(data, args):
    addresses = []
    for i in range(0, len(data), 6):
        mac_bytes = data[i:i+6]
        if len(mac_bytes) < 6:
            mac_bytes += b'\x00' * (6 - len(mac_bytes))
        addresses.append(bytes_to_mac(mac_bytes))
    hash1 = sha256_bytes(data)
    len_bytes = len(data).to_bytes(4, 'little')
    final = hash1 + len_bytes + ','.join(addresses).encode()
    return final
