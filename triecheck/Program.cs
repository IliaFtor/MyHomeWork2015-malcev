using System;
using System.Collections.Generic;
using System.Linq;
using System.IO;
using System.Text;

namespace saod_9
{
	using System;
	using System.Collections.Generic;

	public class TrieNode
	{
		public char Symbol { get; set; }
		public Dictionary<char, TrieNode> Childs { get; set; }
		public bool IsEndOfWord { get; set; }
		public int OccurrenceCount { get; set; }

		public TrieNode(char symbol)
		{
			Symbol = symbol;
			Childs = new Dictionary<char, TrieNode>();
			IsEndOfWord = false;
			OccurrenceCount = 0;
		}
	}

	public class Trie : ITrie
	{
		private TrieNode root;
		
		public int Count;

		public Trie()
		{
			root = new TrieNode('^');
		}

		public void Insert(string word)
		{
			TrieNode currentNode = root;

			foreach(char c in word)
			{
				if(!currentNode.Childs.ContainsKey(c))
				{
					currentNode.Childs[c] = new TrieNode(c);
				}

				currentNode = currentNode.Childs[c];
			}

			currentNode.IsEndOfWord = true;
			currentNode.OccurrenceCount++;
			++Count;
		}

		public int LeafCount
		{
			get
			{
				return CountLeaves(root);
			}
		}

		private int CountLeaves(TrieNode node)
		{
			if(node == null)
			{
				return 0;
			}

			int count = 0;

			foreach(var child in node.Childs.Values)
			{
				count += CountLeaves(child);
			}

			return node.IsEndOfWord ? count + 1 : count;
		}

		public int InternalNodeCount
		{
			get
			{
				return CountInternalNodes(root);
			}
		}

		private int CountInternalNodes(TrieNode node)
		{
			if(node == null)
			{
				return 0;
			}

			int count = 0;

			foreach(var child in node.Childs.Values)
			{
				count += CountInternalNodes(child);
			}

			return count + (node.Childs.Count > 0 ? 1 : 0);
		}

		public int SamePrefixCount(string prefix)
		{
			TrieNode lastNode = FindLastNode(prefix);

			return lastNode != null ? lastNode.OccurrenceCount : 0;
		}

		public List<string> SamePrefixWords(string prefix)
		{
			List<string> words = new List<string>();
			TrieNode lastNode = FindLastNode(prefix);

			if(lastNode != null)
			{
				CollectWordsWithPrefix(lastNode, prefix, words);
			}

			return words;
		}

		private TrieNode FindLastNode(string prefix)
		{
			TrieNode currentNode = root;

			foreach(char c in prefix)
			{
				if(currentNode.Childs.ContainsKey(c))
				{
					currentNode = currentNode.Childs[c];
				}
				else
				{
					return null;
				}
			}

			return currentNode;
		}
		
		public bool Contains(string prefix)
		{
			TrieNode currentNode = root;

			foreach(char c in prefix)
			{
				if(currentNode.Childs.ContainsKey(c))
				{
					currentNode = currentNode.Childs[c];
				}
				else
				{
					return false;
				}
			}

			return currentNode.IsEndOfWord;
		}
		
		public int GetCount(string prefix)
		{
			TrieNode currentNode = root;

			foreach(char c in prefix)
			{
				if(currentNode.Childs.ContainsKey(c))
				{
					currentNode = currentNode.Childs[c];
				}
				else
				{
					return 0;
				}
			}

			return currentNode.OccurrenceCount;
		}
		
		public void Increment(string prefix)
		{
			TrieNode currentNode = root;

			foreach(char c in prefix)
			{
				if(currentNode.Childs.ContainsKey(c))
				{
					currentNode = currentNode.Childs[c];
				}
				else
				{
					return;
				}
			}

			++currentNode.OccurrenceCount;
		}

		private void CollectWordsWithPrefix(TrieNode node, string currentWord, List<string> words)
		{
			if(node.IsEndOfWord)
			{
				words.Add(currentWord);
			}

			foreach(var child in node.Childs.Values)
			{
				CollectWordsWithPrefix(child, currentWord + child.Symbol, words);
			}
		}
	}

	public interface ITrie
	{
		int LeafCount { get; }
		int InternalNodeCount { get; }
		int SamePrefixCount(string prefix);
		List<string> SamePrefixWords(string prefix);
	}

	class Program
	{
		public static void fill_rbtree(string word)
		{
			//var text = "are they the most fun and these are a fun";

			var text = File.ReadAllText("big.txt");

			var dict = new SortedDictionary<string, int>();
			//var dict = new MyAVLContainer<string, int>();
			string str = "";
			foreach(var ch in text) // кроме последнего слова
			{
				//if(isalpha(ch) || ch == '\'')
				if((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '\'')
					str += ch;
				else if(str.Length > 0)
				{
					/*try
					{
						++dict[str];
					}
					catch (KeyNotFoundException)
					{
						dict.Add(str, 1);
					}*/
					if(dict.ContainsKey(str))
					{
						dict[str] = dict[str] + 1; // метод increase value
					}
					else
					{
						dict.Add(str, 1);
					}
					str = "";
				}
			}
			Console.WriteLine("\ndict size: " + dict.Count);
			Console.WriteLine(word + ": " + dict[word]);
		}


		public static void fill_trie(string word)
		{
			//var text = "are they the most fun and these are a fun";

			var text = File.ReadAllText("big.txt");

			Trie dict = new Trie();
			//var dict = new MyAVLContainer<string, int>();
			string str = "";
			foreach(var ch in text) // кроме последнего слова
			{
				//if(isalpha(ch) || ch == '\'')
				if((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '\'')
					str += ch;
				else if(str.Length > 0)
				{

					if(dict.Contains(str))
					{
						dict.Increment(str); // метод increase value
					}
					else
					{
						dict.Insert(str);
					}
					str = "";
				}
			}
			Console.WriteLine("\ndict size: " + dict.Count);
			Console.WriteLine(word + ": " + dict.Contains(word));
			Console.WriteLine(word + ": " + dict.GetCount(word));
			
						Console.WriteLine($"Leaf Count: {dict.LeafCount}");
			Console.WriteLine($"Internal Node Count: {dict.InternalNodeCount}");

			Console.WriteLine($"Words with prefix 'the': {dict.SamePrefixCount("the")}");
			Console.WriteLine($"Words with prefix 'the': {string.Join(", ", dict.SamePrefixWords("the"))}");
		}
		
		static void Main()
		{
			fill_rbtree("the");
			fill_trie("the");
			return;			Trie trie = new Trie();

			Console.WriteLine($"Leaf Count: {trie.LeafCount}");
			Console.WriteLine($"Internal Node Count: {trie.InternalNodeCount}");

			Console.WriteLine($"Words with prefix 'are': {trie.SamePrefixCount("are")}");
			Console.WriteLine($"Words with prefix 'are': {string.Join(", ", trie.SamePrefixWords("are"))}");
		}
	}

}
