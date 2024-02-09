#[allow(unused_variables)]
pub fn decrypt_seed(init_seed_hash: u64, end_seed_hash: u32) -> u32 {
    let mut actual_seed = 0;
    let mut _game_seed_xor: u64 = 0;
    let magic: u64 = 0x6AC690C5;
    let offset: u64 = 666;

    let divisor: u64 = 2 << (16 - 1);
    let mut modifier: u64 = 0;

    for try_seed in 0..divisor {
        let seed_result: u64 = (try_seed as u64 * magic + offset) & 0xFFFFFFFF;
        if seed_result % divisor == end_seed_hash as u64 % divisor {
            modifier = try_seed;
        }
    }

    for i in 0..(u32::MAX / divisor as u32) {
        let try_seed: u64 = modifier + i as u64 * divisor;
        let seed_result: u64 = (try_seed * magic + offset) & 0xFFFFFFFF;

        if seed_result == end_seed_hash as u64 {
            _game_seed_xor = init_seed_hash ^ try_seed;
            actual_seed = try_seed;
        }
    }
    // println!("init_seed_hash: {}, XOR: {} GameSeed: {}", init_seed_hash, game_seed_xor, actual_seed);
    actual_seed.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::decrypt_seed;

    #[test]
    fn test_seed_decrypt() {
        let seed = decrypt_seed(11658208065134863770, 3262333376);
        assert_eq!(seed, 741378286);
        let seed2 = decrypt_seed(1646629795820782549, 726648482);
        assert_eq!(seed2, 1127769192);
        let seed3 = decrypt_seed(16660833852954605639, 1557217863);
        assert_eq!(seed3, 1306694089);
    }
}
