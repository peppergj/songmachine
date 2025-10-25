use rand::seq::IndexedRandom;

fn main () {
    let songs = vec![
        "In The End - Linkin Park",
        "Numb - Linkin Park",
        "Points of Authority - Linkin Park"
    ];
    let glarb = songs.choose(&mut rand::rng()).unwrap().to_string();
    
    println!("{glarb}");
}
