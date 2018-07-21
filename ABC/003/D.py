MOD = 1000000007
C = 30

def nCm_pascal():
    pascal = [[0 for _ in range(C*C)] for _ in range(C*C)]
    for i in range(1, C * C):
        pascal[0][0] = 1
        for j in range(0, C * C):
            if i == j and j == 0:
                pascal[i][j] = 1
            else:
                pascal[i][j] = (pascal[i-1][j] % MOD + pascal[i-1][j-1] % MOD) % MOD
    return pascal


def main():
    r, c = list(map(int, input().split(' ')))
    x, y = list(map(int, input().split(' ')))
    d, l = list(map(int, input().split(' ')))
    pascal = nCm_pascal()
    print(pascal[x*y][d] * (r - x + 1) * (c - y + 1) % MOD)

if __name__ == '__main__':
    main()
