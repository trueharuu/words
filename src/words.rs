pub fn words() -> Vec<String> {
    // println!("{:?}", std::fs::read_dir(".").unwrap().collect::<Vec<_>>());
    std::fs::read_to_string("./words.txt")
        .unwrap()
        .lines()
        .map(str::to_owned)
        .map(|x| x.to_lowercase())
        .collect::<Vec<_>>()
}

pub fn small_words() -> Vec<String> {
    std::fs::read_to_string("./20k.txt")
        .unwrap()
        .lines()
        .map(str::to_owned)
        .map(|x| x.to_lowercase())
        .collect::<Vec<_>>()
}

// pub fn missing_words() -> Vec<String> {
//     std::fs::read_to_string("./missing_words.txt")
//         .unwrap()
//         .lines()
//         .map(str::to_owned)
//         .map(|x| x.to_lowercase())
//         .collect::<Vec<_>>()
// }
