use trev_aoc_common::run;

fn part_1(polymer: String) -> usize {
    let mut i = 0;
    let mut polymer: Vec<char> = polymer.chars().collect();
    while i < polymer.len() - 1 {
        loop {
            if (polymer[i].is_lowercase() ^ polymer[i + 1].is_lowercase())
                && polymer[i].to_lowercase().to_string()
                    == polymer[i + 1].to_lowercase().to_string()
            {
                polymer.remove(i);
                polymer.remove(i);
                if i != 0 {
                    i -= 1;
                }
                if i == polymer.len() - 1 {
                    break;
                }
            } else {
                break;
            }
        }
        i += 1;
    }
    polymer.len()
}

fn part_1_without_char(polymer: String, c: char) -> usize {
    part_1(
        polymer
            .replace(c as char, "")
            .replace((c as char).to_uppercase().next().unwrap(), ""),
    )
}

fn part_2(polymer: String) -> usize {
    (97u8..123)
        .map(|x| part_1_without_char(polymer.clone(), x as char))
        .min()
        .unwrap()
}

fn main() {
    run(&part_1, &part_2);
}
