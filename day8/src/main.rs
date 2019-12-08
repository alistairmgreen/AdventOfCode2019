use bytecount::count;

fn main() {
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    const PIXELS: usize = WIDTH * HEIGHT;
    const BLACK: u8 = 0;
    const WHITE: u8 = 1;
    const TRANSPARENT: u8 = 2;

    let image: Vec<u8> = include_bytes!("image.txt")
        .iter()
        .map(|&digit| digit - b'0')
        .collect();
    
    let layers: Vec<&[u8]> = image.chunks(PIXELS).collect();
    let layer_count = layers.len();
    
    let part1_layer = layers.iter()
        .min_by_key(|&&layer| count(layer, 0))
        .unwrap();

    let ones = count(part1_layer, 1);
    let twos = count(part1_layer, 2);

    println!("Part 1 answer: {}", ones * twos);

    let mut composite_image = layers[layer_count - 1].to_owned();

    for &layer in layers.iter().rev().skip(1) {
        for (&pixel, composite) in layer.iter().zip(composite_image.iter_mut()) {
            if pixel != TRANSPARENT {
                *composite = pixel;
            }
        }
    }

    let mut output = String::with_capacity(PIXELS + layer_count);
    for row in composite_image.chunks(WIDTH) {
        for &pixel in row {
            output.push(match pixel {
                BLACK => ' ',
                WHITE => '#',
                _ => '.'
            });
        }
        output.push('\n');
    }

    println!("{}", output);
}
