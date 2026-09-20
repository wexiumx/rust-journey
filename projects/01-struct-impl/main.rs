struct ColorMaker {
    red: u8,
    green: u8,
    blue: u8,
}

impl ColorMaker {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue
        }
    }

    fn rgb_to_hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

fn main() {
    let color = ColorMaker::new(255, 0, 255);
    let hex = color.rgb_to_hex();

    println!(
        "`color` elements are: red: {}, green: {}, blue: {}",
        color.red,
        color.green,
        color.blue
    );

    println!("hex: {}", hex);
    
}
