pub fn num_steps(s: String) -> i32 {
    let mut result = 0;
    let mut c: Vec<char> = s.chars().collect();
    while c.len() != 1 {
        if *c.last().unwrap() != '1' {
            c.pop();
        } else {
            let mut ind: i32 = c.len() as i32 - 1;
            while ind >= 0 && c[ind as usize] == '1' {
                ind -= 1;
            }
            if ind < 0 {
                c.insert(0, '1');
                ind = 0;
            } else {
                c[ind as usize] = '1';
            }
            for i in ind as usize + 1..c.len() {
                c[i] = '0';
            }
        }
        result += 1;
    }
    result
}
