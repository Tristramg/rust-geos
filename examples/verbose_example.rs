extern crate geos;
use geos::{version, init, finish, GeosGeom};

fn main() {
    init();
    println!("geos_c version: {}", version());
    let g1 = GeosGeom::new("POLYGON ((0 0, 0 5, 5 5, 5 0, 0 0))");
    println!("Geometry 1 created" );
    println!("Area : {:?}", g1.area());
    let l = g1.length();
    println!("Length : {:?}", l);
    println!("Is Geom1 simple : {:?}", g1.is_simple());
    let g2 = GeosGeom::new("POLYGON ((1 1, 1 3, 5 5, 5 0, 1 1))");
    println!("Geometry 2 created");
    println!("Geom1 intersects geom2 : {:?}\n", g1.intersects(&g2));
    let g3 = g1.buffer(100.0, 8);
    println!("Previous area = {} \nNew area = {}", g2.area(), g3.area());
    let result = g1.within(&g2);
    println!("Geom1 within geom2 : {:?}\n", result);
    println!("Geom1 to wkt : {:?}", g1.to_wkt());
    println!("Is geom3 empty ? {:?}", g3.is_empty());
    println!("Is geom3 simple ? {:?}", g3.is_simple());
    println!("Geom3 to wkt : {:?}", g3.to_wkt());
    let g4 = g1.get_centroid();
    println!("Centroid of g1 : {:?}", g4.to_wkt());
    println!("Geom4 contains centroid of geom1 : {:?}", g3.contains(&g4));
    finish();
}