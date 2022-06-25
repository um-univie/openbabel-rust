extern crate openbabel;

fn main() {
    let fpk = openbabel::fingerprint::Kind::ECFP4 { nbits: 4096 };
    let mol = openbabel::molecule::Molecule::new_from_smiles("c1ccccc1");
    let fpg = openbabel::fingerprint::FingerprintGenerator::new(fpk);
    let fpd = fpg.get_fingerprint(&mol);
    println!("ECFP for mole c1ccccc1: {:?}", fpd);
}