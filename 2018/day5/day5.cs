using System;
using System.Collections.Generic;

namespace day5
{
    class Program
    {
		static int react(string polymer) {
			// Envision a stack
			// dabAcCaCBAcCcaDA
			// d
			// da
			// dab
			// dabA
			// dabAc
			// dabA!!
			// dab!!
			// dab
			// ...
			//
			// Move left to right adding letters to the stack.  If the top of the stack
			// has a pair with that letter, then don't add it and instead pop the stack.
			// Proceed.
			Stack<char> processor = new Stack<char>();
			for (int i = 0; i < polymer.Length; ++i) {
				char c = polymer[i];
				if (processor.Count > 0) {
					char opposite = Char.IsUpper(c) ? Char.ToLower(c) : Char.ToUpper(c);
					if (processor.Peek() == opposite) processor.Pop();
					else processor.Push(c);
				}
				else processor.Push(c);
			}
			return processor.Count;
		}

		static string filter(string polymer, char type) {
			// There must be a better way to make Char and String play nice in C#
			char[] upper_char = new char[1];
			upper_char[0] = type;
			char[] lower_char = new char[1];
			lower_char[0] = Char.ToLower(type);
			string upper = new string(upper_char);
			string lower = new string(lower_char);
			return polymer.Replace(upper, "").Replace(lower, "");
		}

        static void Main(string[] args)
        {
			// Allow for a different input file
			string filename = "input.txt";
			if (args.Length > 0) {
				filename = args[0];
			}

			// Read the file in
			System.IO.StreamReader file = new System.IO.StreamReader(filename);
			string polymer= file.ReadLine();

			// Part A: Just react the polymer and return the length.
			Console.WriteLine(react(polymer));

			// Part B: If you could remove all of one type from the polymer before reacting
			// what is the smallest length you could achieve?
			int min = polymer.Length;
			char letter = '?';
			string foo = String.Join('A', 'B');
			string alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
			for (int i = 0; i < alphabet.Length; ++i) {
				string simplified = filter(polymer, alphabet[i]);
				int react_length = react(simplified);
				if (react_length < min) {
					min = react_length;
					letter = alphabet[i];
				}
			}
			Console.WriteLine("{0} ({1})", min, letter);
        }
    }
}
