use rand::Rng;

fn generate_unique_numbers(count: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut unique_numbers = Vec::new();

    while unique_numbers.len() < count {
        let random_number: u32 = rng.gen_range(100_000..1_000_000);

        if !unique_numbers.contains(&random_number) {
            unique_numbers.push(random_number);
        }
    }

    unique_numbers
}

fn main() {
    let keys = vec![
        "ag_1", "ag_2", "ag_3", "ag_4", "ag_5", "ag_6", "ag_7", "ag_8", "ag_9", "ag_10",
        "ag_11", "ag_12", "ag_13", "ag_14", "ag_15", "ag_16", "ag_17", "ag_18", "ag_19", "ag_20",
        "ag_21", "ag_22", "ag_23", "ag_24", "ag_25", "ag_26", "ag_27", "ag_28", "ag_29", "ag_30",
        "ag_31", "ag_32", "ag_33", "ag_34", "ag_35", "ag_36", "ag_37", "ag_38", "ag_39", "ag_40",
        "ag_41", "ag_42", "ag_43", "ag_44",
    ];

    let unique_numbers = generate_unique_numbers(keys.len());

    let key_number_map: Vec<(&str, u32)> = keys.iter().cloned().zip(unique_numbers).collect();

    for (key, number) in key_number_map {
        println!("{} : {:06}", key, number);
    }
}
