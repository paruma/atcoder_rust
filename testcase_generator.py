import random

def main():
    # --- ここを編集 --- #
    sample_N_and_AB_pairs(3)


# ------------------------------------------------------------------------------
# サンプル関数群
# ------------------------------------------------------------------------------

def sample_N_and_A(N):
    """
    N
    A1 A2 ... AN
    """
    print(N)
    A = [random.randint(1, 10**5) for _ in range(N)]
    print(*A)


def sample_grid(H, W):
    """
    H W
    S11S12...S1W
    ...
    SH1SH2...SHW
    """
    print(H, W)
    p = 0.3 # 各マスで '#' が生成される確率
    for _ in range(H):
        row = random.choices([".", "#"], weights=[1 - p, p], k=W)
        print("".join(row))


def sample_N_and_AB_pairs(N):
    """
    N
    A1 B1
    A2 B2
    ...
    AN BN
    """
    print(N)
    for _ in range(N):
        A = random.randint(1, 10**5)
        B = random.randint(1, 10**5)
        print(A, B)


def sample_grid_int(H, W):
    """
    H W
    A11 A12 ... A1W
    ...
    AH1 AH2 ... AHW
    """
    print(H, W)
    for _ in range(H):
        row = [random.randint(1, 10**5) for _ in range(W)]
        print(*row)


if __name__ == "__main__":
    main()