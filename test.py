def i64_mul(a: int, b: int) -> int:
    print(f"i64_mul({a=}, {b=})")
    negative = False
    if a < 0:
        negative = not negative
        a = -a
    if b < 0:
        negative = not negative
        b = -b

    a_low = a & 0xFFFFFFFF
    a_high = a >> 32
    b_low = b & 0xFFFFFFFF
    b_high = b >> 32

    print(f"{a=}, {b=}, {a_low=}, {a_high=}, {b_low=}, {b_high=}")

    res = a_low * b_low
    res += (a_low * b_high) << 32
    res += (a_high * b_low) << 32

    return res

print(i64_mul(1286748865,1454381721) * 1000000)
print("64 bit integer limit:", 2**63 - 1)

