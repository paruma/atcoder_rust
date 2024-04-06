# 電卓
from math import prod, gcd, lcm, comb, perm, factorial
import functools

# === 総和・総積 ===
prod([1,2,3,4]) == 24
sum([1,2,3,4]) == 10


# === modint ===
# 2^{-1}
pow(2, -1, 998244353) == 499122177


# 有理数復元
mod = 998244353
# mod = 1000000007

def f(x: int) -> int:
    """x を有理数に復元する"""
    for denom in range(1, 1000): # 分子
        denom_inv = pow(denom, -1, mod)
        for numer in range(-1000, 1000): # 分母
            if x == numer * denom_inv % mod:
                if denom_inv == 1:
                    return f"{numer}"
                else:
                    return f"{numer}/{denom}"
    return 'Not Found'


# === gcd/ lcm ===

# gcd, lcm は複数引数に対応している
lcm(*range(1,10)) == 2520 # lcm(1, 2,..., 9)


# === reduce の使い方 === 

functools.reduce(lambda x, y : x + y, [1, 2, 3, 4]) == 10
