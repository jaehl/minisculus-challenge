const KEYS: [char; 69] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z', '.', ',', '?', '!', '\'', '"', ' ',
];

const WHEEL_SIZE: i32 = KEYS.len() as i32;

fn key_to_int(ch: char) -> i32 {
    match ch {
        '0'..='9' => (ch as i32) - ('0' as i32),
        'A'..='Z' => (ch as i32) - ('A' as i32) + 10,
        'a'..='z' => (ch as i32) - ('a' as i32) + 36,
        '.' => 62,
        ',' => 63,
        '?' => 64,
        '!' => 65,
        '\'' => 66,
        '"' => 67,
        ' ' => 68,
        _ => panic!("unexpected character"),
    }
}

fn int_to_key(n: i32) -> char {
    KEYS[n.rem_euclid(WHEEL_SIZE) as usize]
}

fn wheel_offset(wheel1: i32, wheel2: i32, wheel3: i32) -> i32 {
    (wheel1 - (2 * wheel2) + (2 * wheel3)).rem_euclid(WHEEL_SIZE)
}

fn encrypt(plain_text: &str, wheel1: i32, wheel2: i32, enable_wheel3: bool) -> String {
    let mut cipher_text = String::with_capacity(plain_text.len());

    let mut wheel3 = 0;
    for plain_char in plain_text.chars() {
        let plain_int = key_to_int(plain_char);

        let cypher_int = plain_int + wheel_offset(wheel1, wheel2, wheel3);
        let cypher_char = int_to_key(cypher_int);

        cipher_text.push(cypher_char);

        if enable_wheel3 {
            wheel3 = plain_int;
        }
    }

    cipher_text
}

fn decrypt(cipher_text: &str, wheel1: i32, wheel2: i32, enable_wheel3: bool) -> String {
    let mut plain_text = String::with_capacity(cipher_text.len());

    let mut wheel3 = 0;
    for cipher_char in cipher_text.chars() {
        let cipher_int = key_to_int(cipher_char);

        let plain_int = cipher_int - wheel_offset(wheel1, wheel2, wheel3);
        let plain_char = int_to_key(plain_int);

        plain_text.push(plain_char);

        if enable_wheel3 {
            wheel3 = key_to_int(plain_char);
        }
    }

    plain_text
}

fn encrypt_i(plain_text: &str, wheel1: i32) -> String {
    encrypt(plain_text, wheel1, 0, false)
}

fn encrypt_ii(plain_text: &str, wheel1: i32, wheel2: i32) -> String {
    encrypt(plain_text, wheel1, wheel2, false)
}

fn encrypt_iv(plain_text: &str, wheel1: i32, wheel2: i32) -> String {
    encrypt(plain_text, wheel1, wheel2, true)
}

fn decrypt_iv(cipher_text: &str, wheel1: i32, wheel2: i32) -> String {
    decrypt(cipher_text, wheel1, wheel2, true)
}

fn crack_iv_wheels(cipher_text: &str) -> [i32; 2] {
    for wheel1 in 0..WHEEL_SIZE {
        for wheel2 in 0..WHEEL_SIZE {
            let plain_text = decrypt_iv(cipher_text, wheel1, wheel2);

            if plain_text.contains("BUNKER") || plain_text.contains("FURLIN") {
                return [wheel1, wheel2];
            }
        }
    }

    panic!("no keywords found")
}

fn crack_iv(cipher_text: &str) -> (String, Vec<[i32; 2]>) {
    let first_wheels = crack_iv_wheels(cipher_text);
    let offset = wheel_offset(first_wheels[0], first_wheels[1], 0);

    let mut all_wheels = Vec::new();

    for wheel1 in 0..WHEEL_SIZE {
        for wheel2 in 0..WHEEL_SIZE {
            if wheel_offset(wheel1, wheel2, 0) == offset {
                all_wheels.push([wheel1, wheel2]);
            }
        }
    }

    let plain_text = decrypt_iv(cipher_text, first_wheels[0], first_wheels[1]);

    (plain_text, all_wheels)
}

fn main() {
    let input1 = "Strong NE Winds!";
    println!("answer 1 = {:?}", encrypt_i(input1, 6));

    let input2 = "The Desert Fox will move 30 tanks to Calais at dawn";
    println!("answer 2 = {:?}", encrypt_ii(input2, 9, 3));

    let input3 = "The white cliffs of Alghero are visible at night";
    println!("answer 3 = {:?}", encrypt_iv(input3, 4, 7));

    let input4 = "WZyDsL3u'0TfxP06RtSSF 'DbzhdyFIAu2 zF f5KE\"SOQTNA8A\"NCKPOKG5D9GSQE'M86IGFMKE6'\
        K4pEVPK!bv83I";
    println!("answer 4 = {:?}", decrypt_iv(input4, 7, 2));

    let input5 = "QT4e8MJYVhkls.27BL9,.MSqYSi'IUpAJKWg9Ul9p4o8oUoGy'ITd4d0AJVsLQp4kKJB2rz4dxfahwUa\
        \"Wa.MS!k4hs2yY3k8ymnla.MOTxJ6wBM7sC0srXmyAAMl9t\"Wk4hs2yYTtH0vwUZp4a\"WhB2u,o6.!8Zt\"Wf,,\
        eh5tk8WXv9UoM99w2Vr4!.xqA,5MSpWl9p4kJ2oUg'6evkEiQhC'd5d4k0qA'24nEqhtAQmy37il9p4o8vdoVr!xWS\
        kEDn?,iZpw24kF\"fhGJZMI8nkI";
    let (output5, wheels5) = crack_iv(input5);
    println!("answer 5 = {:?}", output5);
    println!("answer 5 wheels = {:?}", wheels5);
}
