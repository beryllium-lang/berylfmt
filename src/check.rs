pub fn check_valid(map: &Vec<&'static str>) {
    let mut max_size: usize = 1;
    
    for value in map {
       if !value.is_empty() {
           max_size = max_size.max(value.len());
       } 
    }
}
