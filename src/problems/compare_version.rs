pub fn compare_version(version1: String, version2: String) -> i32 {
    let mut revisions1: Vec<String> = version1.split('.').map(|s| s.to_string()).collect();
    let mut revisions2: Vec<String> = version2.split('.').map(|s| s.to_string()).collect();
    while revisions1.len() < revisions2.len() {
        revisions1.push(String::from("0"));
    }
    while revisions2.len() < revisions1.len() {
        revisions2.push(String::from("0"));
    }
    for index in 0..revisions1.len() {
        let rev1 = revisions1[index].parse::<u32>().unwrap();
        let rev2 = revisions2[index].parse::<u32>().unwrap();
        if rev1 > rev2 {
            return 1;
        } else if rev2 > rev1 {
            return -1;
        }
    }
    0
}
