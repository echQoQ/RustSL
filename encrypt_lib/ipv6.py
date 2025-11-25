name = 'ipv6'
description = 'Encode binary as IPv6 addresses (comma separated) with leading sha256 and length'

import hashlib

def sha256_bytes(b):
	sha = hashlib.sha256()
	sha.update(b)
	return sha.digest()

def bytes_to_ipv6(b):
	parts = []
	for i in range(0, 16, 2):
		v = int.from_bytes(b[i:i+2], 'big')
		parts.append(f'{v:04X}')
	return ':'.join(parts)

def process(data, args):
    addresses = []
    for i in range(0, len(data), 16):
        addr_bytes = data[i:i+16]
        if len(addr_bytes) < 16:
            addr_bytes += b'\x00' * (16 - len(addr_bytes))
        addresses.append(bytes_to_ipv6(addr_bytes))
    hash1 = sha256_bytes(data)
    len_bytes = len(data).to_bytes(4, 'little')
    final = hash1 + len_bytes + ','.join(addresses).encode()
    return final
