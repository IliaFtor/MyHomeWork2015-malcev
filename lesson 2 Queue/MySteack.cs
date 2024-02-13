using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace lesson_2_Queue
{
class Stack<T>
    {
        private T[] items;
        private int top;
        private int capacity;

        public Stack(int capacity)
        {
            if (capacity <= 0)
            {
                throw new Exception("Capacity");
            }

            this.capacity = capacity;
            this.items = new T[capacity];
            this.top = -1;
        }

        public void Push(T item)
        {
            if (top == capacity - 1)
            {
                throw new Exception("is full.");
            }

            items[++top] = item;
        }

        public T Pop()
        {
            if (IsEmpty())
            {
                throw new Exception("is empty.");
            }

            return items[top--];
        }

        public T Peek()
        {
            if (IsEmpty())
            {
                throw new Exception("Stack is empty.");
            }

            return items[top];
        }
        public bool IsEmpty()
        {
            return top == -1;
        }
    
    }
}
