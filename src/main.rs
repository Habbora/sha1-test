use std::result;

fn main() {
    sha1("bom dia");
}

fn sha1(input: &str) {
    let message = input.as_bytes();
    let size_bytes = message.len() as u64;
    let size_bits = &size_bytes * 8;
    let size_blocks = (&size_bits / 512) + 1;

    let vec_bytes: Vec<u8> = {
        let mut vector: Vec<u8> = input
            .as_bytes()
            .iter()
            .take(size_bytes as usize)
            .cloned()
            .collect();
        // Adiciona o byte de padding
        vector.push(0x80 as u8);
        // Calcula o número de zeros necessários
        let last_step = (size_blocks * 64) as usize - size_bytes as usize - 1 - 8;
        // Adiciona os zeros de uma vez usando a função resize
        vector.resize(vector.len() + last_step, 0x00);
        vector.extend_from_slice(&size_bits.to_be_bytes()); // *
        vector
    };

    println!("\nMessage: {:?}\n", vec_u8_to_hex(vec_bytes.clone()));

    let vec_words: Vec<u32> = vec_bytes
        .chunks_exact(4)
        .map(|chunk| {
            let mut bytes = [0; 4];
            bytes.copy_from_slice(chunk);
            u32::from_be_bytes(bytes)
        })
        .collect();

    println!(
        "\nMessage: {:?}\nsize: {}\n",
        vec_u32_to_hex(vec_words.clone()),
        vec_words.len()
    );

    let vec_words = {
        let mut words = vec_words.clone();
        for i in 16..80 {
            let new_word = words[i - 3] ^ words[i - 8] ^ words[i - 14] ^ words[i - 16];
            let new_word = new_word.rotate_left(1);
            words.push(new_word);
        }
        words
    };

    println!(
        "\nMessage: {:?}\nsize: {}\n",
        vec_u32_to_hex(vec_words.clone()),
        vec_words.len()
    );

    {
        let w = |i: usize| vec_words[i].clone();
        let k = |t: usize| {
            match t {
                0..=19 => 0x5A827999 as u32,
                20..=39 => 0x6ED9EBA1 as u32,
                40..=59 => 0x8F1BBCDC as u32,
                60..=79 => 0xCA62C1D6 as u32,
                _ => panic!("Valor de t fora do intervalo válido para SHA-1"),
            }
        };
        let f = |t: usize, b: u32, c: u32, d: u32| {
            match t {
                0..=19 => (b & c) | (!b & d),
                20..=39 => b ^ c ^ d,
                40..=59 => (b & c) | (b & d) | (c & d),
                60..=79 => b ^ c ^ d,
                _ => panic!("Valor de t fora do intervalo válido para SHA-1"),
            }
        };
        let sum = |A: &u32, B: &u32| {
            let a = A.clone() as u64 + B.clone() as u64;
            a as u32
        };

        let mut h0: u32 = 0x67452301;
        let mut h1: u32 = 0xEFCDAB89;
        let mut h2: u32 = 0x98BADCFE;
        let mut h3: u32 = 0x10325476;
        let mut h4: u32 = 0xC3D2E1F0;

        let mut a: u32 = h0;
        let mut b: u32 = h1;
        let mut c: u32 = h2;
        let mut d: u32 = h3;
        let mut e: u32 = h4;

        for i in 0..80 {
            let temp = a.rotate_left(5) as u64
                + f(i, b, c, d) as u64
                + e as u64
                + w(i) as u64
                + k(i) as u64;
            
            e = d.clone();
            d = c.clone();
            c = b.rotate_left(30).clone();
            b = a.clone();
            a = temp as u32;
        }

        let result: Vec<u32> = vec![
            sum(&h0, &a),
            sum(&h1, &b),
            sum(&h2, &c),
            sum(&h3, &d),
            sum(&h4, &e),
        ];

        println!("Hash: {:?}\n", vec_u32_to_hex(result));
    }
}

const HEX_CODE: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

fn byte2hex(byte: u8) -> String {
    let a = HEX_CODE[(byte >> 4) as usize];
    let b = HEX_CODE[(byte & 0x0F) as usize];
    format!("{}{}", a, b)
}

fn print_vec_u8(input: Vec<u8>) {
    let result: String = input.iter().map(|&data| byte2hex(data)).collect();
    println!("{}", result);
}

fn vec_u8_to_hex(input: Vec<u8>) -> Vec<String> {
    let result: Vec<String> = input
        .iter()
        .map(|&data| format!("0x{}", byte2hex(data)))
        .collect();
    result
}

fn vec_u32_to_hex(input: Vec<u32>) -> Vec<String> {
    let result: Vec<String> = input
        .iter()
        .map(|&data| {
            format!(
                "0x{}{}{}{}",
                byte2hex((data >> 24) as u8),
                byte2hex((data >> 16) as u8),
                byte2hex((data >> 8) as u8),
                byte2hex((data) as u8),
            )
        })
        .collect();
    result
}
