import re
from typing import Any, Tuple
from functools import cache


@cache
def compile_pattern(pattern: str) -> re.Pattern:
    """
    Compiles a scanf pattern into a regular expression.

    This function is cached to avoid recompiling the same pattern multiple times.

    Args:
    - pattern (str): The scanf pattern to compile.

    Returns:
    - re.Pattern: The compiled regular expression object.
    """
    scanf_to_regex = {
        "%d": "([-+]?\d+)",  # integer
        "%f": "([-+]?(\d+(\.\d*)?|\.\d+)([eE][-+]?\d+)?)",  # float
        "%s": "(\S+)",  # string
        # add more if needed
    }

    order = []
    for c1, c2 in zip(pattern, pattern[1:]):
        match (c1, c2):
            case ("%", "d"):
                order.append(int)
            case ("%", "f"):
                order.append(float)
            case ("%", "s"):
                order.append(str)
            case (_, _):
                pass

    for id, regex in scanf_to_regex.items():
        pattern = pattern.replace(id, regex)

    return re.compile(pattern), order


def scanf(pattern: str, input: str) -> Tuple[Any, ...]:
    """
    Simulates the scanf function in Python.

    Parses the 'input' string according to the 'pattern' specified, attempting to extract
    and convert values to the designated types (integer, float, string).

    Args:
    - pattern (str): A format string containing %d, %f, %s, etc., to specify the type of data to be scanned.
    - input (str): The string to be parsed.

    Returns:
    - Tuple[Any, ...]: A tuple containing the parsed and type-casted values.

    Raises:
    - ValueError: If the input does not match the pattern.

    Example:
    - scanf("%d %f", "123 45.67") -> (123, 45.67)
    """
    pattern, order = compile_pattern(pattern)
    match = pattern.match(input)

    if match is None:
        raise ValueError("scanf: no match")

    return tuple(f(s) for f, s in zip(order, match.groups()))


if __name__ == "__main__":
    pass
