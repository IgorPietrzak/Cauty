def to_sparse6(n, edges):
    # Sparse6 header
    out = ":"
    # Encode n (simplified, assumes n < 63 for single byte)
    if n <= 62:
        out += chr(63 + n)
    else:
        # Multi-byte encoding for larger n
        bits = bin(n)[2:]
        k = len(bits)
        out += chr(63 + k)
        out += chr(63 + int(bits, 2))
    
    # Encode edges
    for u, v in edges:
        out += chr(63 + u) + chr(63 + v)
    return out

# P_25: 25 vertices, edges 0-1, 1-2, ..., 23-24
n = 25
edges = [(i, i+1) for i in range(n-1)]
sparse6 = to_sparse6(n, edges)
print(sparse6)
