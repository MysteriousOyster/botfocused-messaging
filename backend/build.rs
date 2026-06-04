
use vergen_gitcl::{Emitter, GitclBuilder};

fn main() {
    let gitcl = GitclBuilder::default().all().build().unwrap();

    Emitter::default().add_instructions(&gitcl).unwrap().emit().unwrap();

}