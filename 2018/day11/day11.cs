using System;
using System.Collections.Generic;

namespace dayX
{
    class Program
    {
		static int PowerAtGrid(int x, int y, int serial)
		{
			int rack_id = x + 10;
			int power_digits = (rack_id * y + serial) * rack_id;
			int hundreds = (power_digits % 1000 - power_digits % 100) / 100;
			return hundreds - 5;
		}

		static int PowerOfSquareAt(int[,] grid, int x, int y, int size)
		{
			int power = 0;
			for (int j = y; j < y + size; ++j)
			{
				for (int i = x; i < x + size; ++i) power += grid[i, j];
			}
			return power;
		}

        static void Main(string[] args)
        {
			// Allow for a different input file
			string filename = "input.txt";
			if (args.Length > 0) filename = args[0];

			// Read the file in
			System.IO.StreamReader file = new System.IO.StreamReader(filename);
			int serial = Int32.Parse(file.ReadLine());
			const int WIDTH = 300;
			const int HEIGHT = 300;

			// Man do I ever hate 1-based indexing.
			// Screw it.  Let's just make the array bigger.
			int[,] grid = new int[WIDTH + 1, HEIGHT + 1];
			int[,] grid3x3 = new int[WIDTH + 1, HEIGHT + 1];
			for (int y = 1; y <= HEIGHT; ++y)
			{
				for (int x = 1; x <= WIDTH; ++x)
				{
					grid[x, y] = PowerAtGrid(x, y, serial);
				}
			}
			// 

			int max = -100;
			int x_of_max = 0;
			int y_of_max = 0;
			int s_of_max = 0;
			const int MAX_SIZE = 25;
			for (int x = 1; x <= WIDTH; ++x)
			{
				for (int y = 1; y < HEIGHT; ++y)
				{
					int max_width = WIDTH - x;
					int max_height = HEIGHT - y;
					int max_size = max_height;
					if (max_width < max_height) max_size = max_width;
					if (max_size > MAX_SIZE) max_size = MAX_SIZE;

					for (int s = 0; s < max_size; ++s)
					{
						int square_power = PowerOfSquareAt(grid, x, y, s);
						if (square_power > max)
						{
							x_of_max = x;
							y_of_max = y;
							s_of_max = s;
							max = square_power;
						}
					}
				}
			}
			Console.WriteLine("Power is {0} at ({1}, {2}, {3})", max, x_of_max, y_of_max, s_of_max);
        }
    }
}
