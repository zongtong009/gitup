from datetime import datetime
 
def Main():
        
    #本脚本只能在当前文件夹下操作文件和生成文件
    
    num_lines = 0
    name = 1
    dataList = []
 
    print("开始时间")
    print(datetime.now().strftime('%Y-%m-%d %H:%M:%S'))
 
    with open('case_data_index.json','rb') as f0:
        for line in f0:
          num_lines+=1
          dataList.append(line)         
          if num_lines == 10000: #切割为1万行一份的文件
              with open( "case_splt_" + str(name) + ".json",'wb+') as f_target:
                  for data in dataList:
                      f_target.write(data)
              name+=1
              num_lines = 0
              dataList = []

                
    # 处理最后一批行数少于1万行的
    with open("case_splt_" + str(name) + ".json",'wb+') as f_target:
        for data in dataList:
            f_target.write(data)
 
    print("完成时间")
    print(datetime.now().strftime('%Y-%m-%d %H:%M:%S'))
 
if __name__ == "__main__":
    Main()
