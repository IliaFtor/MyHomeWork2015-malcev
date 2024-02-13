using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Lessom_3_caod
{
    public class CustomList<T> : IEnumerable<T>
    {
        private T[] items;
        private int capacity;
        private int count;

        public CustomList()
        {
            capacity = 4;
            items = new T[capacity];
            count = 0;
        }

        public int Count => count;

        public void Add(T item)
        {
            if (count == capacity)
            {
                capacity *= 2;
                Array.Resize(ref items, capacity);
            }

            items[count] = item;
            count++;
        }

        public void RemoveAt(int index)
        {
            if (index < 0 || index >= count)
                throw new Exception(nameof(index));

            for (int i = index; i < count - 1; i++)
            {
                items[i] = items[i + 1];
            }

            count--;
        }

        public T this[int index]
        {
            get => (index < 0 || index >= count) ? throw new Exception(nameof(index)) : items[index];
            set
            {
                if (index < 0 || index >= count)
                    throw new Exception(nameof(index));

                items[index] = value;
            }
        }
        public IEnumerator<T> GetEnumerator()
        {
            for(int i = 0; i < count; i++)
                yield return items[i];
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return GetEnumerator();
        }
    }
    
}
