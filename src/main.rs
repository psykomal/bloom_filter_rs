//! Toy Bloom Filter implementation in Rust

use std::hash::{Hash, Hasher, BuildHasher};
use std::collections::hash_map::{DefaultHasher, RandomState};


/// Generate N hashers using RandomState
fn random_hashers(num_hashers: usize) -> Vec<DefaultHasher> {
    (0..num_hashers)
        .map(|_| {
            RandomState::new().build_hasher()
        })
        .collect()
}

/// Bloom filter is a space-efficient probabilistic data structure. \
/// Refer <https://en.wikipedia.org/wiki/Bloom_filter>
#[allow(dead_code)]
struct BloomFilter {
    prob_fp: f64,
    data_set_size: usize,
    vector_len: usize,   // optimal vector len computed  
    num_hashers: usize,   // optimal number of hasers
    
    bitvec: Vec<bool>,     // Using simple vector. Can use bit_vec crate instead
    hash_funcs: Vec<DefaultHasher>   // SipHasher is deprecated
}


impl BloomFilter {

    /// Create new bloom filter given  \
    /// prob_fp : Max Tolerable Probability of False Positive  \
    /// data_set_size : Estimated Max Set Size
    fn new(prob_fp: f64, data_set_size: usize) -> Self {

        let optimal_vector_len = Self::get_optimal_vector_len(prob_fp, data_set_size);
        let optimal_num_hashes = Self::get_optimal_num_hashes(prob_fp);


        BloomFilter {
            prob_fp: prob_fp,
            data_set_size: data_set_size,
            vector_len: optimal_vector_len,
            num_hashers: optimal_num_hashes,

            bitvec: vec![false; optimal_vector_len],
            hash_funcs: random_hashers(optimal_num_hashes)
        }
    }

    /// Allows addition of data of any type that implements Hash trait
    fn add<T: Hash>(&mut self, data: T) -> () {

        for i in 0..self.num_hashers {
            let mut hasher = self.hash_funcs[i].clone();
            data.hash(&mut hasher);
            let hash_val = hasher.finish() as usize;

            let index = hash_val % self.vector_len;
            // println!("add {}", index);
            self.bitvec[index] = true;
        }
    }
 
    /// Checks whether data is present or not \
    /// - if False, data is not present with 100% probability \
    /// - if True, data might or might not be present (Can be a false postiive)
    fn contains<T: Hash>(&mut self, data: T) -> bool {

        for i in 0..self.num_hashers {
            let mut hasher = self.hash_funcs[i].clone();
            data.hash(&mut hasher);
            let hash_val = hasher.finish() as usize;

            let index = hash_val % self.vector_len;
            // println!("contains {}", index);
            if self.bitvec[index] != true {
                return false;
            }
        }
        
        true
    }

    fn get_optimal_num_hashes(prob_fp: f64) -> usize {
        let ln_2 = f64::ln(2.0);
        let ln_prob_fp = f64::ln(prob_fp);
        f64::ceil(-(ln_prob_fp/ln_2)) as usize
    }

    fn get_optimal_vector_len(prob_fp: f64, data_set_size: usize) -> usize {
        let ln_2 = f64::ln(2.0);
        let ln_prob_fp = f64::ln(prob_fp);
        f64::ceil(-(((data_set_size as f64) * ln_prob_fp)/(ln_2.powi(2)))) as usize
    }
}


fn main() {

    let mut bloom_filter = BloomFilter::new(0.5, 100);
    println!("Vector Length : {} \nNum Hashes: {} \n", bloom_filter.vector_len, bloom_filter.num_hashers);

    let animals = [
        "dog","cat","giraffe","fly","mosquito","horse","eagle","bird","bison","boar","butterfly","ant","anaconda","bear","chicken","dolphin","donkey","crow","crocodile",
    ];

    let other_animals = [
        "badger","cow","pig","sheep","bee","wolf","fox","whale","shark","fish","turkey","duck","dove","deer","elephant","frog","falcon","goat","gorilla","hawk",
    ];


    for animal in animals {
        bloom_filter.add(animal)
    }

    for animal in animals {
        if bloom_filter.contains(animal) {
            println!("\"{}\" is PROBABLY IN the filter.", animal);
        } 
        else {
            println!("\"{}\" is DEFINITELY NOT IN the filter as expected.", animal);
        }
    }

    for animal in other_animals {
        if bloom_filter.contains(animal) {
            println!("\"{}\" is a FALSE POSITIVE case (please adjust prob_fp to a smaller value).", animal);
        } 
        else {
            println!("\"{}\" is DEFINITELY NOT IN the filter as expected.", animal);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let mut bloom_filter = BloomFilter::new(0.001, 100);
        bloom_filter.add("cat");
        assert!(bloom_filter.contains("cat"));
    }

    #[test]
    fn simple_test_2() {
        let mut bloom_filter = BloomFilter::new(0.01, 100);
        assert!(!bloom_filter.contains("cat"));
        assert!(!bloom_filter.contains("dog"));
        bloom_filter.add(String::from("cat"));
        bloom_filter.add("dog");
        bloom_filter.add("komal");
        bloom_filter.add("animal");
        assert!(bloom_filter.contains(String::from("cat")));
        assert!(bloom_filter.contains("dog"));
        assert!(!bloom_filter.contains("monkey"));
        assert!(bloom_filter.contains("komal"));
        assert!(!bloom_filter.contains("fox"));
        assert!(bloom_filter.contains("animal"));
    }

}
