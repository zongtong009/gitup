read()      读取所有文字，类似这种‘abcdefg\nhijklmn’
readline()  读一行类似这种‘abcdefg\n’
readlines() 读所有行，类似[‘abcdefg’,'hijklmn']

多行文本本身每行自带回车，print也默认回车
打印多行后，每行之间会有空行

sum(map(lambda x:int(x),unicode(1234)))
各位相加
