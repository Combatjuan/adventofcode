#!/usr/bin/env python
import re
import sys
import termcolor

# Let's make the reindeer objects for fun.
class Reindeer(object):
	def __init__(self, name, speed, fly_s, rest_s):
		assert(fly_s > 0)
		assert(rest_s > 0)
		assert(speed > 0)

		self.name = name
		self.speed = int(speed)
		self.fly_s = self.to_fly_s = int(fly_s)
		self.rest_s = self.to_rest_s = int(rest_s)

		self.distance = 0
		self.points = 0
		
	def tick(self):
		if self.to_fly_s > 0:
			self.distance += self.speed
			self.to_fly_s -= 1
		else:
			self.to_rest_s -= 1
			if self.to_rest_s == 0:
				self.to_rest_s = self.rest_s
				self.to_fly_s = self.fly_s

# Load up some reindeer.
reindeer = []
line_re = re.compile('(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\.')
with open(sys.argv[1], 'r') as f:
	for line in f:
		m = line_re.match(line.strip())
		if m:
			reindeer.append(Reindeer(m.groups()[0], m.groups()[1], m.groups()[2], m.groups()[3]))
		else:
			print "Non-matching line:\n", line

# Run the race
race_length = 2503
for s in range(race_length):
	for r in reindeer:
		r.tick()
		#print s, r.name, ('(...)' if r.to_fly_s == 0 else '====>'), r.distance, r.points
	max_distance = max([r.distance for r in reindeer])
	for r in reindeer:
		if r.distance == max_distance:
			r.points += 1

# Display results
# Part 1:
max_distance = max([r.distance for r in reindeer])
print "By Distance", max_distance, [r.name for r in reindeer if r.distance == max_distance]

# Part 2:
max_points =  max([r.points for r in reindeer])
print "By Points", max_points, [r.name for r in reindeer if r.points == max_points]

