using System;
using System.Collections.Generic;

namespace day7
{
    class Program
    {
		static void AppendAtKey(Dictionary<Char, SortedSet<Char>> dict, Char key, Char val)
		{
			SortedSet<Char> value_set;
			if (dict.TryGetValue(key, out value_set))
			{
				value_set.Add(val);
			}
			else
			{
				dict[key] = new SortedSet<Char>();
				dict[key].Add(val);
			}
		}

		static int TimeForStep(Char step)
		{
			return 61 + step - 'A';
			// This is how the example tracks time
			//return 1 + step - 'A';
		}

		//   -->A--->B--
		//  /    \      \
		// C      -->D----->E
		//  \           /
		//   ---->F-----
		// CABDFE
        static void Main(string[] args)
        {
			// Allow for a different input file
			string filename = "input.txt";
			if (args.Length > 0) filename = args[0];

			// Read the file in
			System.IO.StreamReader file = new System.IO.StreamReader(filename);
			string line;
			// Instead of learning how to parse a string in C#...
			// 01234567890123456789012345678901234567890123456
			//      5                             36
			// Step A must be finished before step R can begin.

			// Build structures holding dependents and dependees
			Dictionary<Char, SortedSet<Char>> key_needs = new Dictionary<Char, SortedSet<Char>>();
			Dictionary<Char, SortedSet<Char>> key_allows = new Dictionary<Char, SortedSet<Char>>();

			// Also TIL: SortedList is not a list.

			// A simple plan:
			// Keep track of what letters we're dealing with.
			SortedSet<Char> steps = new SortedSet<Char>();
			// Have a sorted list of unfinished steps.
			SortedSet<Char> unfinished = new SortedSet<Char>();
			// Have a set of finished sets.
			SortedSet<Char> finished = new SortedSet<Char>();
			// Keep a list of the order as we make it.
			List<Char> order = new List<Char>();

			while ((line = file.ReadLine()) != null)
			{
				char allower = line[5];
				char needer = line[36];
				steps.Add(allower);
				steps.Add(needer);

				AppendAtKey(key_needs, needer, allower);
				AppendAtKey(key_allows, allower, needer);
				Console.WriteLine("{0} -> {1}", allower, needer);
			}

			// Bootstrap the unfinished list
			foreach (Char c in steps) { unfinished.Add(c); }

			// Until we have no more steps, find the first one with all its needs met.
			Dictionary<Char, int> jobs = new Dictionary<Char, int>();
			int WORKERS = 5;
			// This is how many workers are in the example
			//int WORKERS = 2;
			int time = 0;
			while (unfinished.Count != 0 || jobs.Count != 0)
			{
				Console.Write(time);
				Console.Write("	");
				// TODO While there are workers available find new jobs ==
				if (jobs.Count < WORKERS)
				{
					// Can't modify in a loop in CSharp.
					// One way is to record modifications for later.  (Done here).
					// Another is to iterate over a copy of keys (Done later).
					List<Char> started = new List<Char>();
				
					foreach (Char step in unfinished)
					{
						SortedSet<Char> needs;
						if (!key_needs.TryGetValue(step, out needs)) needs = new SortedSet<Char>();
						if (needs.IsSubsetOf(finished))
						{
							started.Add(step);
							order.Add(step);
							jobs[step] = TimeForStep(step);

							if (jobs.Count == WORKERS) break;
						}
					}

					foreach (Char step in started) { unfinished.Remove(step); }
				}

				// Do work on the active jobs
				List<Char> keys = new List<Char>(jobs.Keys);
				foreach (var step in keys)
				{
					int job_time = jobs[step];
					Console.Write(step);
					Console.Write("	");
					Console.Write(job_time);

					jobs[step] = job_time - 1;

					// One unit time is being done right now, so we'll be done
					// at the end of this iteration.
					if (job_time == 1)
					{
						Console.Write("!");
						finished.Add(step);
						jobs.Remove(step);
					}
					Console.Write("	");
				}

				Console.WriteLine("");
				time++;
			}

			// Print results
			Console.Write("Part A (Order): ");
			foreach (Char c in order) { Console.Write(c); }
			Console.WriteLine("");
			Console.Write("Part B (Time):  ");
			Console.WriteLine(time);
        }
    }
}
