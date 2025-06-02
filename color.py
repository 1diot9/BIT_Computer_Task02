BOLD = "\033[01m"

RED = "\033[01;31m"
GREEN = "\033[01;32m"
YELLOW = "\033[01;33m"
BLUE = "\033[01;34m"
PURPLE = "\033[01;35m"
CYAN = "\033[01;36m"


END = "\033[0m"


def color(string: str, color: str):
    return color + string + END
