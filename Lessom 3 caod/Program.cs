namespace Lessom_3_caod
{
    internal class Program
    {
        static void Main(string[] args)
        {
            Random random = new Random();
            CustomList<int> list = new CustomList<int>();
            for (int i = 0; i < 100; i++)
            {
                list.Add(random.Next());
            }
            foreach (int i in list)
            {
                Console.WriteLine(i.ToString());
            }
        }
    }
}