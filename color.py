BOLD = "\033[01m"

RED = "\033[01;31m"
GREEN = "\033[01;32m"
YELLOW = "\033[01;33m"
BLUE = "\033[01;34m"
PURPLE = "\033[01;35m"
CYAN = "\033[01;36m"

# 0xDEA584
RUST = "\033[01;38;2;222;165;132m"
# 0xFFBC03
PYTHON = "\033[01;38;2;255;188;3m"

END = "\033[0m"


def color(string: str, color: str):
    return color + string + END
