using System.Collections.Generic;
using System.Xml.Linq;

namespace lesson4
{
    internal class Program
    {
        static void print_lst(LinkedList<char> l)
        {
            for (int i = 0; i < l.Count; i++)
            {
                char value = l[i]; 
                Console.Write(value + " "); 
            }
            Console.WriteLine();
        }
        static void Main(string[] args)
        {
            var lst = new LinkedList<char>(); // ваш список
            Console.WriteLine(lst.Count + " " + lst.Empty());

            for (int i = 0; i < 5; i++)
                lst.PushBack((char)(i + 97));
            print_lst(lst);

            for (int i = 0; i < 5; i++)
                lst.Insert(0, (char)(122 - i));
            print_lst(lst);

            for (int i = 0; i < lst.Count; i++)
                lst[i] = (char)(i + 97); // методы доступа set
            print_lst(lst);

            lst.PopBack();
            lst.PopFront();

            print_lst(lst);

            lst.RemoveAt(5);
            lst.Insert(3, 'o');

            print_lst(lst);

            lst.Clear();

            lst.PushBack('q');
            lst.PushFront('a');
            lst.PushBack('w');

            Console.WriteLine(lst.First() + " " + lst.Last());
            Console.WriteLine(lst.Count + " " + lst.Empty());
        }
    }
}