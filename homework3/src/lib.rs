mod find_errors {
    use std::collections::HashMap;
    use std::iter::FromIterator;

    pub fn clean_line(input: &str) -> String {
        input.trim().chars().filter(|c| c.is_alphabetic() || c.is_whitespace() || *c == '\'' || *c == '-').collect()
    }
    pub struct WordCounter {
        corpus: HashMap<String, u32>,
    }

    impl WordCounter {
        
        pub fn new() -> Self {
            let corpus = HashMap::new();
            Self{corpus: corpus}
        }

        pub fn from_str(input: &str) -> Self {
            let mut counter = WordCounter::new();
            let lines = input.lines();
            for line in lines {
                let cleaned_line = clean_line(line);
                for word in cleaned_line.split_whitespace() {
                    counter.add(word);
                }
            }
            counter
        }

        pub fn words(&self) -> Vec<&String> {
            let mut vec = Vec::from_iter(self.corpus.keys());
            vec.sort_unstable();
            vec
        }

        pub fn add(&mut self, item: &str) {
            let lowercase = item.to_lowercase();
            let trimmed = lowercase.trim();

            let counter = self.corpus.entry(trimmed.to_string()).or_insert(0);
            *counter += 1;
            
        }

        pub fn get(&self, word: &str) -> u32 {
            let value = self.corpus.get(&word.to_string());
            *value.unwrap_or(&0)
        }

        pub fn total_count(&self) -> u32 {
            self.corpus.values().sum()
        }
    }

    impl std::fmt::Display for WordCounter {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "WordCounter, total count: {}\n", self.total_count())?;
            let mut entries: Vec<(&String, &u32)> = self.corpus.iter().collect();
            entries.sort_unstable_by(|a, b| b.1.cmp(a.1));
            for entry in entries {
                write!(f, "{}: {}\n", entry.0, entry.1)?;
            }
            Ok(())
        }
    }

    use std::collections::HashSet;

    /// Тези две константи са за удобство -- ще ги използваме в тестовете, свободни сте да ги
    /// използвате във вашите.
    pub const ALPHABET_EN: &'static str = "abcdefghijklmnopqrstuvwxyz";
    pub const ALPHABET_BG: &'static str = "абвгдежзийклмнопрстуфхцчшщъьюя";

    pub struct SpellChecker {
        corpus: WordCounter,
        alphabet: String,
    }

    impl SpellChecker {
        
        pub fn new(corpus: &str, alphabet: &str) -> Self {
            Self{corpus: WordCounter::from_str(corpus), alphabet: alphabet.to_string()}
        }

        pub fn correction(&self, word: &str) -> String {
            self.candidates(word)[0].clone()
        }

        pub fn probability(&self, word: &str) -> f64 {
            let occurances = self.corpus.get(word);
            let total = self.corpus.total_count();
            occurances as f64 / total as f64
        }

        pub fn known<'a> (&self, words: &'a HashSet<String>) -> Vec<&'a String> {
        words.into_iter().filter(|w| self.corpus.get(w) > 0).collect()
        }
        
        pub fn candidates(&self, word: &str) -> Vec<String> {
            if self.corpus.get(word) > 0 {
                vec![word.to_string()]
            } else {
                let p = self.edits1(word);
                let candidates_1 = self.known(&p);
                if !candidates_1.is_empty() {
                    candidates_1.iter().map(|&s| s.clone()).collect()
                } else {
                    let q = self.edits2(word);
                    let candidates_2 = self.known(&q); 
                    if !candidates_2.is_empty() {
                        candidates_2.iter().map(|&s| s.clone()).collect()
                    } else {
                        vec![word.to_string()]
                    }
                }
            }

        }

        pub fn edits1(&self, word: &str) -> HashSet<String> {
            let mut result = HashSet::new();
            for i in 0.. word.len() {
                let (first, last) = word.split_at(i);
                result.insert([first, &last[1..]].concat());
            }
            for i in 0 .. word.len() - 1 {
                let (first, last) = word.split_at(i);
                result.insert([first, &last[1..2], &last[..1], &last[2..]].concat());
            }
            for i in 0 .. word.len() + 1 {
                for c in self.alphabet.chars() {
                    let (first, last) = word.split_at(i);
                    let mut buffer = [0; 1];
                    let res = c.encode_utf8(&mut buffer);
                    result.insert([first, res, last].concat());
                }
            }
            for i in 0 .. word.len() {
                for c in self.alphabet.chars() {
                    let (first, last) = word.split_at(i);
                    let mut buffer = [0; 1];
                    let res = c.encode_utf8(&mut buffer);
                    result.insert([first, res, &last[1..]].concat());
                }
            }
            result
        }

        pub fn edits2(&self, word: &str) -> HashSet<String> {
            self.edits1(word).iter().flat_map(|word_edit_1| self.edits1(word_edit_1)).collect()
        }
    }
}

mod correct_errors {
    pub struct WordIterator<'a> {
        source: &'a str,
        current_index: usize,
        current_type: SegmentType,
    }

    impl<'a> WordIterator<'a> {
        pub fn new(source: &'a str) -> Self {
            let first_char = source.chars().next();
            let current_type = if first_char.is_none() || WordIterator::word_char(first_char.unwrap()) {
                SegmentType::Word           
            } else {
                SegmentType::NonWord
            };
            Self{source: source, current_index: 0, current_type: current_type}
        }

        fn segment<F: Fn(char) -> bool>(&'a self, next_segment: F) -> &'a str {
            if let Some(next_index) = self.source[self.current_index..].find(next_segment)  {
                &self.source[self.current_index..self.current_index + next_index]
            } else {
                &self.source[self.current_index..]
            }
        }

        fn word_char(c: char) -> bool {
            c.is_alphabetic() || c == '\'' || c == '-'
        }

    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum SegmentType {
        Word,
        NonWord,
    }

    impl<'a> Iterator for WordIterator<'a> {
        type Item = (&'a str, SegmentType);
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.current_index >= self.source.len() {
                None
            } else {
                let segment = match self.current_type {
                    SegmentType::Word => self.segment(WordIterator::word_char),
                    //self.segment(|c| !WordIterator::word_char(c)),
                    
                    SegmentType::NonWord => self.segment(WordIterator::word_char),
                    
                };
                self.current_index += segment.len();
                let res_type = self.current_type;
                self.current_type = if res_type == SegmentType::Word {
                    SegmentType::NonWord
                } else {
                    SegmentType::Word
                };
                Some((segment, res_type))
            }
        }
    }

    pub fn replace_words<F: Fn(&str) -> String>(input: &str, f: F) -> String {
        let wi = WordIterator::new(input);
        wi.map(|el| 
            {
                println!("{}", el.0);
                if el.1 == SegmentType::Word {
                    f(el.0)
                } else {
                    el.0.to_string()
                }
            }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::find_errors;
    use super::find_errors::SpellChecker;
    use super::correct_errors;

    #[test]
    fn it_works() {
        let spell_checker = SpellChecker::new("cec abc warm", find_errors::ALPHABET_EN);
        
        let result = correct_errors::replace_words("werm7-12abc 45 ", |word| spell_checker.correction(word));
        assert_eq!("warm7-12abc 45 ", result);
    }
}
