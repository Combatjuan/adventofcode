using System;
using System.Collections.Generic;

namespace day10
{
    class Program
    {
		class Star
		{
			public Star(int x, int y, int dx, int dy)
			{
				this.x = x;
				this.y = y;
				this.dx = dx;
				this.dy = dy;
			}
			public void Move()
			{
				x = x + dx;
				y = y + dy;
			}
			public void MoveBack()
			{
				x = x - dx;
				y = y - dy;
			}
			public int x;
			public int y;
			int dx;
			int dy;
		}

		static void PrintStarField(List<Star> stars, int x, int y, int w, int h)
		{
			bool[,] field = new bool[w, h];
			foreach (Star s in stars)
			{
				field[s.x - x, s.y - y] = true;
			}
			for (int j = 0; j < h; ++j)
			{
				for (int i = 0; i < w; ++i)
				{
					if (field[i, j]) Console.Write("*");
					else Console.Write(" ");
				}
				Console.WriteLine("");
			}
		}

		static (int, int, int, int) MoveAllStars(List<Star> stars, bool forward)
		{
			int min_x = Int32.MaxValue;
			int min_y = Int32.MaxValue;
			int max_x = Int32.MinValue;
			int max_y = Int32.MinValue;
			foreach (Star s in stars)
			{
				if (forward) s.Move();
				else s.MoveBack();

				if (s.x < min_x) min_x = s.x;
				if (s.y < min_y) min_y = s.y;
				if (s.x > max_x) max_x = s.x;
				if (s.y > max_y) max_y = s.y;
			}
			return (min_x, min_y, max_x, max_y);
		}

        static void Main(string[] args)
        {
			// Allow for a different input file
			string filename = "input.txt";
			if (args.Length > 0) filename = args[0];

			// Read the file in
			System.IO.StreamReader file = new System.IO.StreamReader(filename);
			List<Star> stars = new List<Star>();
			string line;
			// Load in the stars
			while ((line = file.ReadLine()) != null)
			{
				int x = Int32.Parse(line.Substring(10, 6));
				int y = Int32.Parse(line.Substring(18, 6));
				int dx = Int32.Parse(line.Substring(36, 2));
				int dy = Int32.Parse(line.Substring(40, 2));
				stars.Add(new Star(x, y, dx, dy));
			}

			int second = 0;
			// Assume that the message occurs very near when the stars
			// are most closely clustered.  That is, when the furthest stars
			// away from one another is minimized.
			int cluster = Int32.MaxValue;
			int min_cluster = Int32.MaxValue;
			while (true)
			{
				int min_x, min_y, max_x, max_y;
				(min_x, min_y, max_x, max_y) = MoveAllStars(stars, true);

				cluster = (max_x - min_x) + (max_y - min_y);
				if (cluster < min_cluster) min_cluster = cluster;
				else if (cluster > min_cluster)
				{
					PrintStarField(stars, min_x, min_y, max_x - min_x + 1, max_y - min_y + 1);
					Console.WriteLine("--------------------------------------------------------");
					(min_x, min_y, max_x, max_y) = MoveAllStars(stars, false);
					PrintStarField(stars, min_x, min_y, max_x - min_x + 1, max_y - min_y + 1);
					Console.WriteLine("--------------------------------------------------------");
					Console.WriteLine("After {0} seconds", second);
					break;
				}
				second++;
			}
        }
    }
}
