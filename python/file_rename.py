#!/usr/bin/python
# -*- coding:utf-8 -*-
"""
不同文件夹下的文件操作
"""

import os


# 列出目标文件夹下所有的文件
files = os.listdir(r'./aed')

for filename in files:
    portion = os.path.splitext(filename)

    # 如果后缀是.txt     "abc.txt"会得到['abc','.txt']
    if portion[1] == r".txt":
        
       
        newname = portion[0] + ".rmvb"      # 重新组合文件名和后缀名
        os.chdir("./aed")                   #切换文件路径,如无路径则要新建或者路径同上,做好备份
        os.rename(filename,newname)         # 重命名
