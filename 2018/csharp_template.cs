using System;
using System.Collections.Generic;

namespace dayX
{
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
        }
    }
}
