using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Binary_Search_Tree_lesson_6
{
    internal class BinarySearchTree<T>
    {
        private Node<T> root;
        public BinarySearchTree()
        {
            root = null;
        }

        public void Insert(T key)
        {
            root = InsertRec(root, key);
        }
        public bool Contains(T item)
        {
            return SearchRec(root, item);
        }
        private bool SearchRec(Node<T> root, T key)
        {
            if (root == null)
            {
                return false;
            }

            if (Comparer<T>.Default.Compare(key, root.Key) == 0)
            {
                return true;
            }

            if (Comparer<T>.Default.Compare(key, root.Key) < 0)
            {
                return SearchRec(root.Left, key);
            }
             return SearchRec(root.Right, key);
        }

        private Node<T> InsertRec(Node<T> root, T key)
        {
            if (root == null)
            {
                root = new Node<T>(key);
                return root;
            }

            if (Comparer<T>.Default.Compare(key, root.Key) < 0)
            {
                root.Left = InsertRec(root.Left, key);
            }
            else if (Comparer<T>.Default.Compare(key, root.Key) > 0)
            {
                root.Right = InsertRec(root.Right, key);
            }

            return root;
        }

        public void InOrderTraversal()
        {
            InOrderTraversal(root);
        }

        private void InOrderTraversal(Node<T> root, string indent = "", bool isRight = false)
        {
            if (root != null)
            {
                Console.WriteLine(indent + (isRight ? "R-" : "L-") + root.Key);
                InOrderTraversal(root.Left, indent + (isRight ? "|  " : "    "));
                InOrderTraversal(root.Right, indent + (isRight ? "|  " : "    "), true);
            }
        }

        public bool Search(T key)
        {
            return SearchRec(root, key);
        }

    }
}
