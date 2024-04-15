# 電卓
from math import prod, gcd, lcm, comb, perm, factorial, log10, log2
import functools

# === 総和・総積 ===
prod([1,2,3,4]) == 24
sum([1,2,3,4]) == 10


# === modint ===
# from math import pow をすると、組み込みの pow が上書きされて↓がエラーになるので注意
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

# === 素数/素因数分解 ===

# 素因数分解は linux の factor コマンドでもできる
from sympy import factorint, primerange, primepi, prime, primorial, totient
factorint(24) == {2: 3, 3: 1}
list(primerange(7, 18)) == [7, 11, 13, 17]
prime(3) == 5 # 3番目の素数
primepi(12) == 5 # 12以下の素数の数 (2, 3, 5, 7, 11)
primorial(3) == 30 # 2 * 3 * 5 =30 (素数階乗)
totient(6) == 2 # オイラーのトーシェント関数。[0, 6) のうち 6 と互いに素な数は2つ
