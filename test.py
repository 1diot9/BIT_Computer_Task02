from qshifter import QuickShifter

# TEST: 测试部分
if __name__ == "__main__":

    # TEST: 简单测试
    tst = QuickShifter("A simple test string")
    for i in tst:
        print(i)

    print(tst.string)
    print(tst.word_list)

    print(tst.shifts)
