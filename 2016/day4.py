#!/usr/bin/env python
import re

part_regex = re.compile('((([a-z]+)-))+([0-9]+)\[([a-z]+)\]')

with file('day4.input') as f:
	for line in f:
		m = part_regex.match(line)
		if not m:
			sys.exit
		else:
			print m.groups()
			print m.groups()[1]
		
