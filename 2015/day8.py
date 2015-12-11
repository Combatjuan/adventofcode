#!/usr/bin/env python
import sys

# '\\' (single backslash)
# '\"' (single double-quote)
# '\xXX' (single hex char)

chars = 0
bytes = 0

escaped = False
hex_char = None

with open(sys.argv[1], 'r') as f:
	for line in f:
		lchars = 0
		lbytes = -2 
		line = line.strip()
		for c in line:
			lchars += 1
			if escaped:
				if c == '\\':
					escaped = False
					lbytes += 1
				elif hex_char is not None:
					hex_char = hex_char + 1
					if hex_char == 2:
						hex_char = None
						escaped = False
						lbytes += 1
				elif c == 'x':
					hex_char = 0
				else:
					lbytes += 1
					escaped = False
			else:
				if c == '\\':
					escaped = True
				else:
					lbytes += 1

		print line.strip(), lchars, lbytes
		chars += lchars
		bytes = bytes + lbytes

print chars - bytes

