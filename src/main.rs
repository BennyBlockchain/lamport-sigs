use sha2::{Sha256, Digest};
use rand::Rng;

fn generate_keys() {
    let mut rng = rand::thread_rng();

    let mut priv_one: Vec<String> = Vec::new();
    let mut priv_two: Vec<String> = Vec::new();

    let mut pub_one: Vec<String> = Vec::new();
    let mut pub_two: Vec<String> = Vec::new();

    // Create first 256 length private key
    let mut i = 0;
    while i < 256 {
        let rand_num: f64 = rng.gen();
        let rand_string = rand_num.to_string();
        let rand_hash = Sha256::digest(rand_string);
        let hash_string = format!("{:x}", rand_hash);
        priv_one.push(hash_string); 
        i += 1;
    }

    // Create second 256 length private key
    let mut j = 0;
    while j < 256 {
        let rand_num: f64 = rng.gen();
        let rand_string = rand_num.to_string();
        let rand_hash = Sha256::digest(rand_string);
        let hash_string = format!("{:x}", rand_hash);
        priv_two.push(hash_string); 
        j += 1; 
    }

    // Create public keys hashed from private keys
    for i in 0..255 {
        let pub_hash_one = Sha256::digest(priv_one[i].clone());
        let pub_hash_one_string = format!("{:x}", pub_hash_one);
        pub_one.push(pub_hash_one_string);

        let pub_hash_two = Sha256::digest(priv_two[i].clone());
        let pub_hash_two_string = format!("{:x}", pub_hash_two);
        pub_two.push(pub_hash_two_string);
    }
}

fn main() {
    generate_keys();

}
