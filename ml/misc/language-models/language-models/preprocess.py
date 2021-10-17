"""
Exports a function or method called preprocess_line:

arguments: line of text (string) as an argument.
return: a new string with all characters from the line that are not in the following set removed:
characters in the English alphabet, space, digits, or the ‘.’ character.
(That is, handle characters with accents and umlauts and the other punctuation marks).

Effects:

- Function should also lowercase all remaining characters
- Convert all digits to ‘0’.
- Treat each line as a separate sequence for modelling purposes (rather than treating the entire input file as a single sequence).
"""
import re
from unidecode import unidecode


def preprocess_line(line) -> str:
    """
    Transform string to lowercase and remove non-alphanumeric characters.

    :param line: A string representing a line in the dataset.
    :type line: str
    """
    line = unidecode(line).lower()
    line = re.sub(r'[^a-z0-9\s\.]', '', line)
    line = re.sub(r'\d', '0', line)
    return line
