using System.Xml.Linq;

namespace lesson4
{
    internal class Program
    {
        static void Main(string[] args)
        {
            Node<string> firstNode = new Node<string>("Ссылка 1");
            Node<string> secondNode = new Node<string>("Ссылка 2");
            Node<string> serviceNode = new Node<string>("Дополнительный служебный узел");

            firstNode.Next = secondNode;
            secondNode.Next = serviceNode;

            Node<string> currentNode = firstNode;
            while (currentNode != null)
            {
                Console.WriteLine(currentNode.Value);
                currentNode = currentNode.Next;
            }
        }
    }
}