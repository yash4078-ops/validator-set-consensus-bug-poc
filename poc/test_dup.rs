use arc_consensus_types::{Validator, ValidatorSet};
use arc_consensus_types::signing::PrivateKey;
use arc_consensus_types::proposer::{ProposerSelector, RoundRobin};
use arc_consensus_types::{Height, Round};

use rand::{SeedableRng, rngs::StdRng};

fn main() {
    let mut rng = StdRng::seed_from_u64(0x42);

    let sk = PrivateKey::generate(&mut rng);

    let v1 = Validator::new(sk.public_key(), 10);
    let v2 = Validator::new(sk.public_key(), 999);

    let vs = ValidatorSet::new(vec![v1.clone(), v2.clone()]);

    println!("validator count = {}", vs.len());
    println!("total vp = {}", vs.total_voting_power());

    for (i, v) in vs.iter().enumerate() {
        println!(
            "{} addr={:?} power={}",
            i,
            v.address,
            v.voting_power
        );
    }

    let rr = RoundRobin;

    for r in 0..6 {
        let proposer = rr.select_proposer(
            &vs,
            Height::new(1),
            Round::new(r),
        );

        println!(
            "round={} proposer={:?} power={}",
            r,
            proposer.address,
            proposer.voting_power
        );
    }
}
