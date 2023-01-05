mod challenges;

fn main() {
    let key = "WHYRUST";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JRKUC
    ";
    let plaintext = challenges::vigenere::decrypt(&ciphertext, key);

    println!("Plaintext {}", plaintext);
    println!("Ciphertext {}", challenges::vigenere::encrypt(&plaintext, key))
}
