using System;
using System.Linq;
using System.Collections.Generic;

namespace day14
{
	class Program
	{
		static void Print(List<int> recipes, int first_elf, int second_elf)
		{
			for (int i = 0; i < recipes.Count; ++i)
			{
				if (i == first_elf) Console.Write("({0})", recipes[i]);
				else if (i == second_elf) Console.Write("[{0}]", recipes[i]);
				else Console.Write(" {0} ", recipes[i]);
			}
			Console.WriteLine();
		}

		static List<int> NumberStringToDigitList(string s)
		{
			List<int> digits = new List<int>();
			foreach (char c in s)
			{
				digits.Add(c - '0');
			}
			return digits;
		}

		static int DigitListToNumber(List<int> digits)
		{
			return digits[0] * 100000
				+ digits[1] * 10000
				+ digits[2] * 1000
				+ digits[3] * 100
				+ digits[4] * 10
				+ digits[5];
		}

		static void PartA(string s)
		{
			int iterations = Int32.Parse(s);
			List<int> recipes = new List<int> {3, 7};
			int elfy_index = 0;
			int elfina_index = 1;

			//Print(recipes, elfy_index, elfina_index);
			while (recipes.Count < iterations + 10)
			{
				int new_score = recipes[elfy_index] + recipes[elfina_index]; 
				int tens_digit = new_score / 10;
				int ones_digit = new_score % 10;
				if (tens_digit > 0) recipes.Add(tens_digit);
				recipes.Add(ones_digit);
				elfy_index = (elfy_index + 1 + recipes[elfy_index]) % recipes.Count;
				elfina_index = (elfina_index + 1 + recipes[elfina_index]) % recipes.Count;
				//Print(recipes, elfy_index, elfina_index);
				Console.Write("{0} {1}:	", elfy_index, elfina_index);
				Print(recipes, elfy_index, elfina_index);
			}
			for (int i = iterations; i < iterations + 10; ++i)
			{
				Console.Write(recipes[i]);
			}
			Console.WriteLine();
		}

		/*
		static void PartB(string s)
		{
			List<int> digits = NumberStringToDigitList(s);
			List<int> some_digits = NumberStringToDigitList("32085");
			List<int> recipes = new List<int> {3, 7};
			int elfy_index = 0;
			int elfina_index = 1;

			List<int> first_found = new List<int>();
			List<int> count_found = new List<int>();
			for (int i = 0; i < 1000000; ++i)
			{
				first_found.Add(0);
				count_found.Add(0);
			}

			while (true)
			{
				//if (recipes.Count >= some_digits.Count && some_digits.SequenceEqual(recipes.GetRange(recipes.Count - some_digits.Count - 1, some_digits.Count))) Console.WriteLine(recipes.Count);

				if (recipes.Count >= digits.Count)
				{
					int number = DigitListToNumber(recipes.GetRange(recipes.Count - digits.Count, digits.Count));
					if (first_found[number] == 0) first_found[number] = recipes.Count;
					count_found[number] += 1;

					if (digits.SequenceEqual(recipes.GetRange(recipes.Count - digits.Count, digits.Count))) break;
				}

				int new_score = recipes[elfy_index] + recipes[elfina_index]; 
				int tens_digit = new_score / 10;
				int ones_digit = new_score % 10;
				if (tens_digit > 0)
				{
					recipes.Add(tens_digit);
					//Console.Write(tens_digit);
					//Console.Write(" ");
				}
				recipes.Add(ones_digit);
				//Console.Write(ones_digit);
				//Console.Write(" ");
				elfy_index = (elfy_index + 1 + recipes[elfy_index]) % recipes.Count;
				elfina_index = (elfina_index + 1 + recipes[elfina_index]) % recipes.Count;
				//Print(recipes, elfy_index, elfina_index);	

				if (recipes.Count > 100000000)
				{
					int x = 0;
					foreach (int n in first_found)
					{
						Console.WriteLine("{0}	{1}	{2}", x , n, count_found[x]);
						++x;
					}
					break;
				}
			}

			Console.WriteLine(recipes.Count - digits.Count);
		}
		*/

		static void Main(string[] args)
		{
			// Allow for a different input file
			string filename = "input.txt";
			if (args.Length > 0) filename = args[0];

			// Read the file in
			System.IO.StreamReader file = new System.IO.StreamReader(filename);
			string line = file.ReadLine();
			PartA(line);
			//PartB(line);
		}
	}
}

