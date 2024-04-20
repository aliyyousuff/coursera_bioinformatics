/* 
Code Challenge: Implement PatternCount (reproduced below).
     Input: Strings Text and Pattern.
     Output: Count(Text, Pattern).

Sample Input:
GCGCG
GCG
Sample Output:
2

PatternCount(Text, Pattern)
  count ← 0
  for i ← 0 to |Text| − |Pattern|
    if Text(i, |Pattern|) = Pattern
      count ← count + 1
  return count

  */

pub fn pattern_count(text: &str, pattern: &str) -> usize 
 {   
    let mut count: usize = 0;
    let text_len = text.len();
    let pat_len = pattern.len();
     for i in 0..=(text_len - pat_len)
     {     
        //println!("{}", &text[i..pat_len+i]);
        if *&text[i..pat_len+i] == *pattern
        {
            count = count +1;
        }
     }
     count 
}

