using System;
using System.Collections.Generic;

namespace day8
{
	class Node
	{
		public Node()
		{
			this.children = new List<Node>();
			this.metadata = new List<int>();
		}

		public void AddChild(Node child)
		{
			children.Add(child);
		}

		public void AddMetadata(int n)
		{
			metadata.Add(n);
		}

		// Recursive sum of all Metadata (Part A)
		public int SumAllMetadata()
		{
			int sum = this.SumMetadata();
			foreach (Node child in this.children)
			{
				sum = sum + child.SumAllMetadata();
			}
			return sum;
		}

		// Sum of just this node's metadata.
		public int SumMetadata()
		{
			int sum = 0;
			foreach (int datum in this.metadata)
			{
				sum = sum + datum;
			}
			return sum;
		}

		// Part 2 silly sum
		// Consider memoization since this can get called multiple times.
		public int SpecialSum()
		{
			int sum = 0;
			if (this.children.Count == 0) return this.SumMetadata();
			else
			{
				foreach (int datum in this.metadata)
				{
					// Problem uses one-based indexing
					int child_index = datum - 1;
					if (child_index >= 0 && child_index < this.children.Count)
					{
						sum += this.children[child_index].SpecialSum();
					}
				}
			}
			return sum;
		}

		List<Node> children;
		List<int> metadata;
	}

    class Program
    {
		static public Node MakeNode(IEnumerator<int> numbers)
		{
			Node node = new Node();
			int child_count = numbers.Current;
			numbers.MoveNext();
			int data_count = numbers.Current;
			numbers.MoveNext();
			for (int c = 0; c < child_count; ++c)
			{
				node.AddChild(MakeNode(numbers));
			}
			for (int n = 0; n < data_count; ++n)
			{
				node.AddMetadata(numbers.Current);
				numbers.MoveNext();
			}
			return node;
		}

		// Defining simple iterators in C# is pretty nice.
		public static IEnumerator<int> InputNumbers(String input_string)
		{
			string[] data = input_string.Split();
			foreach (String s in data)
			{
				yield return Int32.Parse(s);
			}
		}

        static void Main(string[] args)
        {
			// Allow for a different input file
			string filename = "input.txt";
			if (args.Length > 0) filename = args[0];

			// Read the file in
			System.IO.StreamReader file = new System.IO.StreamReader(filename);

			// Ideally we'd stream the file a few (buffered) bytes at a time so and just
			// advance our file pointer, but meh.  These inputs will be much less than RAM.
			string line = file.ReadLine();
			IEnumerator<int> numbers = InputNumbers(line);
			numbers.MoveNext();
			Node root = MakeNode(numbers);

			Console.Write("Part A - Simple Sum: ");
			Console.WriteLine(root.SumAllMetadata());

			Console.Write("Part B - Special Sum: ");
			Console.WriteLine(root.SpecialSum());
        }
    }
}
