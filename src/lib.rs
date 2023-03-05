// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
  let map_magazine: HashMap<String, i8> = count_words_in(magazine);
  let map_note    : HashMap<String, i8> = count_words_in(note);

  for (key, value) in map_note.iter(){
    match map_magazine.get(key) {
        None => return false,
        Some(compare_value) if compare_value < value => return false,
        Some(_) => continue,
    }
  }

  true
}

fn count_words_in(magazine: &[&str]) -> HashMap<String, i8> {
  let mut map: HashMap<String, i8> = HashMap::new();
  for word in magazine {
    if map.contains_key(word.to_owned()){ 
      *map.get_mut(word.to_owned()).unwrap() += 1;
      continue;
    }
    map.insert(word.to_string(), 1);
  }

  return map;
}
