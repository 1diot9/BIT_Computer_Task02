from qshifter import QuickShifter, QuickShifterLines
from color import color, RED, CYAN, BLUE, YELLOW, GREEN, RUST, PYTHON
from rshifter import RapidShifter, RapidShifterLines
from typing import Literal
import argparse


VERSION = "0.2.3"

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

NORMAL_PROMPT = color("cmd> ", GREEN)

RUST_PROMPT = color("rs> ", RUST)
PY_PROMPT = color("qs> ", PYTHON)

SEARCH_PROMPT = color("search> ", CYAN)
REGEX_PROMPT = color("regex> ", RED)

OK = color("[+]", GREEN)
INFO = color("[*]", BLUE)
ERROR = color("[-]", RED)


def process(input_string: str,
            lines: bool,
            verbose: bool = False,
            merge: bool = False,
            backend: Literal["rust", "python"] = "python"):
    if lines:
        input_lines = input_string.split("\n")
        if backend == "rust":
            shifter = RapidShifterLines(input_lines)
            shifter.show_all(verbose=verbose)
        else:
            shifter = QuickShifterLines(input_lines)
            shifter.show_all(verbose=verbose, merge=merge)

    else:
        if backend == "rust":
            shifter = RapidShifter(input_string)
        else:
            shifter = QuickShifter(input_string)

        shifter.show_all(verbose=verbose)

    return shifter


def console():
    parser = argparse.ArgumentParser(
        prog=NORMAL_PROMPT,
        description="将输入的字符串按单词循环移位", exit_on_error=False, add_help=False)
    parser.add_argument(
        "version", action="store_true", help="显示版本信息")

    subparsers = parser.add_subparsers(title="命令列表", dest="subcommand")
    subparsers.add_parser("exit", help="退出")

    help = subparsers.add_parser("help", help="显示帮助")
    help.add_argument("command", type=str, nargs='?',
                      default="", help="需要帮助的命令")
    # TODO: 增加不同命令的帮助界面

    rust = subparsers.add_parser("rs", help="使用rust后端循环移位（支持搜索）")
    rust.add_argument("-v", "--verbose", action="store_true", help="详细模式")
    rust.add_argument("input", type=str, nargs='*', help="输入字符串")

    python = subparsers.add_parser("qs", help="使用python后端循环移位")
    python.add_argument("-v", "--verbose", action="store_true", help="详细模式")
    python.add_argument("-m", "--merge", action="store_true", help="合并多行输入")
    python.add_argument("input", type=str, nargs='*', help="输入字符串")

    search = subparsers.add_parser("search", help="搜索上一次循环移位结果")
    search.add_argument("-r", "--regex", action="store_false", help="使用正则匹配")
    search.add_argument("-a", "--all", action="store_true", help="搜索包括网址URL")
    search.add_argument("pat", type=str, nargs='*', help="查找字符串")

    print(BANNER)
    print(parser.description)
    print(f"请按输入 {color("exit", YELLOW)} 退出")
    print(f"或输入 {color("CTRL+C/CTRL+D", YELLOW)} 强制退出")

    latest = None
    ret = True
    while ret:
        ret, latest = interactive(parser=parser, lastest=latest)


def interactive(parser, lastest: object) -> (bool, object):
    input_string: str = ""
    backend: str = ""
    shifter = lines = False
    verbose = merge = False

    try:
        inputs = input(NORMAL_PROMPT).strip()

        args = parser.parse_args(inputs.split())

        match args.subcommand:
            case "help":
                parser.print_help()
            case "rs":
                shifter = True
                backend = "rust"

                verbose = args.verbose
            case "qs":
                shifter = True
                backend = "python"

                verbose = args.verbose
                merge = args.merge
            case "search":
                if isinstance(lastest, (RapidShifter, RapidShifterLines)):
                    result = []
                    pat = re = ""
                    param = " ".join(args.pat)
                    use_interact = args.pat.__len__() == 0
                    if args.regex:
                        pat = input(SEARCH_PROMPT).strip(
                        ) if use_interact else param
                        result = lastest.search(pat=pat, all=args.all)
                    else:
                        re = input(REGEX_PROMPT).strip(
                        ) if use_interact else param
                        result = lastest.regex_search(re=re, all=args.all)

                    print(f"{INFO} 搜索字符串\"{pat or re}\"")
                    if result is None:
                        print(f"{ERROR} 未找到匹配序列： {pat or re}")
                    else:
                        print(f"{OK} 匹配序列序号：{list(map(lambda x: x + 1, result))}")
                        for res in result:
                            print(f"[{res + 1}] {lastest[res]}")
                else:
                    print(f"{ERROR} 类型 {type(lastest)} 目前不支持搜索或为空")

            case "exit":
                return (False, None)
            case _:
                print(f"{INFO} 请输入`{color("help", YELLOW)}`查看帮助")

        if not shifter:
            return (True, lastest)

        param = args.input
        if param.__len__() != 0:
            input_string = " ".join(param)
            lastest = process(input_string, lines=False,
                              verbose=verbose, backend=backend)
        else:
            input_string = input(RUST_PROMPT).strip()
            while input_string.endswith("\\"):
                lines = True
                input_string = input_string[:-1] + '\n'
                input_string += input(RUST_PROMPT).strip()
            lastest = process(input_string, lines=lines, merge=merge,
                              verbose=verbose, backend=backend)
    except argparse.ArgumentError as e:
        print(f"{ERROR} {color(e.message, YELLOW)}")
        print(f"{INFO} 请输入`{color("help", YELLOW)}`查看帮助")
    except (EOFError, KeyboardInterrupt):
        return (False, None)
    return (True, lastest)


def parse_file(file_name: str, verbose: bool, merge: bool):
    try:
        with open(file_name, "r") as f:
            lines: list[str] = f.readlines()
            lines = [line.strip() for line in lines]
            # shifter = QuickShifterLines(lines, merge=merge)
            shifter = RapidShifterLines(lines)
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
    parser.add_argument("--search", type=str, help="搜索特定字符串")
    parser.add_argument("--regex-search", type=str, help="使用正则搜索特定字符串")

    # TODO: 增加处理命令行参数搜索的逻辑

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
        console()
    elif args.server:
        from app import app

        print(BANNER)
        print("进入服务器模式")
        app.run()
