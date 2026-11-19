#![no_std]
#![no_main]

use panic_probe as _;

#[cfg(test)]
#[defmt_test::tests]
mod rng_tests {
    use defmt::println;
    use ectf_firmware_template::random::SecureRng;

    #[test]
    fn test_random_bytes() {
        let mut rng = SecureRng::new().unwrap();
        let random = rng.random_bytes::<256>().unwrap();
        println!("{:?}", random.as_slice());
    }
}
