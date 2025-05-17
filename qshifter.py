from qshifter import QuickShifter
from app import app
import argparse

RED = "\033[01;31m"
GREEN = "\033[01;32m"
YELLOW = "\033[01;33m"
BLUE = "\033[01;34m"

END = "\033[0m"


def color(string: str, color: str):
    return color + string + END


BANNER: str = color(
    """\
            _     _  ___
           | |   (_)/ __)_
  ____  ___| | _  _| |__| |_  ____  ____   v 0.1.0
 / _  |/___) || \\| |  __)  _)/ _  )/ ___)
| | | |___ | | | | | |  | |_( (/ /| |
 \\_|| (___/|_| |_|_|_|   \\___)____)_|
    |_|
""",
    BLUE,
)

PROMPT = color("qs> ", RED)


def interactive() -> bool:
    input_string: str = ""
    try:
        input_string = input(PROMPT)
        shifter = QuickShifter(input_string)
        shifter.show()
    except EOFError:
        return False
    except KeyboardInterrupt:
        return False
    return input_string != "exit"


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="将输入的字符串按单词循环移位")
    group = parser.add_mutually_exclusive_group()
    group.add_argument("-p", "--input", type=str, help="需要循环移位的字符串")
    group.add_argument(
        "-i", "--console", action="store_true", help="进入命令行交互模式"
    )
    group.add_argument(
        "-s",
        "--server",
        action="store_true",
        help="进入服务器模式，在网页中交互",
    )
    args = parser.parse_args()

    # TODO: 增加文件处理功能(-f)

    # TODO: 增加详细信息功能(-v)

    if args.input is not None:
        shifter = QuickShifter(args.input)
        shifter.show()
    elif args.console:
        print(BANNER)
        print(parser.description)
        print("请按CTRL+C/CTRL+D退出")
        while interactive():
            pass
    elif args.server:
        print("enter server mode")
        app.run()
