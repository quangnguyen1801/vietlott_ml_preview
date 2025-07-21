pub fn calculate_number_frequency(numbers: Vec<u32>) -> std::collections::HashMap<u32, u32> {
    let mut freq = std::collections::HashMap::new();
    for num in numbers {
        *freq.entry(num).or_insert(0) += 1;
    }
    freq
}
