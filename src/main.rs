mod casper;
use casper::{Message, Validator, Weight};
use std::collections::{HashMap, HashSet};


fn main() {
    
    // allows us to give ids to messages for ordering
    let mut id_iterator = interator(0);

    //ugly for now, but I gotta start somewhere
    let genesis_block:Message = Message::genesis(id_iterator.next().unwrap());
    
    // maps validators to their respective weights
    // note: validator 0 is used to "send" the genesis block
    let mut validator_weights: HashMap<Validator, Weight> = HashMap::new();

    let validator1: Validator = 1;
    let validator2: Validator = 2;
    let validator3: Validator = 3;
    let validator4: Validator = 4;

    validator_weights.insert(validator1, 10.0);
    validator_weights.insert(validator2, 11.0);
    validator_weights.insert(validator3, 9.0);
    validator_weights.insert(validator4, 8.0);

    println!("Genesis Message{:#?}", genesis_block);
    
    let mut j1: HashSet<& Message> = HashSet::new();
    j1.insert(&genesis_block);
    let m1: Message = Message::build_message(validator1,j1,  &genesis_block, id_iterator.next().unwrap(), &validator_weights);

    println!("Message 1 {:#?}", m1);
    
    let mut j2: HashSet<& Message> = HashSet::new();
    j2.insert(&genesis_block);
    j2.insert(&m1);
    let m2: Message = Message::build_message(validator2,j2,  &genesis_block, id_iterator.next().unwrap(), &validator_weights);

    println!("Message 2 {:#?}", m2);
}


// practicing iterators...
fn interator(start:i64) -> It
{
	It { curr: start-1 }
}

struct It{
    curr: i64,
}

impl Iterator for It{
    type Item = i64;
    fn next(&mut self) -> Option<i64>{
        self.curr = self.curr + 1;
        Some(self.curr)
    }
}
