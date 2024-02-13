using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace lesson4
{
    
        interface ISLList<T>
        {
            void PushBack(T value); // добавляет элемент в конец списка;
            void PushFront(T value); // добавляет элемент в начало списка;
            void Insert(int index, T value); // добавляет элемент по индексу;
            void PopBack(); // удаляет последний элемент в списке;
            void PopFront(); // удаляет первый элемент в списке;
            void RemoveAt(int index); // удаляет элемент в списке по указанному индексу;
            T this[int index] { set; get; } // для записи/чтения элемента по указанному индексу;
            int Count { get; } // возвращает количество элементов в~списке;
            bool Empty(); // отвечает на вопрос пустой ли список;
            void Clear(); // очищает список, удаляет все узлы;
            T First(); // возвращает первый элемент списка;
            T Last(); // возвращает последний элемент списка.
        }
    
}
