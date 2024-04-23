def solve(n, m, k):
    return ((1 << n) % ((1 << m) - (1 << k))) % 10


def solve1(n, m, k):
    return ((1 << n) % ((1 << m) - (1 << k)))

# 1111100000

# 9876543210
# 1100000000
# 1がm-k個
# 0がk個


for i in range(0,15):
    print((2**i) % 31)


n=123
m=84
k=50
(n-k)%(m-k) + k


