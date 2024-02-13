﻿internal class Program
{
    public static uint count1 = 0, count2 = 0;

    static void FillCharMatrix(char[,] matrix, int Pluses)
    {
        if (Pluses < 0 || Pluses > 100)
        {
            throw new Exception("Значение процента плюсов должно быть в диапазоне от 0 до 100.");
        }

        Random rand = new Random();
        int rows = matrix.GetLength(0);
        int cols = matrix.GetLength(1);
        int numPluses = (int)Math.Round(rows * cols * Pluses / 100.0);
        for (int i = 0; i < rows; i++)
        {
            for (int j = 0; j < cols; j++)
            {
                matrix[i, j] = ' ';
            }
        }
        for (int i = 0; i < numPluses; i++)
        {
            int Rom = rand.Next(rows);
            int Col = rand.Next(cols);
            if (matrix[Rom, Col] != '+')
            {
                matrix[Rom, Col] = '+';
                count1++;
            }
            else
            {
                i--;
            }
        }
    }

    static void FillUsingStack(int startX, int startY, char[,] a)
    {
        int rows = a.GetLength(0);
        int cols = a.GetLength(1);

        Stack<(int, int)> stack = new Stack<(int, int)>();
        stack.Push((startX, startY));

        while (stack.Count > 0)
        {
            var (x, y) = stack.Pop();

            if (x >= 0 && y >= 0 && x < rows && y < cols && a[x, y] == ' ')
            {
                count2++;
                a[x, y] = '+';

                stack.Push((x + 1, y));
                stack.Push((x, y + 1));
                stack.Push((x - 1, y));
                stack.Push((x, y - 1));
            }
        }
    }

    static void Main(string[] args)
    {
        int rows = 20;
        int cols = 20;

        char[,] charMatrix = new char[rows, cols];

        FillCharMatrix(charMatrix, 40);

        int startX = 2;
        int startY = 2;

        FillUsingStack(startX, startY, charMatrix);

        for (int i = 0; i < rows; i++)
        {
            for (int j = 0; j < cols; j++)
            {
                Console.Write(charMatrix[i, j].ToString() + " ");
            }
            Console.WriteLine();
        }
        Console.WriteLine("\nCount 1 = " + count1 + " Count 2 = " + count2 + "\n");
    }
}
