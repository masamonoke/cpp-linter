import re
import sys

def check_pointer_naming(filename):
    with open(filename, 'r') as file:
        lines = file.readlines()

    pointer_regex = re.compile(r'\b(\w+)\s*\*\s*(\w*)\b')

    for line_num, line in enumerate(lines, 1):
        match = pointer_regex.search(line)
        if match:
            pointer_name = match.group(2)
            declaration = match.group(0).strip()

            if pointer_name and not pointer_name.startswith('p'):
                print(f"Line {line_num}: Pointer variable '{pointer_name}' should start with 'p'.")

            if not re.search(r'\b(\w+)\s+\*\s*(\w*)\b', line):
                print(f"Line {line_num}: '{declaration}' should have a space between the type and '*'.")

check_pointer_naming(sys.argv[1])

