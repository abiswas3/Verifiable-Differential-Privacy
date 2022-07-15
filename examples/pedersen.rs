extern crate dp_client as ss;

fn main() {
    let mut commitment = ss::pederson::PedersenCommitment::new(4).unwrap();
    // let mut commitment = ss:pederson_additive::PedersenCommitment::new(512).unwrap();
    println!("commitment {:#?}", commitment);

    let msg1 = 500;
    let msg2 = 100;
    let msg3 = 600;

    let (c1, r1) = commitment.commit(msg1).unwrap();
    let (c2, r2) = commitment.commit(msg2).unwrap();
    let (c3, r3) = commitment.commit(msg3).unwrap();

    println!();
    println!("c1: {}, \nr1: {}\n", c1, r1);
    println!("c2: {}, \nr2: {}\n", c2, r2);
    println!("c3: {}, \nr3: {}\n", c3, r3);

    let add_cm = commitment.add(&[&c1, &c2, &c3]).unwrap();
    println!("add_cm: {}\n", add_cm);

    let res = commitment
        .open(&add_cm, msg1 + msg2 + msg3, &[&r1, &r2, &r3])
        .unwrap();
    assert_eq!(res, true)

}
