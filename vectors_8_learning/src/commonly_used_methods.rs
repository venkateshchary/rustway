
pub fn truncate(v:&mut Vec<i32>, specified_length: i32) {
    println!("Truncate the vector");
    v.truncate(specified_length as usize)
}

pub fn first(v: & Vec<i32>) -> Option<&i32> {
    if v.len() > 0 {
        v.first()
    }
    else{
        None
    }

}

pub fn last(v: &Vec<i32>) -> Option<&i32> {
    if v.len() > 0 {
        println!("vector has {} elements", v.len());
        v.last()
    }
    else{
        None
    }
}

pub fn remove_by_index(v: &mut Vec<i32>, specified_index: usize) -> Option<i32> {
    if v.len() > 0 && specified_index < v.len() {
        Some(v.remove(specified_index))
    }
    else {
        None
    }
}

pub fn pop_from_vector(v: &mut Vec<i32>) -> Option<i32> {
    if v.len() > 0 {
        v.pop()
    }
    else {
        None
    }
}

pub fn reverse(v: &mut Vec<i32>) {
    v.reverse()
}