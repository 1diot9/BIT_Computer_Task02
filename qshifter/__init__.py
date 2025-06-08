from typing import Self
from collections import deque
from functools import cmp_to_key, cache
from string import ascii_letters
from color import *

MAGIC_LEN = 1000


class QuickShifter:
    """QuickShifter类
    通过迭代器来产生所有需要的序列

    :param string: 给定字符串
    """

    def __init__(self, string: str) -> None:
        self.string = string.strip()

        self.words = string.split(" ")

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
        def magic(x): return (((ord(x) << 3) | (ord(x) >> 5)) & 0xFF) ^ 0x1
        magic = magic if len(string) < MAGIC_LEN else cache(magic)

        def cmp(x, y) -> int:
            for x, y in zip(x, y):
                if x == y:
                    continue
                return magic(x) - magic(y)
            return 0

        self.shifts.sort(key=cmp_to_key(cmp))

    def __getitem__(self, index) -> str:
        return self.shifts[index]

    def __iter__(self):
        return QuickShifterIter(self.words)

    def __len__(self) -> int:
        return len(self.shifts)

    def show_all(self, verbose: bool = False):
        if self.string != "":
            if verbose:
                print(f"[{color(self.string, BOLD)}]:")
                for i, shift in enumerate(self.shifts):
                    print(f"{i + 1}: {shift}")
                print(f"\n共输出{self.__len__()}行移位序列")
            else:
                for shift in self.shifts:
                    print(shift)


class QuickShifterLines:
    """QuickShifterLines类
    用于处理多行输入的循环移位序列
    内部调用QuickShifer类进行处理

    若merge为True,仅会产生一个list

    :param str_list: 给定多行字符串序列
    :type str_list: list[str]

    :param merge: 是否合并输出（默认否）
    :type merge: bool = False
    """

    def __init__(self, str_list: list[str], merge: bool = False) -> None:
        self.lines = list(map(lambda x: x.strip(), str_list))
        self.merge = merge

        self.lshifts: list[list[str]] = []
        self.all_len: int = 0

        if merge:
            shifts = []
            for string in str_list:
                words = string.split(" ")
                shifts += [x for x in QuickShifterIter(words)]

            magic = cache(lambda x: (
                ((ord(x) << 3) | (ord(x) >> 5)) & 0xFF) ^ 0x1)

            def cmp(x, y) -> int:
                for x, y in zip(x, y):
                    if x == y:
                        continue
                    return magic(x) - magic(y)
                return 0

            shifts.sort(key=cmp_to_key(cmp))

            self.all_len = shifts.__len__()
            self.lshifts.append(shifts)
        else:
            for line in filter(lambda x:  x.__len__() != 0, self.lines):
                line_shifter = QuickShifter(line)
                self.all_len += line_shifter.__len__()
                self.lshifts.append(line_shifter.shifts)

    @classmethod
    def from_str(cls, string: str, merge: bool = False) -> Self:
        return cls(string.split("\n"), merge=merge)

    def __getitem__(self, index) -> str:
        return self.lshifts[index]

    def __iter__(self):
        return self.lshifts.__iter__()

    def __len__(self) -> int:
        return len(self.lshifts)

    def show(self, index: int, verbose: bool = False):
        """在merge值为True时，忽略index参数（暂定）"""
        if verbose:
            for i, shift in enumerate(self.lshifts[0 if self.merge else index]):
                print(f"{i + 1}: {shift}")
        else:
            for shift in self.lshifts[0 if self.merge else index]:
                print(shift)

    def show_all(self, verbose: bool = False):
        """显示所有移位序列
        :param verbose: 是否显示详细信息（默认否）
        :type verbose: bool = False
        """
        for line in range(self.__len__()):
            if verbose:
                print(
                    f"[{color(self.lines[line], BOLD) if not self.merge else "合并结果"}]:"
                )
                self.show(line, verbose=True)
                print()
            else:
                self.show(line)
        if verbose:
            print(f"共输出{self.all_len}行移位序列，处理{len(self.lines)}个字符串")


class QuickShifterIter:
    """QuickShifter[Lines]类专用迭代器
    不要在别的类调用/单独调用
    利用双端队列加速

    :param queue: 给定单词序列
    :type queue: list[str]
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
