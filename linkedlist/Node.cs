using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace lesson4
{
    internal class Node<T>
    {
            public T Value;
            public Node<T> Next;

            public Node(T value)
            {
                Value = value;
                Next = null;
            }
        
    }
}
