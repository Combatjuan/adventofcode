using System;
using System.Collections.Generic;

namespace day9
{
	class Ring
	{
		public Ring()
		{
			this.marbles = new LinkedList<long>();
			ptr = marbles.AddFirst(0);
		}

		public long Turn(long n)
		{
			long points = 0;
			if (n % 23 == 0)
			{
				MoveCounterClockwise(7);
				LinkedListNode<long> to_remove = ptr;
				points += n + to_remove.Value;
				MoveClockwise(1);
				marbles.Remove(to_remove);
			}
			else
			{
				MoveClockwise(1);
				ptr = marbles.AddAfter(ptr, n);
			}
			return points;
		}

		public void Print()
		{
			LinkedListNode<long> node = marbles.First;
			do
			{
				if (node == ptr) {
					Console.Write("({0})	", node.Value);
				}
				else Console.Write(" {0}	", node.Value);
			} while (!((node = node.Next) is null));
			Console.WriteLine("");
		}

		void MoveClockwise(long n)
		{
			for (long i = 0; i < n; ++i)
			{
				LinkedListNode<long> next = this.ptr.Next;
				if (next is null)
				{
					//Console.WriteLine("Wrap to first.");
					this.ptr = this.marbles.First;
				}
				else
				{
					//Console.WriteLine("{0} -> {1}", this.ptr.Value, next.Value);
					this.ptr = next;
				}
			}
		}

		void MoveCounterClockwise(long n)
		{
			for (long i = 0; i < n; ++i)
			{
				LinkedListNode<long> prev = this.ptr.Previous;
				if (prev is null)
				{
					//Console.WriteLine("Wrap to last.");
					this.ptr = this.marbles.Last;
				}
				else
				{
					//Console.WriteLine("{0} <- {1}", prev.Value, this.ptr.Value);
					this.ptr = prev;
				}
			}
		}

		LinkedList<long> marbles;
		LinkedListNode<long> ptr;
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
			string line = file.ReadLine();
			string[] parts = line.Split();
			// 423 players; last marble is worth 71944 points
			long players = Int32.Parse(parts[0]);
			long end = Int32.Parse(parts[6]);

			long[] scores = new long[players];
			Ring marbles = new Ring();
			for (long i = 1; i <= end; ++i) 
			{
				long points = marbles.Turn(i);
				long player = (i - 1) % players + 1;
				scores[player - 1] += points;
				//Console.Write("{0}:	", player);
				//marbles.Print();
			}

			// Show the winner
			long winner = 0;
			long max = 0;
			for (long i = 0; i < players; ++i)
			{
				if (scores[i] > max)
				{
					winner = i + 1;
					max = scores[i];
				}
			}
			Console.WriteLine("Player {0} wins with {1} points.", winner, max);
        }
    }
}
