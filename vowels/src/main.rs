#[derive(Debug)]
struct VowelCount {
    a: u32,
    e: u32,
    i: u32,
    o: u32,
    u: u32,
}

impl VowelCount {
    fn count_vowels(&mut self, input: &str) {
        for ch in input.chars() {
            match ch {
                'a' => self.a += 1,
                'e' => self.e += 1,
                'i' => self.i += 1,
                'o' => self.o += 1,
                'u' => self.u += 1,
                _ => (),
            }
        }
    }

    fn new() -> VowelCount {
        return VowelCount{
            a: 0,
            e: 0,
            i: 0,
            o: 0,
            u: 0,
        };
    }
}

fn main() {
    let sentence = "Hello, world!";
    let mut counter = VowelCount::new();
    counter.count_vowels(sentence);
    println!("Count is {:?}", counter);
}
