using System.ComponentModel;
using System.Data;

namespace Binary_Search_Tree_lesson_6
{
    internal class Program
    {
        static void Main(string[] args)
        {
            DateTime currentTime = DateTime.Now;
            BinarySearchTree<int> tree = new BinarySearchTree<int>();
            for (int i = 0; i < 10; i++)
            {
                int valuie = (int.Parse(i.ToString() + i.GetHashCode().ToString()) 
                    * currentTime.Ticks.GetHashCode() / 1466) % 100 ;
                tree.Insert(valuie);
            }
            tree.Insert(3);
            tree.InOrderTraversal();

            int keyToSearch = 10;
            tree.Search(keyToSearch);

            int searchValue = 3;
            bool found = tree.Contains(searchValue);

            if (found)
            {
                Console.WriteLine($"\n \n Дерево содержит значение {searchValue}.");
            }
            else
            {
                Console.WriteLine($"\n \nДерево не содержит значение {searchValue}.");
            }
        }
    }
}