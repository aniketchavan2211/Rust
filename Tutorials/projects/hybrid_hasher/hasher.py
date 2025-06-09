import ctypes
import time

# Load the shared libraries
rust = ctypes.CDLL('./librust_hash.so')
c = ctypes.CDLL('./libc_hash.so')

# Define argument and return types for Rust function
rust.rust_hash_string.argtypes = [ctypes.c_char_p]
rust.rust_hash_string.restype = ctypes.c_char_p

# Define argument types for C function
c.c_hash_string.argtypes = [ctypes.c_char_p, ctypes.c_char_p]

# Rust function wrapper
def hash_with_rust(s):
    result = rust.rust_hash_string(s.encode())
    return ctypes.cast(result, ctypes.c_char_p).value.decode()

# C function wrapper
def hash_with_c(s):
    out = ctypes.create_string_buffer(65)  # SHA256 produces 64 chars in hex + null-terminator
    c.c_hash_string(s.encode(), out)
    return out.value.decode()

# Read inputs from file and hash them
with open("input.txt", "r") as f:
    lines = [line.strip() for line in f if line.strip()]

with open("hashes.txt", "w") as f:
    for line in lines:
        # Time C
        t1 = time.perf_counter()
        c_hash = hash_with_c(line)
        t2 = time.perf_counter()

        # Time Rust
        t3 = time.perf_counter()
        rust_hash = hash_with_rust(line)
        t4 = time.perf_counter()

        f.write(f"{line}:\n")
        f.write(f"  C    : {c_hash}\n")
        f.write(f"  Time : {t2 - t1:.6f} sec\n")
        f.write(f"  Rust : {rust_hash}\n")
        f.write(f"  Time : {t4 - t3:.6f} sec\n\n")
