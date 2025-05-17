from qshifter import QuickShifter

TEST_IDX = 0


def test(func):
    # TODO: 改善检测
    def wrapper():
        print(f"TEST {TEST_IDX}")
        try:
            func()
        except AssertionError:
            print(f"TEST {TEST_IDX} failed")
            return
        print(f"TEST {TEST_IDX} pass")

    return wrapper


class QshifterTest:
    # TODO: 完善测试功能
    pass


@test
def test_test():
    assert 1 == 2


# TEST: 测试部分
if __name__ == "__main__":

    # TEST: 简单测试
    tst = QuickShifter("A simple test string")
    for i in tst:
        print(i)

    print(tst.string)
    print(tst.word_list)

    print(tst.shifts)

    test_test()
