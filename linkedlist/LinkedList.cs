using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace lesson4
{
    public class LinkedList<T>:IEnumerable<T>, ISLList<T>
    {
        private Node<T> head;
        private int count;

        public LinkedList()
        {
            Node<T> firstNode = new Node<T>(default(T)); 
            Node<T> secondNode = new Node<T>(default(T)); 

            firstNode.Next = secondNode;

            head = firstNode;
        }
        public void PushBack(T value)
        {
            Node<T> newNode = new Node<T>(value);
            if (head == null)
            {
                head = newNode;
            }
            else
            {
                Node<T> current = head;
                while (current.Next != null)
                {
                    current = current.Next;
                }
                current.Next = newNode;
            }
            count++;
        }

        public void PushFront(T value)
        {
            Node<T> newNode = new Node<T>(value);
            newNode.Next = head;
            head = newNode;
            count++;
        }

        public void Insert(int index, T value)
        {
            if (index < 0 || index > count)
            {
                throw new Exception(nameof(index));
            }

            if (index == 0)
            {
                PushFront(value);
            }
            else if (index == count)
            {
                PushBack(value);
            }
            else
            {
                Node<T> newNode = new Node<T>(value);
                Node<T> current = head;
                for (int i = 0; i < index - 1; i++)
                {
                    current = current.Next;
                }
                newNode.Next = current.Next;
                current.Next = newNode;
                count++;
            }
        }

        public void PopBack()
        {
            if (count == 0)
            {
                throw new Exception("Список пуст.");
            }

            if (count == 1)
            {
                head = null;
            }
            else
            {
                Node<T> current = head;
                while (current.Next.Next != null)
                {
                    current = current.Next;
                }
                current.Next = null;
            }
            count--;
        }

        public void PopFront()
        {
            if (count == 0)
            {
                throw new Exception("Список пуст.");
            }

            head = head.Next;
            count--;
        }

        public void RemoveAt(int index)
        {
            if (index < 0 || index >= count)
            {
                throw new Exception(nameof(index));
            }

            if (index == 0)
            {
                PopFront();
            }
            else
            {
                Node<T> current = head;
                for (int i = 0; i < index - 1; i++)
                {
                    current = current.Next;
                }
                current.Next = current.Next.Next;
                count--;
            }
        }

        public T this[int index]
        {
            get
            {
                if (index < 0 || index >= count)
                {
                    throw new Exception(nameof(index));
                }

                Node<T> current = head;
                for (int i = 0; i < index; i++)
                {
                    current = current.Next;
                }
                return current.Value;
            }
            set
            {
                if (index < 0 || index >= count)
                {
                    throw new Exception(nameof(index));
                }

                Node<T> current = head;
                for (int i = 0; i < index; i++)
                {
                    current = current.Next;
                }
                current.Value = value;
            }
        }

        public int Count
        {
            get { return count; }
        }

        public bool Empty()
        {
            return count == 0;
        }

        public void Clear()
        {
            head = null;
            count = 0;
        }

        public T First()
        {
            if (count == 0)
            {
                throw new Exception("Список пуст.");
            }
            return head.Value;
        }

        public T Last()
        {
            if (count == 0)
            {
                throw new Exception("Список пуст.");
            }

            Node<T> current = head;
            while (current.Next != null)
            {
                current = current.Next;
            }
            return current.Value;
        }

        public IEnumerator<T> GetEnumerator()
        {
            Node<T> current = head; 
            while (current != null)
            {
                yield return current.Value; 
                current = current.Next;
            }
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return GetEnumerator();
        }
    }
}
