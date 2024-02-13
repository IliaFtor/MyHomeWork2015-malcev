using System.Text.RegularExpressions;
namespace AVL_serch
{
    class Program
    {
        class Node
        {
            public string Word { get; set; }
            public int Count { get; set; }
            public int Height { get; set; }
            public Node Left { get; set; }
            public Node Right { get; set; }
        }

        static int Height(Node node)
        {
            return node == null ? -1 : node.Height;
        }

        static int Max(int a, int b)
        {
            return a > b ? a : b;
        }

        static Node RotateRight(Node y)
        {
            if (y == null || y.Left == null)
            {
                return y;
            }

            Node x = y.Left;
            Node T = x.Right;

            x.Right = y;
            y.Left = T;

            // Обновление высоты
            y.Height = Max(Height(y.Left), Height(y.Right)) + 1;
            x.Height = Max(Height(x.Left), Height(x.Right)) + 1;

            return x;
        }

        static Node RotateLeft(Node x)
        {
            if (x == null || x.Right == null)
            {
                return x;
            }

            Node y = x.Right;
            Node T = y.Left;

            y.Left = x;
            x.Right = T;

            // Обновление высоты
            x.Height = Max(Height(x.Left), Height(x.Right)) + 1;
            y.Height = Max(Height(y.Left), Height(y.Right)) + 1;

            return y;
        }

        static int GetBalance(Node node)
        {
            return node == null ? 0 : Height(node.Left) - Height(node.Right);
        }

        static Node Insert(Node root, string word)
        {
            if (root == null)
            {
                return new Node { Word = word, Count = 1, Height = 0, Left = null, Right = null };
            }

            int cmp = string.Compare(root.Word, word, StringComparison.OrdinalIgnoreCase);

            if (cmp < 0)
            {
                root.Left = Insert(root.Left, word);
            }
            else if (cmp > 0)
            {
                root.Right = Insert(root.Right, word);
            }
            else
            {
                root.Count++;
                return root;
            }

            // Обновление высоты текущего узла
            root.Height = 1 + Max(Height(root.Left), Height(root.Right));

            // Получение баланса текущего узла и выполнение соответствующих вращений
            int balance = GetBalance(root);

            // Вращение вправо
            if (balance > 1 && string.Compare(word, root.Left.Word, StringComparison.OrdinalIgnoreCase) < 0)
                return RotateRight(root);

            // Вращение влево
            if (balance < -1 && string.Compare(word, root.Right.Word, StringComparison.OrdinalIgnoreCase) > 0)
                return RotateLeft(root);

            // Вращение влево-вправо
            if (balance > 1 && string.Compare(word, root.Left.Word, StringComparison.OrdinalIgnoreCase) > 0)
            {
                root.Left = RotateLeft(root.Left);
                return RotateRight(root);
            }

            // Вращение вправо-влево
            if (balance < -1 && string.Compare(word, root.Right.Word, StringComparison.OrdinalIgnoreCase) < 0)
            {
                root.Right = RotateRight(root.Right);
                return RotateLeft(root);
            }

            return root;
        }

        static int Find(Node root, string word)
        {
            if (root == null)
            {
                return 0;
            }

            int cmp = string.Compare(root.Word, word, StringComparison.OrdinalIgnoreCase);

            if (cmp < 0)
            {
                return Find(root.Left, word);
            }
            else if (cmp > 0)
            {
                return Find(root.Right, word);
            }
            else
            {
                return root.Count;
            }
        }

        static void Main()
        {
            Node avlTree = null;

            foreach (string line in File.ReadLines("check.txt"))
            {
                string word = Regex.Replace(line, "[^a-zA-Z']", "").Trim();
                if (!string.IsNullOrEmpty(word))
                {
                    avlTree = Insert(avlTree, word);
                }
            }

            Dictionary<string, int> wordCount = new Dictionary<string, int>();
            List<char> buffer = new List<char>();

            using (StreamReader reader = new StreamReader("big.txt"))
            {
                int nextChar;
                while ((nextChar = reader.Read()) != -1)
                {
                    char c = (char)nextChar;
                    if (Regex.IsMatch(c.ToString(), "[a-zA-Z']"))
                    {
                        buffer.Add(c);
                    }
                    else
                    {
                        string word = new string(buffer.ToArray());
                        if (!string.IsNullOrEmpty(word))
                        {
                            int count = Find(avlTree, word);
                            wordCount[word] = wordCount.ContainsKey(word) ? wordCount[word] + count : count;
                        }
                        buffer.Clear();
                    }
                }
            }

            foreach (var pair in wordCount)
            {
                Console.WriteLine($"{pair.Key}: {pair.Value}");
            }
        }
    }
    
}