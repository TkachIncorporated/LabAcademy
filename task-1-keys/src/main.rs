use num::{bigint::Sign, BigInt};
use num_bigint::{RandBigInt, ToBigInt};
use std::time::Instant;

fn solving(i: BigInt, length: i32) {
    let mut cur: BigInt = BigInt::new(Sign::Plus, vec![0, 0]);
    let now = Instant::now();

    loop {
        cur += 1;

        if cur == i {
            let elapsed = now.elapsed();
            println!("Time takes for {} bit guessing: {:?}", length, elapsed);

            break;
        }
    }
}

fn generate() -> Vec<BigInt> {
    let mut length: i32 = 8;
    let zero: i32 = 0;

    let mut vec: Vec<BigInt> = Vec::new();

    loop {
        let bin = BigInt::new(Sign::Plus, vec![2, 0]);
        let mut rng = rand::thread_rng();
        let numb = rng.gen_bigint_range(
            &zero.try_into().unwrap(),
            &num::pow(bin, length.try_into().unwrap())
                .to_bigint()
                .expect("convert error"),
        );

        println!("Your key for {} bit: {:#x}", length, numb);

        vec.push(numb);

        if length == 4096 {
            break;
        }

        length *= 2;
    }

    vec
}

fn key_variation() {
    let mut length: i32 = 8;

    loop {
        let bin = BigInt::new(Sign::Plus, vec![2, 0]);

        println!(
            "Key space for {} bit: {:?}",
            length,
            num::pow(bin, length.try_into().unwrap())
                .to_bigint()
                .expect("convert error")
        );
        if length == 4096 {
            break;
        }

        length *= 2;
    }
}

fn main() {
    key_variation();

    println!("------------------------------------");

    let vec = generate();

    println!("------------------------------------");

    let mut length = 8;

    //solving(*vec.get(0).expect("reading error"), length);

    for i in vec {
        solving(i, length);

        length *= 2;
    }
}
