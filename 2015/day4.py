#!/usr/bin/env python
import md5

value = ''
i = -1
key = 'iwrupvqb'
while True:
	i = i + 1
	m = md5.new()
	m.update(key + str(i))
	value = m.hexdigest()
	if value.startswith('000000'):
		break
print i

