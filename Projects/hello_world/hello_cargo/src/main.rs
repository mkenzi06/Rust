fn seulement_positif(v: i32) -> Option<i32> {
	if v < 0 {
		None
	} else {
		Some(v)
	}
}fn main() {
    // Tests
    println!("{:?}", seulement_positif(-5)); // Affiche None
    println!("{:?}", seulement_positif(3));  // Affiche Some(3)
}