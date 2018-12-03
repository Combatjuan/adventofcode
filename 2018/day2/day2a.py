#!/usr/bin/env python3

DEBUG = False
def debug_print(s):
    print(s)

def check(s):
    """Returns two booleans, the first is whether there are character twins.  The second is whether there are character triplets."""
    t = sorted(s)
    t.append("\n")
    has_twins = False
    has_triplets = False
    run_count = 1
    prev = None
    for x in t:
        if x == prev:
            run_count = run_count + 1
        else:
            if run_count == 2:
                has_twins = True
            elif run_count == 3:
                has_triplets = True
            run_count = 1
            prev = x
        prev = x

    debug_print("{}:    {} {}".format(s, has_twins, has_triplets))
    return has_twins, has_triplets

twins_count = 0
triplets_count = 0
with open("input.txt") as the_file:
    for line in the_file:
        s = line.strip()
        has_twins, has_triplets = check(s)
        if has_twins: twins_count = twins_count + 1
        if has_triplets: triplets_count = triplets_count + 1

print(twins_count * triplets_count)

