
def Main():

    with open('chi.csv','rb') as f0:
        num=0
        dataList = []
        
        for line in f0.readlines():
            num += 1
            print(num)
            if "李鑫".encode("utf-8") in line:
                dataList.append(line)
                
        with open( "chi00.csv",'wb+') as f2:
            for data in dataList:
                f2.write(data)
            #s=str(num)+"李鑫"
            #f2.write(s.encode("utf-8"))
            #print(s,s,s)            


if __name__ == "__main__":
    Main()
    #input()