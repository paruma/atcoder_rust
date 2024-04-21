from functools import lru_cache

import sys
sys.setrecursionlimit(10**6) # 今回はなくても大丈夫

s = set()
def hoge(x):
    if x in s:
        return
    s.add(x)
    if x == 0:
        return
    hoge(x//2)
    hoge(x//3)
    hoge(x//4)
    hoge(x//5)
    hoge(x//6)

hoge(10**18)

# print(s)
print(len(list(s))) # 7254
