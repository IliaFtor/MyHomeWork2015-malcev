namespace lesson_1_saod
{
    internal class Program
    {
        // коунтер для заполнения замену и кол-пробелов до и после запонения
        public static uint count1 = 0,count2 = 0;
        static void FillCharMatrix(char[,] matrix, int percentPluses)
        {
            if (percentPluses < 0 || percentPluses > 100)
            {
                throw new Exception("Значение процента плюсов должно быть в диапазоне от 0 до 100.");
            }

            Random rand = new Random();
            int rows = matrix.GetLength(0);
            int cols = matrix.GetLength(1);
            int numPluses = (int)Math.Round(rows * cols * percentPluses / 100.0);
            for (int i = 0; i < rows; i++)
            {
                for (int j = 0; j < cols; j++)
                {
                    matrix[i, j] = ' ';               
                }
            }
            for (int i = 0; i < numPluses; i++)
            {
                int randomRow = rand.Next(rows);
                int randomCol = rand.Next(cols);
                if (matrix[randomRow, randomCol] != '+')
                {
                    matrix[randomRow, randomCol] = '+';
                    count1++;
                }
                else
                {
                    i--;
                }
            }
        }
        static void fill(int x, int y, char[,] a)
        {
            int rows = a.GetLength(0);
            int cols = a.GetLength(1);

            if (x >= 0 && y >= 0 && x < rows && y < cols && a[x, y] == ' ')
            {
                count2++;
                a[x, y] = '+';
                fill(x + 1, y, a);
                fill(x, y + 1, a);
                fill(x - 1, y, a);
                fill(x, y - 1, a);
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

            fill(startX, startY, charMatrix);

            for (int i = 0; i < rows; i++)
            {
                for (int j = 0; j < cols; j++)
                {
                    Console.Write(charMatrix[i, j].ToString() + " ");
                }
                Console.WriteLine();
            }
            Console.WriteLine("\nCount 1 = " + count1 + " Count 1 = " + count2 + "\n");
        }
    }
}