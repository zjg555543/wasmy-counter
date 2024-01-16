class Uint256:
    def __init__(self, value):
        self.value = value

# 生成1000条数据
data_list = []
for i in range(1, 1001):
    data_list.append(f'delta{i}:Uint256,')

# 打印生成的数据
for data in data_list:
    print(data)