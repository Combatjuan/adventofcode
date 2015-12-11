#!/usr/bin/env python

def string_to_parts(s):
	last = s[0]
	count = 1
	parts = []
	for c in s[1:]:
		if c == last:
			count += 1
		else:
			parts.append((count, last))
			count = 1
		last = c
	parts.append((count, last))
	return parts

def parts_to_string(parts):
	s = ''
	for count, c in parts:
		s += str(count) + c
	return s

s = '1113122113'
print s
#for i in range(1, 41):
for i in range(1, 51):
	s = parts_to_string(string_to_parts(s))
	print i, s
print len(s)

