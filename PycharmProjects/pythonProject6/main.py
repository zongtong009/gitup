"""
虚拟币的量化
"""
a = 12.205
t = 1.007
for i in range(100):
    a = a / t
    # print("{:.3f}".format(a), end="\t")
    # if (i - 9) % 10 == 0:
    #     print()
    if a <= 7.654:
        # print("\n\n")
        for j in range(221):
            a = a * t
            print("{:.3f}".format(a), end="\t")
            if (j - 15) % 16 == 0:
                print()
                print(a, end="\t")
                print(a)
                pass
            print(a)
        break
