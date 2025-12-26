use vtk_rs as vtk;

fn main() {
    println!("Testing LookupTable...");

    let mut lut = vtk::LookupTable::new();
    println!("Created lookup table");

    lut.set_range(0.0, 100.0);
    println!("Set range");

    lut.set_hue_range(0.66, 0.0);
    println!("Set hue range");

    lut.build();
    println!("Built table");

    let (r, g, b) = lut.get_color(50.0);
    println!("Color at 50.0: ({}, {}, {})", r, g, b);

    println!("Success!");
}
