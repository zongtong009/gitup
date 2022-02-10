#!/usr/bin/python
# -*- coding:utf-8 -*-

import os
 
# outer_dir = './aaa'
# outer_file='./1.txt'


'''
with open(outer_file) as ofile:
    m=ofile.readlines()
    for i in m :
        print(i,end='')
print(input('\n'))'''


# 列出当前目录下所有的文件
files = os.listdir(".")

for file in files:
    portion = os.path.splitext(file)
    if portion[1] != ".py":  
    # 重新组合文件名和后缀名
        newname = portion[0] +".pdf"
        os.rename(file,newname)
            
