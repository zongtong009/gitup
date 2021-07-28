# a = """\
# 'and' "five 'six'" and 'jix "core" in' an n\
# """
# b = "one"
# print(a)
# print(b, end="")
import random

b = [0, 0]
for i in range(100000):
    b[0] += random.randint(-1, 1)
    b[1] += random.randint(-1, 1)

print(b)
