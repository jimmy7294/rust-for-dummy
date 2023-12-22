pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val)
}

pub fn at_index(vec: &mut Vec<String>, index: usize) -> String {
    if let Some(val) = vec.get_mut(index) {
        val.to_string()
    } else {
        panic!("Index out of bounds")
    }
}
