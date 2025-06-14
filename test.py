from qshifter import QuickShifter, QuickShifterLines
from rshifter import RapidShifter, RapidShifterLines
from color import color, RED, GREEN
from functools import wraps
from pyinstrument import Profiler

# TODO: 更多的测试


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
            self.push(func)

    def push(self, func):
        """增加测试函数"""
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
        print(
            f"running {passed + failed} tests: {passed} passed, {failed} failed")

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
    def test_sort_blank(self):
        tst = QuickShifter("a aA ap aP aaa")
        res = [
            "a aA ap aP aaa",
            "aaa a aA ap aP",
            "aA ap aP aaa a",
            "ap aP aaa a aA",
            "aP aaa a aA ap",
        ]
        assert tst.shifts == res


@test
def test_sort_2():
    tst = QuickShifter("aa aA ab aB ap aP")
    res = [
        "aa aA ab aB ap aP",
        "aA ab aB ap aP aa",
        "ab aB ap aP aa aA",
        "aB ap aP aa aA ab",
        "ap aP aa aA ab aB",
        "aP aa aA ab aB ap",
    ]
    assert tst.shifts == res


@test
def test_lines():
    tst = QuickShifterLines(
        ["A a B b", "Another yet new string",
            "Once upon a time", "It is my shift now"],
    )

    res = [
        "a B b A",
        "A a B b",
        "b A a B",
        "B b A a",
    ]
    assert tst[0] == res


@test
def test_bigdata():
    import random
    import string

    # 生成一个100_000字符长的随机字符串压力测试
    random_str = "".join(
        random.choice(string.ascii_letters + " ") for _ in range(100_000)
    )
    # print(random_str)
    QuickShifter(random_str)
    # rs = RapidShifter(random_str)
    # rs.process()
    # _ = rs[0]
    # RapidShifter(random_str).qshifts()


@test
def test_bigtimes():
    import random
    import string

    # 生成一个100_000字符长的随机字符串压力测试
    for _ in range(100):
        random_str = "".join(
            random.choice(string.ascii_letters + " ") for _ in range(10000)
        )
    # print(random_str)
        QuickShifter(random_str)
        RapidShifter(random_str).process()
        # RapidShifter(random_str).qshifts(16)


@test
def test_biglist():
    import random
    import string

    random_str = []
    # 生成一个100_000字符长的随机字符串压力测试
    for _ in range(1000):
        random_str += ["".join(
            random.choice(string.ascii_letters + " ") for _ in range(5000)
        )]

    QuickShifterLines(random_str, merge=True)
    RapidShifterLines(random_str).process()


@test
def test_sometext():

    tst = QuickShifterLines(
        [
            "Lorem ipsum dolor sit amet consectetur adipiscing elit",
            "Sed facilisis gravida turpis id iaculis libero sollicitudin vel",
            "Etiam gravida justo sit amet ipsum tincidunt, sed rutrum ante pulvinar",
            "Sed eget quam nec risus consequat faucibus",
            "Aliquam id dui placerat consequat mauris non efficitur erat",
            "Curabitur ullamcorper a quam sed luctus",
            "Sed quis tempus elit",
            "Aenean tincidunt lacus ut condimentum vehicula nunc leo elementum odio a vehicula metus urna eu massa",
            "Suspendisse a iaculis quam",
            "Curabitur lacinia ligula facilisis congue volutpat diam felis rutrum quam",
            "et facilisis ante massa sed risus",
        ]
        * 100,
        merge=True,
    )
    assert tst.all_len == 90 * 100


@test
def test_search():
    tst2 = RapidShifter("Aspera Pipe process Zenic Brute http://www.baidu.com")
    # tst2.show_all(verbose=True)
    # print(tst2.search("www", all=True))
    # print(tst2.search("www"))

    assert (tst2.search("www", all=True) == [0, 1, 2, 3, 4])
    assert (tst2.search("www") is None)


@test
def test_search2():
    tst2 = RapidShifter("Aspera Pipe process Zenic Brute http://www.baidu.com")
    tst2.show_all(verbose=True)
    shifts = tst2.shifts()

    matches = tst2.search("Pipe ")

    # print(matches)
    # print([shifts[x] for x in matches])

    res = [shifts[x] for x in matches]

    assert (matches == [0, 1, 3, 4])
    assert (res[0] == 'Aspera Pipe process Zenic Brute')


@test
def test_rapidshifter():
    tst1 = RapidShifterLines(
        ["A a B p P b Z z http://www.google.com",
         "a A p p b B a W R P https://127.0.0.1",
         "A simple test sentence with no https",])
    # tst1.show_all(verbose=True)

    assert ("A simple test sentence with no https <None>" == tst1[5])


def analyze(browser: bool = False):
    """装饰器analyze
    用于进行性能分析
    用法：
    ```
    @analyze(browser=True)
    def func():
        pass
    ```
    :param func: 性能分析函数
    :param browser: 是否用浏览器显示结果
    :type browser: bool
    """

    def analyze_func(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            profiler = Profiler()
            profiler.start()

            func(*args, **kwargs)

            profiler.stop()
            if browser:
                profiler.open_in_browser()
            else:
                profiler.print()

        return wrapper
    return analyze_func


@analyze(browser=False)
def main():
    # TEST: 运行所有测试
    qshifer_test = QshifterTest(
        [test_sort_2, test_lines, test_search,
            test_sometext, test_rapidshifter]
    )
    qshifer_test.push(test_biglist)
    qshifer_test.push(test_bigdata)
    qshifer_test.push(test_bigtimes)
    '''
    qshifer_test = QshifterTest()

    qshifer_test.push(test_bigdata)
    '''

    qshifer_test.run_all_tests()


# TEST: 测试部分
if __name__ == "__main__":
    main()
