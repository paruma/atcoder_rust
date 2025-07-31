# 電卓

# TAB補完の有効化
import rlcompleter
import readline
readline.parse_and_bind('tab: complete')


from math import prod, gcd, lcm, comb, perm, factorial, log10, log2
import functools

# === 文字列処理 ===
assert sorted([int(x) for x in '7 11 3 9'.split()]) == [3, 7, 9, 11]

# === 総和・総積 ===
assert prod([1, 2, 3, 4]) == 24
assert sum([1, 2, 3, 4]) == 10


# === modint ===
# from math import pow をすると、組み込みの pow が上書きされて↓がエラーになるので注意
# 2^{-1}
assert pow(2, -1, 998244353) == 499122177

# 有理数復元
mod = 998244353
# mod = 1000000007


def f(x: int) -> int:
    """x を有理数に復元する"""
    for denom in range(1, 1000):  # 分子
        denom_inv = pow(denom, -1, mod)
        for numer in range(-1000, 1000):  # 分母
            if x == numer * denom_inv % mod:
                if denom_inv == 1:
                    return f"{numer}"
                else:
                    return f"{numer}/{denom}"
    return "Not Found"

# x ∈ (ℤ/ mod ℤ)^× の位数の計算
def mul_order_of_mod(x: int, mod: int) -> int:
    from sympy import totient, divisors

    assert(gcd(x, mod) == 1) # x ∈ (ℤ/ mod ℤ)^×
    for d in divisors(totient(mod)):
        if pow(x, d, mod) == 1:
            return d

    assert False

assert mul_order_of_mod(2, 998244353) == 499122176
assert mul_order_of_mod(3, 998244353) == 998244352
assert mul_order_of_mod(10, 998244353) == 998244352
assert mul_order_of_mod(17, 998244353) == 557056

# === gcd/ lcm ===

# gcd, lcm は複数引数に対応している
assert lcm(*range(1, 10)) == 2520  # lcm(1, 2,..., 9)


# === reduce の使い方 ===

assert functools.reduce(lambda x, y: x + y, [1, 2, 3, 4]) == 10

# === 素数/素因数分解 ===

# 素因数分解は linux の factor コマンドでもできる
from sympy import factorint, primerange, primepi, prime, primorial, totient, divisors

assert factorint(24) == {2: 3, 3: 1}
assert list(primerange(7, 18)) == [7, 11, 13, 17]
assert prime(3) == 5  # 3番目の素数
assert primepi(12) == 5  # 12以下の素数の数 (2, 3, 5, 7, 11)
assert primorial(3) == 30  # 2 * 3 * 5 =30 (素数階乗)
assert (
    totient(6) == 2
)  # オイラーのトーシェント関数。[0, 6) のうち 6 と互いに素な数は2つ

# === ランダム ===

import random

random.uniform(100, 200)  # 100 から 200 の float の一様乱数
random.randrange(10)  # 0 から 9 の整数の一様乱数
random.randrange(10, 20)  # 10 から 19 の整数の一様乱数
random.randrange(10, 20, 2)  # step=2
# xs = [1, 2, 3]
# random.shuffle(xs) # inplace なシャッフル
random.random() < 0.3  # bool をランダムに生成 (30%でtrue)

# === 写像12相 ===
assert perm(5, 3) == 60
assert comb(5, 3) == 10

# 以下の関数の計算は速くはないので注意


def num_surj(ball: int, box: int) -> int:
    return sum((-1) ** (box - i) * comb(box, i) * i**ball for i in range(0, box + 1))


def stirling_s2(ball: int, box: int) -> int:
    return sum(
        (-1) ** (box - i) * comb(box, i) * i**ball for i in range(0, box + 1)
    ) // factorial(box)


def bell(ball: int, box: int) -> int:
    return sum(stirling_s2(ball, i) for i in range(0, box + 1))


def partition(ball: int, box: int) -> int:
    if ball == 0:
        return 1
    if box == 0:
        return 0
    if ball < box:
        return partition(ball, box - 1)
    return partition(ball, box - 1) + partition(ball - box, box)


assert num_surj(5, 3) == 150
assert stirling_s2(5, 3) == 25
assert bell(5, 3) == 41
assert partition(5, 3) == 5
