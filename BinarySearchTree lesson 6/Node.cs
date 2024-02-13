using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Binary_Search_Tree_lesson_6
{
    internal class Node<T>
    {
        public T Key;
        public Node<T> Left;
        public Node<T> Right;

        public Node(T key)
        {
            Key = key;
            Left = null;
            Right = null;
        }
    }
}
