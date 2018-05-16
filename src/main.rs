mod casper;
use casper::{Message, Validator, Weight};
use std::collections::{HashMap, HashSet};


fn main() {
    
    // allows us to give ids to messages for ordering
    let mut id_iterator = interator(0);


    let validator1: Validator = 1;
    let validator2: Validator = 2;
    let validator3: Validator = 3;
    let validator4: Validator = 4;

    let validators_weights: Vec<(Validator, Weight)> = vec![
        (validator1, 1.0),
        (validator2, 1.0),
    // if you change the next weight to something > 2, the blockchain changes completely
    // even though the validator 3 only has the genesis block as a justification
    // and validator 4 has all the messages as justification
        (validator3, 1.0),
        (validator4, 1.0),
    ];

    // maps validators to their respective weights
    // note: validator 0 is used to "send" the genesis block
    let validators_weights: HashMap<_, _> = validators_weights.into_iter().collect();

    // create a genesis message
    let genesis_block:Message = Message::genesis(&mut id_iterator);
    
    println!("Genesis Message: {:#?}", genesis_block);
    
    let mut j1: HashSet<& Message> = HashSet::new();
    j1.insert(&genesis_block);
    let m1: Message = Message::build_message(
        validator1, 
        j1, 
        &genesis_block, 
        &mut id_iterator, 
        &validators_weights);

    println!("Message 1: {:#?}", m1);
    
    let mut j2: HashSet<& Message> = HashSet::new();
    j2.insert(&genesis_block);
    j2.insert(&m1);

    let m2: Message = Message::build_message(
        validator2,
        j2,  
        &genesis_block, 
        &mut id_iterator, 
        &validators_weights);

    println!("Message 2: {:#?}", m2);
    
    let mut j3: HashSet<& Message> = HashSet::new();
    j3.insert(&genesis_block);
//    j3.insert(&m1);
//    j3.insert(&m2);
    
    let m3: Message = Message::build_message(
        validator3,
        j3,
        &genesis_block, 
        &mut id_iterator, 
        &validators_weights);
    
    println!("Message 3: {:#?}", m3);
    
    let mut j4: HashSet<& Message> = HashSet::new();
    j4.insert(&genesis_block);
    j4.insert(&m1);
    j4.insert(&m2);
    j4.insert(&m3);

    let m4: Message = Message::build_message(
        validator4,
        j4, 
        &genesis_block, 
        &mut id_iterator, 
        &validators_weights);

    println!("Message 4: {:#?}", m4);
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
