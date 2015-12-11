#!/usr/bin/env python
import sys
import re

#!bad_pairs = re.compile('ab|cd|pq|xy')
#!
#!def has_3_vowels(s):
#!	vowels = 0
#!	for c in s:
#!		if c in ('a', 'e', 'i', 'o', 'u'):
#!			vowels += 1
#!	return vowels >= 3
#!
#!def has_a_repeat(s):
#!	last = 'X'
#!	for c in s:
#!		if c == last:
#!			return True
#!		last = c
#!	return False
#!
#!def has_bad_pair(s):
#!	return bad_pairs.search(s) is not None
#!
double_pair = re.compile('(..).*\\1')
rep_sandwich = re.compile('(.).\\1')

nice = 0
with open(sys.argv[1], 'r') as f:
	for s in f:
		#if has_3_vowels(s) and has_a_repeat(s) and not has_bad_pair(s):
		if double_pair.search(s) is not None and rep_sandwich.search(s) is not None:
			nice += 1
print nice
