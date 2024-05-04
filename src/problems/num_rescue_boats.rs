pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort();
    let mut result = 0;
    while people.len() > 0 {
        if people.len() == 1 {
            result += 1;
            break;
        }
        let last = people.last().unwrap();
        match people.binary_search(&(limit - *last)) {
            Ok(index) => {
                people.remove(index);
            }
            Err(close_index) => {
                if close_index != 0 {
                    people.remove(close_index - 1);
                }
            }
        }
        people.remove(people.len() - 1);
        result += 1;
    }
    result
}
