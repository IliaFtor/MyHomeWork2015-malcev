
namespace lesson_5_Saod
{
    internal class Program
    {
        static void Main(string[] args)
        {
            System.Diagnostics.Stopwatch watch;

            DoubleLList<int> dl = new DoubleLList<int>();
            LinkedList<int> l = new LinkedList<int>();
            Random rnd = new Random(1);

            long elapsedMs;
            int n = 20000;

            int[] A = new int[n];
            for (int i = 0, count = n; i < n; i++, count++)
                A[i] = rnd.Next(0, count);

            for (int i = 0; i < n; i++)
            {
                dl.Add(i);
                l.PushBack(i);
            }

            watch = System.Diagnostics.Stopwatch.StartNew();
            for (int i = 0; i < n; i++)
            {
                dl.Insert(A[i], -1);
            }
            watch.Stop();
            elapsedMs = watch.ElapsedMilliseconds;
            System.Console.WriteLine("time for duoble linked list: " + elapsedMs);

            watch = System.Diagnostics.Stopwatch.StartNew();
            for (int i = 0; i < n; i++)
            {
                l.Insert(A[i], -1);
            }
            watch.Stop();
            elapsedMs = watch.ElapsedMilliseconds;
            System.Console.WriteLine("time for linked list: " + elapsedMs);
        }
    }
    
}