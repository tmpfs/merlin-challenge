use merlin_challenge::challenge_labels;

challenge_labels!();

fn main() {
    //println!("{}", STAT_PARAM);
    for label in CHALLENGE_LABELS {
        takes_static_label(&label);
    }
}

fn takes_static_label(label: &'static [u8]) {
    println!("{:#?}", label);
}
