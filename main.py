from qshifter import QuickShifter
import argparse

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
    pass
