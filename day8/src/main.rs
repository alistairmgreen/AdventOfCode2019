use bytecount::count;

fn main() {
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    const PIXELS: usize = WIDTH * HEIGHT;

    let image: Vec<u8> = include_bytes!("image.txt")
        .iter()
        .map(|&digit| digit - b'0')
        .collect();
    
    let part1_layer = image.chunks(PIXELS)
        .min_by_key(|&layer| count(layer, 0))
        .unwrap();

    let ones = count(part1_layer, 1);
    let twos = count(part1_layer, 2);

    println!("Part 1 answer: {}", ones * twos);
}
