/*

The pseudocode for FrequentWords is shown below.

FrequentWords(Text, k)
    FrequentPatterns ← an empty set
    for i ← 0 to |Text| − k
        Pattern ← the k-mer Text(i, k)
        Count(i) ← PatternCount(Text, Pattern)
    maxCount ← maximum value in array Count
    for i ← 0 to |Text| − k
        if Count(i) = maxCount
            add Text(i, k) to FrequentPatterns
    remove duplicates from FrequentPatterns
    return FrequentPatterns

 */



use std::collections::HashSet;
use crate::course_01;

pub fn frequent_words(text: &str, kmer: usize) -> HashSet<&str>
{
    let text_len = text.len();
    let mut count: Vec<usize> = Vec::new(); 

    let mut frequent_pattern: HashSet<&str> = HashSet::new();

    for i in 0 ..= ( text_len - kmer )
    {
        let pattern: &str = &text[i..kmer+i];
        count.push(course_01::pattern_count::pattern_count(&text, &pattern));
    }

    let max_count: usize = *count.iter().max().unwrap();

    for i in 0 ..=( text_len - kmer )
    {
        if count[i] == max_count
        {
            frequent_pattern.insert(&text[i..kmer+i]);
        }
    }

    frequent_pattern
}