using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

class Program
{
    static void Main()
    {
        List<string> wordsToCheck = new List<string>(File.ReadAllLines("check.txt"));
        List<string> wordList = new List<string>();
        SortedList<string, int> wordCounts = new SortedList<string, int>();

        foreach (string word in wordsToCheck)
        {
            wordList.Add(word.ToLower());
        }

        string text = File.ReadAllText("big.txt");
        string[] wordsInText = text.Split(new[] { ' ', '\t', '\n', '\r', '.', ',', ';', ':', '!', '?' }, StringSplitOptions.RemoveEmptyEntries);

        foreach (string wordInText in wordsInText)
        {
            string cleanWord = CleanWord(wordInText);
            if (!string.IsNullOrEmpty(cleanWord) && wordList.Contains(cleanWord))
            {
                if (wordCounts.TryGetValue(cleanWord, out int count))
                {
                    wordCounts[cleanWord] = count + 1;
                }
                else
                {
                    wordCounts[cleanWord] = 1;
                }
            }
        }

        foreach (var kvp in wordCounts)
        {
            Console.WriteLine($"{kvp.Key}: {kvp.Value}");
        }

        Console.WriteLine($"Total unique words: {wordCounts.Count}");

        Console.Write("Enter a word to find its occurrences: ");
        string userWordInput = Console.ReadLine().ToLower();
        int userWordCount = wordCounts.ContainsKey(userWordInput) ? wordCounts[userWordInput] : 0;
        Console.WriteLine($"Occurrences of the word '{userWordInput}' in big.txt: {userWordCount}");
    }

    static string CleanWord(string word)
    {
        return new string(word.ToLower().Where(c => char.IsLetter(c) || c == '\'').ToArray());
    }
}