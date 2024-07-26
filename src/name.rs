use crate::first_names;
use crate::last_names;

pub fn random() -> String {
    let f_index = rand::random::<usize>() % first_names::FIRST_NAMES.len();    
    let l_index = rand::random::<usize>() % last_names::LAST_NAMES.len();
    format!("{} {}", first_names::FIRST_NAMES[f_index], last_names::LAST_NAMES[l_index])
}