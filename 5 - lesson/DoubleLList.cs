
using System;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;
using System.Text;
using System.Threading.Tasks;
using System.Xml.Linq;

namespace lesson_5_Saod
{
    public class DoubleLList<T>
    {
        private Node2<T> head;
        private Node2<T> middle;
        private Node2<T> tail;
        private int count;

        public DoubleLList()
        {
        }
        public void Add(T item)
        {
            Node2<T> newNode = new Node2<T>(item);

            if (head == null)
            {
                head = newNode;
                tail = newNode;
            }
            else
            {
                tail.Next = newNode;
                newNode.Prev = tail;
                tail = newNode;
            }

            count++;
        }

        public void Insert(int index, T item)
        {
            if (index < 0 || index > count)
                throw new IndexOutOfRangeException();

            if (index == count)
            {
                Add(item);
                return;
            }

            Node2<T> newNode = new Node2<T>(item);

            if (index == 0)
            {
                newNode.Next = head;
                head.Prev = newNode;
                head = newNode;
            }
            else
            {
                Node2<T> current;

                if (middle != null && index < count / 2)
                {
                    current = middle;
                    middle = middle.Prev;
                }
                else
                {
                    current = tail;
                    if (index < count - 1)
                        tail = tail.Prev;
                }

                Node2<T> prevNode = current.Prev;

                newNode.Prev = prevNode;
                newNode.Next = current;

                prevNode.Next = newNode;
                current.Prev = newNode;
            }

            count++;
        }      
    }
}
