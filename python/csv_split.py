from datetime import datetime
 
def Main():
    
    #target_dir = 'D:/aaa'

    flag = 0
    name = 1
    dataList = []
 
    print("开始时间")
    print(datetime.now().strftime('%Y-%m-%d %H:%M:%S'))
 
    with open('chi.csv','rb') as f0:
        for line in f0:
          flag+=1
          dataList.append(line)
          if flag == 10000: #切割为1W行一份的文件
              with open( "zhs_" + str(name) + ".csv",'wb+') as f_target:
                  for data in dataList:
                      f_target.write(data)
              name+=1
              flag = 0
              dataList = []
                
    # 处理最后一批行数少于1W行的
    with open("zhs_" + str(name) + ".csv",'wb+') as f_target:
        for data in dataList:
            f_target.write(data)
 
    print("完成时间")
    print(datetime.now().strftime('%Y-%m-%d %H:%M:%S'))
 
if __name__ == "__main__":
    Main()
