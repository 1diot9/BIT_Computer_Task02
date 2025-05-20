from qshifter import QuickShifter
from color import *
import argparse


VERSION = "0.1.0"

BANNER: str = color(
    f"""\
            _     _  ___
           | |   (_)/ __)_
  ____  ___| | _  _| |__| |_  ____  ____   v {VERSION}
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
    parser.add_argument("-V", "--version", action="store_true", help="显示版本信息")

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

    if args.version:
        print(BANNER)
        print("Copyright (c) 2025 软件工程小组. All Rights Reserved.")
        exit(0)

    if args.input is not None:
        shifter = QuickShifter(args.input)
        shifter.show()
    elif args.console:
        print(BANNER)
        print(parser.description)
        print(f"请按 {color("CTRL+C/CTRL+D", YELLOW)} 退出")
        while interactive():
            pass
    elif args.server:
        from app import app

        print(BANNER)
        print("进入服务器模式")
        app.run()
