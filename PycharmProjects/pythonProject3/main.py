import math
import itertools


def gauss(n):
    return n * (n + 1) / 2


def c_base(x, y):
    if x != y * 2:
        return False
    _sum = 0
    for i in range(y + 1):
        _sum += gauss(i)
    return _sum


def c2_1(x: int, y: int):
    if x < y:
        return False
    if x == y:
        return 1
    y = min(x - y, y)
    if y == 1:
        return x
    if y == 2:
        return x * (x - 1) / 2
    # result = 0
    # for i in range(1, x - y + 1 + 1):
    #     for j in range(1, y - 1 + 1):
    #         result += 0
    #         return 1
    top = math.factorial(x)
    bot1 = math.factorial(y)
    bot2 = math.factorial(x - y)
    return top / (bot1 * bot2)


def a2_1(x: int, y: int):
    if x < y:
        return False
    top = math.factorial(x)
    bot = math.factorial(y)
    return top / bot


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    for i in itertools.combinations('12345678', 4):
        print(''.join(i), end=" ")
    # print(c2_1(7, 3))
    # print(a2_1(5, 2))
