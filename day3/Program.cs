using System;

namespace day3
{
    class Program
    {
        static void Main(string[] args)
        {
            string[] lines = System.IO.File.ReadAllLines(@"./data.txt");
            double Go(int right, int down)
            {
                var tree = 0;
                var position = 1;
                for (int i = down; i < lines.Length; i += down)
                {
                    position += right;
                    var line = lines[i];
                    if (position > line.Length)
                    {
                        position -= line.Length;
                    }
                    var letter = line[position - 1];
                    if (letter == '#')
                    {
                        tree += 1;
                    }
                    Console.WriteLine(letter + "]: ${letter}  [" + $"{position:D2}" + "]: " + line);
                }
                Console.WriteLine("Trees: " + tree);
                return tree;
            }
            Console.WriteLine(Go(1, 1) *
            Go(3, 1) *
            Go(5, 1) *
            Go(7, 1) *
            Go(1, 2));
        }
    }
}
