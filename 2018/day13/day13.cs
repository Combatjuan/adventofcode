using System;
using System.Collections.Generic;

namespace day13
{
	enum Decision
	{
		GO_LEFT,
		GO_STRAIGHT,
		GO_RIGHT
	}

	enum Facing
	{
		UP,
		LEFT,
		DOWN,
		RIGHT
	}

	class Train : IComparable<Train>
	{
		public Train(int row, int column, char c, int number)
		{
			this.row = row;
			this.column = column;
			this.next_decision = Decision.GO_LEFT;
			this.number = number;

			if (c == 'v') this.facing = Facing.DOWN;
			else if (c == '^') this.facing = Facing.UP;
			else if (c == '<') this.facing = Facing.LEFT;
			else if (c == '>') this.facing = Facing.RIGHT;
		}

		int IComparable<Train>.CompareTo(Train that)
		{
			if (this.row < that.row) return -1;
			else if (this.row > that.row) return 1;
			else
			{
				if (this.column < that.column) return -1;
				else if (this.column > that.column) return 1;
			}
			return 0;
		}

		public int row;
		public int column;
		public int number;
		public Facing facing;
		public Decision next_decision;
	}

	class Program
	{
		static char CharForFacing(Facing facing)
		{
			if (facing == Facing.LEFT) return '<';
			else if (facing == Facing.RIGHT) return '>';
			else if (facing == Facing.DOWN) return 'v';
			else if (facing == Facing.UP) return '^';
			else throw new InvalidOperationException("Unexpected facing.");
		}

		static Facing LeftOf(Facing facing)
		{
			if (facing == Facing.UP) return Facing.LEFT;
			else if (facing == Facing.LEFT) return Facing.DOWN;
			else if (facing == Facing.DOWN) return Facing.RIGHT;
			else if (facing == Facing.RIGHT) return Facing.UP;
			else throw new InvalidOperationException("Unexpected facing.");
		}

		static Facing RightOf(Facing facing)
		{
			if (facing == Facing.UP) return Facing.RIGHT;
			else if (facing == Facing.RIGHT) return Facing.DOWN;
			else if (facing == Facing.DOWN) return Facing.LEFT;
			else if (facing == Facing.LEFT) return Facing.UP;
			else throw new InvalidOperationException("Unexpected facing.");
		}

		static Decision NextDecision(Decision decision)
		{
			if (decision == Decision.GO_LEFT) return Decision.GO_STRAIGHT;
			else if (decision == Decision.GO_STRAIGHT) return Decision.GO_RIGHT;
			else if (decision == Decision.GO_RIGHT) return Decision.GO_LEFT;
			else throw new InvalidOperationException("Unexpected decision.");
		}

		//static void PrintTracks(List<char[]> tracks, List<Train> trains)
		//{
		//	Console.BackgroundColor = ConsoleColor.Black;
		//	Console.ForegroundColor = ConsoleColor.White;
		//	Console.Clear();
		//	foreach (char[] track in tracks)
		//	{
		//		Console.WriteLine(track);
		//	}
		//	foreach (Train train in trains)
		//	{
		//		if (train.number == 5 || train.number == 10) Console.ForegroundColor = ConsoleColor.Red;
		//		else Console.ForegroundColor = ConsoleColor.Blue;
		//		Console.SetCursorPosition(train.column, train.row);
		//		Console.Write(CharForFacing(train.facing));
		//	}
		//	Console.SetCursorPosition(0, 0);
		//}

		//const int FRAME_SLEEP_MS = 5;

		static void Main(string[] args)
		{
			// Allow for a different input file
			string filename = "input.txt";
			if (args.Length > 0) filename = args[0];

			// Read the file in
			System.IO.StreamReader file = new System.IO.StreamReader(filename);
			List<Char[]> tracks = new List<Char[]>();
			List<Train> trains = new List<Train>();
			string line;
			int r = 0;
			int train_number = 1;
			// Read in the tracks, build a list of trains
			// Modify the tracks under the trains in place.
			while ((line = file.ReadLine()) != null)
			{
				char[] track_row = line.ToCharArray();
				int c = 0;
				foreach (char symbol in track_row)
				{
					if (symbol == 'v' || symbol == '^')
					{
						Train train = new Train(r, c, symbol, train_number++);
						track_row[c] = '|';
						trains.Add(train);
					}
					else if (symbol == '>' || symbol == '<')
					{
						Train train = new Train(r, c, symbol, train_number++);
						track_row[c] = '-';
						trains.Add(train);
					}
					++c;
				}
				tracks.Add(track_row);
				//Console.WriteLine(line);
				++r;
			}

			int collision_x = -1;
			int collision_y = -1;
			Dictionary<(int, int), int> locations = new Dictionary<(int, int), int>();
			List<(int, int)> crashes;
			// Explicit break when we collide
			while (trains.Count > 1)
			{
				List<int> crashed = new List<int>();
				trains.Sort();

				foreach (Train train in trains)
				{
					locations.Remove((train.row, train.column));
					// Move each train
					int row = train.row;
					int col = train.column;
					if (train.facing == Facing.UP) row -= 1;
					else if (train.facing == Facing.DOWN) row += 1;
					else if (train.facing == Facing.LEFT) col -= 1;
					else if (train.facing == Facing.RIGHT) col += 1;

					// Oh noes!  A collision!!!
					if (locations.ContainsKey((row, col)))
					{
						collision_x = col;
						collision_y = row;
						crashes.Add((row, col));
						crashed.Add(train.number);
						crashed.Add(locations[(row, col)]);
						locations.Remove((row, col));
						break;
					}
					else
					{	
						locations.Add((row, col), train.number);
						train.row = row;
						train.column = col;
					}

					// Now deal with trains potentially facing new directions
					char symbol = tracks[row][col];
					if (symbol == '+')
					{
						if (train.next_decision == Decision.GO_LEFT) train.facing = LeftOf(train.facing);
						else if (train.next_decision == Decision.GO_RIGHT) train.facing = RightOf(train.facing);
						train.next_decision = NextDecision(train.next_decision);
					}
					else if (symbol == '/')
					{
						if (train.facing == Facing.UP) train.facing = Facing.RIGHT;
						else if (train.facing == Facing.LEFT) train.facing = Facing.DOWN;
						else if (train.facing == Facing.RIGHT) train.facing = Facing.UP;
						else if (train.facing == Facing.DOWN) train.facing = Facing.LEFT;
					}
					else if (symbol == '\\')
					{
						if (train.facing == Facing.UP) train.facing = Facing.LEFT;
						else if (train.facing == Facing.LEFT) train.facing = Facing.UP;
						else if (train.facing == Facing.RIGHT) train.facing = Facing.DOWN;
						else if (train.facing == Facing.DOWN) train.facing = Facing.RIGHT;
					}
				}

				// Remove the crashed trains
				foreach (int number in crashes)
				{
					trains.Remove(number);
				}

				//PrintTracks(tracks, trains);
				//System.Threading.Thread.Sleep(FRAME_SLEEP_MS);
			}
			//Console.SetCursorPosition(collision_x, collision_y);
			//Console.ForegroundColor = ConsoleColor.Red;
			//Console.Write('X');
			//Console.SetCursorPosition(0, 151);
			//Console.SetCursorPosition(0, tracks.Count + 1);
			Console.WriteLine("Oh no! Collision at {0},{1}", collision_x, collision_y);
		}
	}
}
