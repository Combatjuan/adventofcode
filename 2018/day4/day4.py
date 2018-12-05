#!/usr/bin/env python3
import datetime
import re
import sys

# [1518-11-10 23:52] Guard #881 begins shift
# [1518-04-23 00:58] wakes up
# [1518-05-14 00:40] falls asleep

LINE_DEF = re.compile(".(\d\d\d\d)-(\d\d)-(\d\d) (\d\d):(\d\d). (Guard #(\d+) begins shift|wakes up|falls asleep)")

filename = "input.txt"
if len(sys.argv) > 1:
	filename = sys.argv[1]

# Read in logical records
records = []
with open(filename) as f:
	for line in f:
		line = line.strip()
		m = LINE_DEF.match(line)
		if m:
			g = m.groups()
			d = datetime.datetime(
					year = int(g[0]), 
					month = int(g[1]), 
					day = int(g[2]), 
					hour = int(g[3]), 
					minute = int(g[4]), 
			)
			guard = int(g[6]) if g[6] else None
			event = g[5]

			records.append((d, guard, event))
		else:
			print("Could not interpret line:", line)
			sys.exit(1)

# Sort and interpret the records
elf_naps = {}
records = sorted(records)
current_guard = None
asleep = None
for (time, guard, event) in records:
	if guard:
		current_guard = guard
	elif event == "falls asleep":
		asleep = time.minute
	elif event == "wakes up":
		elf_naps.setdefault(current_guard, []).append((time.minute - asleep, asleep, time.minute))
	else:
		raise RuntimeError("Could not understand event: {}".format(event))

# Now find the nappiest elves
(_, sleepiest) = sorted(
		[(sum([minute for minute, _, _ in times]), guard) for guard, times in elf_naps.items()]
)[-1]

def sleepiest_minute(naps):
	# Now figure out when that sleepy elf likes to nap most
	minutes = {}
	for (_, start, end) in naps:
		for m in range(start, end):
			if m in minutes:
				minutes[m] += 1
			else:
				minutes[m] = 1
	return sorted(minutes.items(), key=lambda x: x[1])[-1]

# Part 1 just wants the sleepiest minute of the sleepiest elf
naps = elf_naps[sleepiest]
sleepy_minute, _ = sleepiest_minute(naps)

print(sleepy_minute * sleepiest)

# Part 2 wants the most sleepiest minute out of all the elves
highest_count = 0
most_consistent_elf = 0
most_consistent_minute = -1
for elf, naps in elf_naps.items():
	sleepy_minute, count = sleepiest_minute(naps)
	if count > highest_count:
		highest_count = count
		most_consistent_elf = elf
		most_consistent_minute = sleepy_minute

print(most_consistent_minute * most_consistent_elf)

