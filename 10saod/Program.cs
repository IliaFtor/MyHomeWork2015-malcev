namespace _10saod
{
    public class Abonent
    {
        public string Name { get; set; }
        public string PhoneNumber { get; set; }
        public DateTime BirthDate { get; set; }

        public Abonent(string name, string phoneNumber, DateTime birthDate)
        {
            Name = name;
            PhoneNumber = phoneNumber;
            BirthDate = birthDate;
        }
    }

    public class MyHashTable<K, V>
    {
        private List<LinkedList<KeyValuePair<K, V>>> items;

        public MyHashTable(int size)
        {
            items = Enumerable.Repeat(new LinkedList<KeyValuePair<K, V>>(), size)
                              .ToList();
        }

        protected int GetArrayPosition(K key)
        {
            int position = key.GetHashCode() % items.Count;
            return Math.Abs(position);
        }

        public V Find(K key)
        {
            int position = GetArrayPosition(key);
            LinkedList<KeyValuePair<K, V>> linkedList = GetLinkedList(position);
            foreach (KeyValuePair<K, V> pair in linkedList)
            {
                if (pair.Key.Equals(key))
                {
                    return pair.Value;
                }
            }

            return default(V);
        }

        public void Add(K key, V value)
        {
            int position = GetArrayPosition(key);
            LinkedList<KeyValuePair<K, V>> linkedList = GetLinkedList(position);
            KeyValuePair<K, V> item = new KeyValuePair<K, V>(key, value);
            linkedList.AddLast(item);
        }

        protected LinkedList<KeyValuePair<K, V>> GetLinkedList(int position)
        {
            LinkedList<KeyValuePair<K, V>> linkedList = items[position];
            if (linkedList == null)
            {
                linkedList = new LinkedList<KeyValuePair<K, V>>();
                items[position] = linkedList;
            }

            return linkedList;
        }
    }

    class Program
    {
        static void Main()
        {
            MyHashTable<string, Abonent> abonentTable = new MyHashTable<string, Abonent>(100);

            // Пример добавления абонента в хэш-таблицу
            Abonent abonent1 = new Abonent("John Doe", "123-456-7890", new DateTime(1990, 1, 1));
            abonentTable.Add(abonent1.Name, abonent1);

            // Пример поиска абонента по имени
            Abonent foundAbonent = abonentTable.Find("John Doe");
            if (foundAbonent != null)
            {
                Console.WriteLine($"Found Abonent: {foundAbonent.Name}, Phone: {foundAbonent.PhoneNumber}, BirthDate: {foundAbonent.BirthDate}");
            }
            else
            {
                Console.WriteLine("Abonent not found.");
            }
        }
    }


}