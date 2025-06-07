from qshifter import QuickShifter, QuickShifterLines
from color import color, RED, CYAN, BLUE, YELLOW
# from rshifter import RapidShifter
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

NORMAL_PROMPT = color("qs> ", RED)
APPEND_PROMPT = color("ap> ", CYAN)


def interactive(verbose: bool) -> bool:
    input_string: str = ""
    lines: bool = False
    try:
        input_string = input(NORMAL_PROMPT).strip()
        while input_string.endswith("\\"):
            lines = True
            input_string = input_string[:-1]
            input_string += input(APPEND_PROMPT).strip()
        if lines:
            shifter = QuickShifterLines(input_string.split("\n"))
            shifter.show_all(verbose=verbose)
        else:
            shifter = QuickShifter(input_string)
            shifter.show(verbose=verbose)
    except (EOFError, KeyboardInterrupt):
        return False
    return input_string != "exit"


def parse_file(file_name: str, verbose: bool, merge: bool):
    try:
        with open(file_name, "r") as f:
            lines: list[str] = f.readlines()
            lines = [line.strip() for line in lines]
            shifter = QuickShifterLines(lines, merge=merge)
            shifter.show_all(verbose=verbose)
    except FileNotFoundError as e:
        print(e)
        exit(1)


def parse_string(string: str, verbose: bool, merge: bool):
    shifter = QuickShifterLines.from_str(string, merge=merge)
    shifter.show_all(verbose=verbose)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="将输入的字符串按单词循环移位")
    parser.add_argument("-V", "--version", action="store_true", help="显示版本信息")
    parser.add_argument("-m", "--merge", action="store_true", help="合并多行输入")
    parser.add_argument("-v", "--verbose", action="store_true", help="详细模式")

    group = parser.add_mutually_exclusive_group()
    group.add_argument("-p", "--input", type=str, help="需要循环移位的字符串")
    group.add_argument("-f", "--file", type=str, help="以文件格式输入")
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

    if args.version:
        print(BANNER)
        print("Copyright (c) 2025 软件工程小组. All Rights Reserved.")
        exit(0)

    merge = args.merge
    verbose = args.verbose

    if args.input is not None:
        parse_string(args.input, verbose=verbose, merge=merge)
    elif args.file:
        parse_file(args.file, verbose=verbose, merge=merge)
    elif args.console:
        print(BANNER)
        print(parser.description)
        print(f"请按 {color("CTRL+C/CTRL+D", YELLOW)} 退出")
        while interactive(verbose=verbose):
            pass
    elif args.server:
        from app import app

        print(BANNER)
        print("进入服务器模式")
        app.run()
