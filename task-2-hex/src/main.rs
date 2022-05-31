use num::{BigInt, ToPrimitive};
use num_bigint::{Sign, ToBigInt};
use unicode_segmentation::UnicodeSegmentation;

fn from_hex_to_le(vector: &str) -> BigInt {
    let mut numb: BigInt = num::zero();
    let split: Vec<&str> = vector.split('x').collect();

    let str = split[1];
    let mut parser: i16;

    while numb != num::zero() {
        print!("sd");
    }

    for (pows, i) in (0_i16..).zip(str.to_lowercase().chars()) {
        match i {
            'a' => parser = 10,
            'b' => parser = 11,
            'c' => parser = 12,
            'd' => parser = 13,
            'e' => parser = 14,
            'f' => parser = 15,
            _ => parser = i.to_string().parse::<i16>().unwrap(),
        };

        let bin: BigInt = BigInt::new(Sign::Plus, vec![16, 0]);

        numb += parser.to_bigint().expect("convert error")
            * num::pow(bin, pows.try_into().unwrap())
                .to_bigint()
                .expect("convert error");
    }

    numb
}

fn from_le_to_hex(mut value: BigInt) -> String {
    let mut out: String = String::new();

    while value != num::zero() {
        let temp: u8 = ((value.clone() % 16) as BigInt)
            .to_u8()
            .expect("convert error");

        let ch = match temp {
            10 => 'a',
            11 => 'b',
            12 => 'c',
            13 => 'd',
            14 => 'e',
            15 => 'f',
            _ => temp as char,
        };

        out.push(ch);

        value /= 16;
    }

    out.insert_str(0, "0x");

    out
}

fn from_hex_to_be(vector: &str) -> BigInt {
    let mut numb: BigInt = num::zero();
    let split: Vec<&str> = vector.split('x').collect();

    let str = split[1];
    let mut parser: i16;

    for (pows, i) in (0_i16..).zip(str.to_lowercase().chars().rev()) {
        match i {
            'a' => parser = 10,
            'b' => parser = 11,
            'c' => parser = 12,
            'd' => parser = 13,
            'e' => parser = 14,
            'f' => parser = 15,
            _ => parser = i.to_string().parse::<i16>().unwrap(),
        };

        let bin: BigInt = BigInt::new(Sign::Plus, vec![16, 0]);

        numb += parser.to_bigint().expect("convert error")
            * num::pow(bin, pows.try_into().unwrap())
                .to_bigint()
                .expect("convert error");
    }

    numb
}

fn from_be_to_hex(mut value: BigInt) -> String {
    let mut out: String = String::new();

    while value != num::zero() {
        let temp: u8 = ((value.clone() % 16) as BigInt)
            .to_u8()
            .expect("convert error");

        let ch = match temp {
            10 => 'a',
            11 => 'b',
            12 => 'c',
            13 => 'd',
            14 => 'e',
            15 => 'f',
            _ => temp as char,
        };

        out.push(ch);

        value /= 16;
    }

    let mut new_out: String = out.graphemes(true).rev().collect();
    new_out.insert_str(0, "0x");

    new_out
}

fn main() {
    // ----------- Test 1 -----------
    let vector = "0xff00000000000000000000000000000000000000000000000000000000000000";
    let le_value = from_hex_to_le(vector);
    let be_value = from_hex_to_be(vector);

    println!("Testing vector: {}\n", vector);

    println!("Test 1 little endian value: {}", le_value);
    println!("Test 1 big endian value: {}", be_value);
    println!("Test 1 from le to hex: {}", from_le_to_hex(le_value));
    println!("Test 1 from be to hex: {}\n", from_be_to_hex(be_value));

    // ----------- Test 2 -----------
    let vector = "0xaaaa000000000000000000000000000000000000000000000000000000000000";
    let le_value = from_hex_to_le(vector);
    let be_value = from_hex_to_be(vector);

    println!("Testing vector: {}\n", vector);

    println!("Test 2 little endian value: {}", le_value);
    println!("Test 2 big endian value: {}", be_value);
    println!("Test 2 from le to hex: {}", from_le_to_hex(le_value));
    println!("Test 2 from be to hex: {}\n", from_be_to_hex(be_value));

    // ----------- Test 3 -----------
    let vector = "0xFFFFFFFF";
    let le_value = from_hex_to_le(vector);
    let be_value = from_hex_to_be(vector);

    println!("Testing vector: {}\n", vector);

    println!("Test 3 little endian value: {}", le_value);
    println!("Test 3 big endian value: {}", be_value);
    println!("Test 3 from le to hex: {}", from_le_to_hex(le_value));
    println!("Test 3 from be to hex: {}\n", from_be_to_hex(be_value));

    // ----------- Test 4 -----------
    let vector = "0xF000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    let le_value = from_hex_to_le(vector);
    let be_value = from_hex_to_be(vector);

    println!("Testing vector: {}\n", vector);

    println!("Test 4 little endian value: {}", le_value);
    println!("Test 4 big endian value: {}", be_value);
    println!("Test 4 from le to hex: {}", from_le_to_hex(le_value));
    println!("Test 4 from be to hex: {}\n", from_be_to_hex(be_value));

    // ----------- Test 5 -----------
    // For being sure that string swaps work properly
    let vector = "0xffaa0000";
    let le_value = from_hex_to_le(vector);
    let be_value = from_hex_to_be(vector);

    println!(
        "There is some weird space between tests 4 and 5 in executable file, i don't know why"
    );
    println!("Testing vector: {}\n", vector);

    println!("Custom test 5 little endian value: {}", le_value);
    println!("Custom test 5 big endian value: {}", be_value);
    println!("Custom test 5 from le to hex: {}", from_le_to_hex(le_value));
    println!("Custom test 5 from be to hex: {}", from_be_to_hex(be_value));

    println!("Press enter to close the console!");
    let mut line = String::new();
    let _something_useless = std::io::stdin().read_line(&mut line).unwrap();
}
