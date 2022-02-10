
import os

files = os.listdir(".") # 列出当前目录下所有的文件


for file in files:
    portion = os.path.splitext(file)# 切片split
    if portion[-1] == ".pdf1":  
    # 重新组合文件名和后缀名
        newname = portion[0] +".pdf"
        os.rename(file,newname)