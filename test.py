from qshifter import QuickShifter
from color import *
from functools import wraps

import qshifter


def test(func):
    """装饰器test
    用于统计测试信息
    用法：
    ```
    @test
    def func():
        pass
    ```
    :param func: 测试函数
    """

    @wraps(func)
    def wrapper(*args, **kwargs) -> bool:
        name = func.__name__
        try:
            func(*args, **kwargs)
        except AssertionError:
            print(f"test {name}: ... {color("failed", RED)}")
            return False
        else:
            print(f"test {name}: ... {color("passed", GREEN)}")
            return True

    return wrapper


class QshifterTest:
    """QShifterTest类
    运行所有测试函数，并统计通过/失败信息
    测试函数可以写在类内部，也可以从外部导入

    注意：测试函数函数名一定要以test开头，并且加上`@test`装饰器
    否则会无法被识别/统计（名称不要重复）

    :param func_list: 类外部测试函数序列，可以为空，默认为空
    :type func_list: list | None
    """

    def __init__(self, func_list: list | None = None) -> None:

        if func_list is None:
            return

        # 通过反射导入外部函数
        for func in func_list:
            if callable(func) and getattr(self, func.__name__, None) is None:
                self.__setattr__(func.__name__, func)

    def run_all_tests(self):
        """通过反射运行所有测试函数并统计信息"""
        passed = 0
        failed = 0
        for test in self.__dir__():
            if test.startswith("test"):
                method = self.__getattribute__(test)
                if callable(method):
                    if method():
                        passed += 1
                    else:
                        failed += 1
        print(f"running {passed + failed} tests: {passed} passed, {failed} failed")

    @test
    def test_test(self):
        assert 1 == 2

    @test
    def test_simple(self):
        tst = QuickShifter("A simple test string")
        res = [
            "A simple test string",
            "simple test string A",
            "string A simple test",
            "test string A simple",
        ]
        assert tst.shifts == res

    @test
    def test_sort(self):
        tst = QuickShifter("a A b B p P")
        res = [
            "a A b B p P",
            "A b B p P a",
            "b B p P a A",
            "B p P a A b",
            "p P a A b B",
            "P a A b B p",
        ]
        assert tst.shifts == res


@test
def test_outtest():
    assert 2 + 2 == 4


# TEST: 测试部分
if __name__ == "__main__":
    tst = qshifter.QuickShifterLines(
        ["A a B b", "Another yet new string", "Once upon a time", "It is my shift now"]
    )
    tst.show_all()

    # TEST: 运行所有测试
    qshifer_test = QshifterTest([test_outtest])
    qshifer_test.run_all_tests()
