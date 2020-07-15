use saddle_points::create_saddle;

fn main() {
    let saddle = create_saddle((0, 35), (0, 35));
    println!("{:?}", saddle);
    for row in saddle {
        for item in row {
            print!("{:4} ", item);
        }
        println!();
    }
}
