#!/usr/bin/python
# -*- coding:utf-8 -*-

import os

files = os.listdir(".") # 列出当前目录下所有的文件


for file in files:
    portion = os.path.splitext(file)
    # 更改exb文件名,GLLD改为GLLP
    if portion[1] == r".exb" and "GLLD" in portion[0]:  
    # 重新组合文件名和后缀名
        newname = "GLLP" + portion[0][4:] +".exb"
        os.rename(file,newname)
            