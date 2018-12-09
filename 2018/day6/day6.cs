using System;
using System.Collections.Generic;

namespace day6
{
	/// Just a simple (X, Y) coordinate
	class Coord
	{
		public Coord(int x, int y)
		{
			this.x = x;
			this.y = y;
		}

		public Coord(string from_string)
		{
			string[] parts = from_string.Split(",", 2);
			this.x = Int32.Parse(parts[0].Trim());
			this.y = Int32.Parse(parts[1].Trim());
		}

		public int X
		{
			get { return this.x; }
		}
		public int Y
		{
			get { return this.y; }
		}

		public int x;
		public int y;
	}

	class Spot
	{
		public Spot(Coord c)
		{
			this.x = c.x;
			this.y = c.y;
			this.id = ++max_id;
			this.infinite = false;
			this.area = new List<Coord>();
		}
		public int id;
		public bool infinite;
		public List<Coord> area;
		public int x;
		public int y;

		public int DistanceTo(int x, int y)
		{
			return Math.Abs(x - this.x) + Math.Abs(y - this.y);
		}

		static int max_id = 1;
	}

    class Program
    {
        static void Main(string[] args)
        {
			// Allow for a different input file
			string filename = "input.txt";
			if (args.Length > 0) filename = args[0];

			// Read the file in
			System.IO.StreamReader file = new System.IO.StreamReader(filename);
			string line;
			int clustered_size = 0;
			int max_width = 0;
			int max_height = 0;

			Dictionary<int, Spot> spots = new Dictionary<int, Spot>();

			// Find the dimensions of the area we're concerned with.
			// Load data about the spots of interest.
			while ((line = file.ReadLine()) != null)
			{
				Coord c = new Coord(line);
				Spot s = new Spot(c);
				spots[s.id] = s;

				if (c.x > max_width) max_width = c.x;
				if (c.y > max_height) max_height = c.y;
			}

			/// Find the closest spot to each coordinate
			for (int y = 0; y <= max_height; y++)
			{
				for (int x = 0; x <= max_width; x++)
				{
					int min = max_width + max_height;
					int id = 0;
					int total_distance = 0;
					foreach (var iter in spots)
					{
						Spot spot = iter.Value;
						int distance = spot.DistanceTo(x, y);
						if (distance == min)
						{
							id = 0;
						}
						else if (distance < min)
						{
							id = spot.id;
							min = distance;
						}
						total_distance += distance;
					}
					if (id != 0)
					{
						Spot spot = spots[id];
						spot.area.Add(new Coord(x, y));
						if (x == 0 || y == 0 || x == max_width || y == max_height)
						{
							spot.infinite = true;
						}
					}
					if (total_distance < 10000) clustered_size++;

					// DEBUG: Print a map for sanity checking
					// Print a map of regions
					//if (id == 0) Console.Write(".");
					//else Console.Write(Char.ConvertFromUtf32('A' - 1 + id));

					// Print a cluster map
					//if (total_distance < 10000) Console.Write("#");
					//else Console.Write(".");
				}
				//Console.WriteLine("");
			}

			int max = 0;
			int biggest = 0;
			foreach (var iter in spots)
			{
				Spot s = iter.Value;
				if (!s.infinite && s.area.Count > max)
				{
					max = s.area.Count;
					biggest = s.id;
				}
			}
			Console.WriteLine("Part A: {0}", max);
			Console.WriteLine("Part B: {0}", clustered_size);
        }
    }
}

