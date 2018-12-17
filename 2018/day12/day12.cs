using System;
using System.Collections.Generic;

namespace day12
{
	class Rule
	{
		public Rule(string s)
		{
			string[] parts = s.Split();
			Console.WriteLine(s);

			if (parts[2] == "#") this.plant = true;
			else this.plant = false;

			this.pattern = new bool[5];

			for (int i = 0; i < 5; i++)
			{
				pattern[i] = (parts[0][i] == '#');
			}
		}

		public bool Matches(bool[] pots, int n)
		{
			for (int i = 0; i < 5; ++i)
			{
				if (pattern[i] != pots[i + n - 2]) return false;
			}
			return true;
		}
		public bool plant;
		bool[] pattern;
	}

    class Program
    {
		const int HALL_LENGTH = 1024;
		const int ZERO = HALL_LENGTH / 2;
		const int GENERATIONS = 200;

		static void PrintRuler()
		{
			for (int i = 0; i < HALL_LENGTH; ++i)
			{
				if (i == ZERO) Console.Write('0');
				else Console.Write("-");
			}
			Console.WriteLine();
		}

		static void PrintPlants(bool[] plants, int generation)
		{
			int sum = 0;
			int h = 0;
			foreach (bool b in plants)
			{
				if (b)
				{
					Console.Write('#');
					sum = sum - ZERO + h;
				}
				else Console.Write('.');
				++h;
			}
			Console.WriteLine("	{0}: {1}", generation, sum);
		}

        static void Main(string[] args)
        {
			// Allow for a different input file
			string filename = "input.txt";
			if (args.Length > 0) filename = args[0];

			// Read the file in
			System.IO.StreamReader file = new System.IO.StreamReader(filename);
			string line = file.ReadLine();
			string initial_state = line.Split()[2];

			// In theory this should be dynamically sized
			// But a nice fixed array is easier and works for our input.
			int i = 0;
			bool[] plants = new bool[HALL_LENGTH];
			foreach (char c in initial_state)
			{
				if (c == '#') plants[ZERO + i] = true;
				else plants[ZERO + i] = false;
				++i;
			}

			file.ReadLine();

			List<Rule> rules = new List<Rule>();
			while ((line = file.ReadLine()) != null)
			{
				Rule rule = new Rule(line);
				// It's not clear why we need rules that we need **all** the rules.
				if (rule.plant) rules.Add(rule);
				//rules.Add(new Rule(line));
			}

			// Actually run generations
			bool[] last_plants;
			PrintRuler();
			for (int generation = 0; generation < GENERATIONS; ++generation)
			{
				last_plants = plants;
				plants = new bool[HALL_LENGTH];
				PrintPlants(last_plants, generation);
				for (int p = 2; p < (HALL_LENGTH - 2); ++p)
				{
					foreach (Rule rule in rules)
					{
						if (rule.Matches(last_plants, p))
						{
							plants[p] = true;
						}
					}
				}
			}
			PrintRuler();
			PrintPlants(plants, GENERATIONS);
			// Notice that the pattern stabilizes at about day 93
			// Realize that each day thereafter the sum increases by 98.
			// Add the amount on day 100 to 98 times fifty billion minus 100 days later.
			// 4,900,000,001,793
        }
    }
}
