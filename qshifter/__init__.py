class QuickShifter:
    """QuickShifter类
    通过迭代器来产生所有需要的序列

    :param string: 给定字符串
    """

    def __init__(self, string: str) -> None:
        self.string = string

        self.word_list = string.split(" ")

        self.shifts = [x for x in self]

        """
        产生所有的移位序列
        循环左移之后异或排序大小写，只对26个英文字母大小写生效

        例：
        ord('a') == 0b0110_0001
        ord('A') == 0b0100_0001
        ord('b') == 0b0110_0010
        ord('B') == 0b0110_0010
        ord('p') == 0b0111_0000
        ord('P') == 0b0101_0000

        循环左移3位后：
        rol(ord('a'), 3) == 0b0000_1011
        rol(ord('A'), 3) == 0b0000_1010
        rol(ord('b'), 3) == 0b0001_0011
        rol(ord('B'), 3) == 0b0001_0010
        rol(ord('p'), 3) == 0b1000_0011
        rol(ord('P'), 3) == 0b1000_0010
        
        此时满足 A < a < B < b < P < p，排序结果是A a B b P p
        为此我们要转换一下同一个字母大小写的大小关系

        异或1之后：
        rol(ord('a'), 3) ^ 1 == 0b0000_1010
        rol(ord('A'), 3) ^ 1 == 0b0000_1011
        rol(ord('b'), 3) ^ 1 == 0b0001_0010
        rol(ord('B'), 3) ^ 1 == 0b0001_0011
        rol(ord('p'), 3) ^ 1 == 0b1000_0010
        rol(ord('P'), 3) ^ 1 == 0b1000_0011

        此时满足 a < A < b < B < p < P，符合题意
        """
        # NOTE: 如果仅循环移位4位，会导致字母p ~ z与a ~ o的大小关系出现问题
        # WARN: 此举会破坏除拉丁字母之外的ASCII字符的大小关系
        self.shifts.sort(
            key=lambda x: (((ord(x[0]) << 3) | (ord(x[0]) >> 5)) & 0xFF) ^ 0x1
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
