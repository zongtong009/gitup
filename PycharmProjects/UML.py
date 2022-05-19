"""
    你是公司营销系统的工程师。
    你们某个商品的购买概率和补贴额的关系为，p(x) = 0.05 x + 0.2。
    该商品原价 m 为 16 元，成本价 c 为 8 元，求利润最大的补贴额应该是多少？
"""
from sympy import expand, symbols, plot_implicit, Eq
import math
from sympy.abc import x, y

p1 = plot_implicit(Eq(-0.0025 * x ** 2 + 0.38 * x + 1.56, y))
x = symbols('x')
print(expand(
    (16 - 8 - 0.05 * x - 0.2) * (0.05 * x + 0.2)
))


def getSubsidy(k, b, m, c):
    rx = [-k, k * (m - c) - b, b * (m - c)]
    # -系数   (m-c-kx-0.2)*(kx+0.2)
    # (16-8-0.05x-0.2)*(0.05x+0.2)
    rpx = [-2 * k, k * (m - c) - b]
    return -rpx[1] / rpx[0]

# print(getSubsidy(0.05, 0.2, 16, 8))
