use std::ascii::AsciiExt;
use std::collections::HashMap;

fn get_letter_frequency_map() -> HashMap<char, f32> {
    let frequencies = vec![('e', 0.1041442), ('t', 0.0729357), ('a', 0.0651738),
                           ('o', 0.0596302), ('i', 0.0558094), ('n', 0.0564513),
                           ('s', 0.0515760), ('r', 0.0497563), ('h', 0.0492888),
                           ('l', 0.0331490), ('d', 0.0349835), ('c', 0.0217339),
                           ('u', 0.0225134), ('m', 0.0202124), ('f', 0.0197881),
                           ('p', 0.0137645), ('g', 0.0158610), ('w', 0.0171272),
                           ('y', 0.0145984), ('b', 0.0124248), ('v', 0.0082903),
                           ('k', 0.0050529), ('x', 0.0013692), ('j', 0.0009033),
                           ('q', 0.0008606), ('z', 0.0007836), (' ', 0.1918182)];

    frequencies.into_iter().collect::<HashMap<char, f32>>()
}

pub fn score_freq(s: &String) -> f32 {
    let mut score: f32 = 0.0;
    let freqs = get_letter_frequency_map();
    for c in s.chars() {
        match freqs.get(&(c.to_ascii_lowercase())) {
            Some(f) => score = score + f,
            None => ()
        };
    };
    score
}
