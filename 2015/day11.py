#!/usr/bin/env python
import ctypes
import re


def inc_char(c):
	if c == 'z': return 'a'
	else: return chr(ord(c) + 1)
	
def tri_increments(password):
	ords = [ord(c) for c in password]
	for i, o in enumerate(ords[:-2]):
		if o + 2 ==  ords[i+1] + 1 == ords[i+2]:
			return True
	return False

dbl_dbls = re.compile('(.)\\1.*(.)\\2')
has_oil = re.compile('[oil]')

def matches(password):
	return has_oil.search(password) is None and dbl_dbls.search(password) is not None and tri_increments(password)


#password = 'hxbxwxba'
password = 'hxbxxyzz'
while True:
	print password
	# I think this may actually be the fastest implementation even though it's ugly...
	mut = ctypes.create_string_buffer(password)
	mut[7] = inc_char(mut[7])
	if mut[7] == 'a':
		mut[6] = inc_char(mut[6])
		if mut[6] == 'a':
			mut[5] = inc_char(mut[5])
			if mut[5] == 'a':
				mut[4] = inc_char(mut[4])
				if mut[4] == 'a':
					mut[3] = inc_char(mut[3])
					if mut[3] == 'a':
						mut[2] = inc_char(mut[2])
						if mut[2] == 'a':
							mut[1] = inc_char(mut[1])
							if mut[1] == 'a':
								mut[0] = inc_char(mut[0])
								if mut[0] == 'a':
									mut[7] = inc_char(mut[7])
	password = mut.value
	if matches(password):
		break

print password
