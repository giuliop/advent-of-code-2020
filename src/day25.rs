fn key(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= 20201227;
    }
    value
}

pub fn a() -> String {
    let card_public_key = 9033205;
    let door_public_key = 9281649;
    let subject_number = 7;
    let mut value: usize = 1;
    let mut loop_size: usize = 1;

    loop {
        value *= subject_number;
        value %= 20201227;

        if value == card_public_key {
            return key(door_public_key, loop_size).to_string();
        }
        if value == door_public_key {
            return key(card_public_key, loop_size).to_string();
        }
        loop_size += 1;
    }
}
