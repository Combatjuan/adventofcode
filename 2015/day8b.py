#!/usr/bin/env python
import sys

# '\\' (single backslash)
# '\"' (single double-quote)
# '\xXX' (single hex char)

chars = 0
qchars = 0

escaped = False
hex_char = None

with open(sys.argv[1], 'r') as f:
	for line in f:
		line = line.strip()
		chars += len(line)
		print line, len(line)
		line = line.replace('\\', '\\\\')
		line = line.replace('"', '\\"')
		line = '"' + line + '"'
		print line, len(line)
		print
		qchars += len(line)
print qchars - chars

