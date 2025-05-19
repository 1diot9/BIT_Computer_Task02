from typing import Self
from collections import deque


class QuickShifter:
    """QuickShifter类
    通过迭代器来产生所有需要的序列

    :param string: 给定字符串
    """

    def __init__(self, string: str) -> None:
        self.string = string

        self.word_list = string.split(" ")

        self.shifts: list[str] = [x for x in self]

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
            key=lambda y: list(
                map(
                    lambda x: (((ord(x) << 3) | (ord(x) >> 5)) & 0xFF) ^ 0x1
                    if x.isalpha()
                    else ord(x),
                    y,
                )
            )
        )

    def __getitem__(self, index) -> str:
        return self.shifts[index]

    def __iter__(self):
        return QuickShifterIter(self.word_list)

    def __len__(self) -> int:
        return len(self.shifts)

    #! TODO: 详细模式输出/统计
    def show(self):
        for shift in self.shifts:
            print(shift)


class QuickShifterLines:
    """QuickShifterLines类
    用于处理多行输入的循环移位序列
    内部调用QuickShifer类进行处理

    :param str_list: 给定多行字符串序列
    :type str_list: list[str]
    """

    def __init__(self, str_list: list[str]) -> None:
        self.lines = str_list

        self.line_shifts: list[list[str]] = []

        for line in self.lines:
            line_qshifter = QuickShifter(line)
            self.line_shifts.append(line_qshifter.shifts)

    @classmethod
    def from_str(cls, string: str) -> Self:
        return cls(string.split("\n"))

    def __getitem__(self, index) -> str:
        return self.line_shifts[index]

    def __iter__(self):
        return self.line_shifts.__iter__()

    def __len__(self) -> int:
        return len(self.line_shifts)

    #! TODO: 详细模式输出/统计
    def show(self, index: int):
        for shift in self.line_shifts[index]:
            print(shift)

    def show_all(self):
        for line in range(self.__len__()):
            self.show(line)
            print()


class QuickShifterIter:
    """QuickShifter类专用迭代器
    不要在别的类调用/单独调用

    :param queue: 给定单词序列
    """

    def __init__(self, queue: list[str]) -> None:
        self.queue = deque(queue)
        self.len = len(queue)
        self.count = 0

    def __iter__(self):
        return self

    def __next__(self) -> str:
        if self.count >= self.len:
            raise StopIteration
        self.queue.append(self.queue.popleft())
        self.count += 1
        return " ".join(self.queue)
