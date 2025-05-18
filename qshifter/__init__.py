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

        """
        循环右移之后异或排序大小写，只对26个英文字母大小写生效

        例：
        ord('a') == 0b0110_0001
        ord('A') == 0b0100_0001
        ord('b') == 0b0110_0010
        ord('B') == 0b0110_0010

        循环右移4位后：
        ror(ord('a'), 4) == 0b0001_0110
        ror(ord('A'), 4) == 0b0001_0100
        ror(ord('b'), 4) == 0b0010_0110
        ror(ord('B'), 4) == 0b0010_0100
        
        此时满足 A < a < B < b，排序结果是A a B b
        为此我们要转换一下同一个字母大小写的大小关系

        异或2之后：
        ror(ord('a'), 4) ^ 2 == 0b0001_0100
        ror(ord('A'), 4) ^ 2 == 0b0001_0110
        ror(ord('b'), 4) ^ 2 == 0b0010_0100
        ror(ord('B'), 4) ^ 2 == 0b0010_0110

        此时满足 a < A < b < B，符合题意
        """
        # WARN: 此举会破坏除拉丁字母之外的ASCII字符的大小关系
        self.shifts.sort(
            key=lambda x: (((ord(x[0]) << 4) | (ord(x[0]) >> 4)) & 0xFF) ^ 0x2
            if x[0].isalpha()
            else ord(x[0])
        )

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
