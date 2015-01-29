pub fn contains(string: &str, chr: char) -> Option<usize> {
    let mut i: usize = 0;
    for c in string.chars() {
        if c == chr {
            return Some(i);
        }else
        {
            i += 1;
        }
    }
    None
}

/*pub fn get_date_formatted() -> String {*/
/*}*/
