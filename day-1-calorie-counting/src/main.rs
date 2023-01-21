use std::fs;

fn main() {
    let mut elf_totals: Vec<usize> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split("\n")
                .map(|energy| energy.parse().unwrap_or(0))
                .sum()
        })
        .collect();

    elf_totals.sort();
    elf_totals.reverse();

    let max = elf_totals.get(1).unwrap();
    println!("{}", max);

    let top_three_sum: usize = elf_totals.get(0..3).unwrap().into_iter().sum();
    println!("{}", top_three_sum);
}
