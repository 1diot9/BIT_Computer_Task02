class QuickShifter:
    """QuickShifter类
    通过迭代器来产生所有需要的序列

    :param string: 给定字符串
    """

    def __init__(self, string: str) -> None:
        self.string = string

        # TODO: word_list错误处理
        self.word_list = string.split(" ")

        self.shifts = [x for x in self]
        self.shifts.sort()

    def __getitem__(self, index) -> str:
        return self.shifts[index]

    def __iter__(self):
        return QuickShifterIter(self.word_list)

    def __len__(self) -> int:
        return len(self.shifts)

    def show(self):
        for shift in self.shifts:
            print(shift)

    # TODO: 更多相关方法(采用list初始化，多组字符串处理)


class QuickShifterIter:
    """QuickShifter类专用迭代器
    不要在别的类调用/单独调用

    :param queue: 给定单词序列
    """

    def __init__(self, queue: list[str]) -> None:
        self.queue = queue
        self.len = len(queue)
        self.count = 0

    def __iter__(self):
        return self

    def __next__(self) -> str:
        if self.count >= self.len:
            raise StopIteration
        self.queue.append(self.queue.pop(0))
        self.count += 1
        return " ".join(self.queue)
