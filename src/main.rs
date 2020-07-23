use hive::grid::geo::hex::Hex;
use hive::grid::geo::{axial_to_cube, cube_to_axial};

fn main() {
    let hex = Hex { q: -1, r: -1 };
    println!("Hex:{{ {} }}", hex);
    let cube = axial_to_cube(hex);
    println!("Cube: {{ {} }}", cube);
    let hex = cube_to_axial(cube);
    println!("Hex:{{ {} }}", hex);
}
