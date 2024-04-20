use std::fs;
use coursera_bioinformatics::course_01;  // build and run with Cargo


fn main() 
{
     let file_name = "/home/yousuf/coursera_bioinformatics/data/test.txt";
     let data = file_reader(&file_name);

    //println!("{}", course_01::pattern_count::pattern_count(&data[0], &data[1]));
    let text: &str = &data[0];

    let kmer: usize = data[1].parse().expect("Not a valid number");

    println!("{:?}", course_01::frequent_words::frequent_words(&text, kmer));
    
    

    
}










fn file_reader(filename: &str) -> Vec<String>
{
    fs::read_to_string(filename)
    .unwrap()
    .lines()
    .map(String::from)
    .collect()
}


