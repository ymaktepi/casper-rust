use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};

pub type Validator = u64;
pub type Weight = f64;


#[derive(Debug, Eq, PartialEq)]
pub struct Message<'a>{
    id: i64,
    sender: Validator,
    estimate: Option<&'a Message<'a>>,
    justification: HashSet<&'a Message<'a>>,
}

impl<'a> Message<'a>{

    /// Sender id 0 is taken for the genesis block
    pub fn genesis(id: i64)-> Message<'a>
    {
        Message{
            sender:0,
            estimate: None,
            justification: HashSet::new(),
            id: id,
        }
    }
}

/// utility function for hashset
impl<'a> Hash for Message<'a>{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.estimate.hash(state);
        self.sender.hash(state);
        self.id.hash(state);
    }
}

fn score<'a>(block: &'a Message<'a>, justification: &HashSet<&'a Message<'a>>, validators_weights: &HashMap<Validator, Weight>) -> f64{
    validators_weights.keys()
        .

}

/// Estimates the next block
/// For now, returns any item because I want this code to compile
fn estimator<'a>(justification: &HashSet<&'a Message<'a>>, genesis_block: &'a Message<'a>, validators_weights: &HashMap<Validator, Weight>)-> Option<&'a Message<'a>>{
    let mut b: &'a Message<'a> = genesis_block;
    let mut b_id: i64 = b.id;

    let mut last_message_per_validator = HashMap::new();

    for validator in validators_weights.keys(){
        let last_messages = justification.iter().filter(
            |m|
            m.sender == validator
            );
    }

    loop {
        let b_id_cmp = b_id;
        let b_children = justification.iter().filter(
            |m| {
                if m.estimate.is_none()
                {
                    false
                }
                else
                {
                    m.estimate.unwrap().id == b_id_cmp
                }   
            });

        let mut sorted_by_score_and_hash:Vec<_> = b_children
            .map(
                |child| 
                    (child, score(child, justification, validators_weights))
                )
            .collect();


        sorted_by_score_and_hash.sort_unstable_by(|a, b|{
                if a.1 == b.1{
                    return a.0.id.cmp(&b.0.id);
                } else {
                    return a.1.partial_cmp(&b.1).unwrap();
                }
            });
        println!("Children {:#?}", sorted_by_score_and_hash);
        if sorted_by_score_and_hash.len() > 0 {
            b = sorted_by_score_and_hash[0].0; 
            b_id = b.id;
        }
        else {
            break;
        }
    }
    Some(b)
//    justification.iter().next().map(|m| *m)
}

impl<'a> Message<'a>{
    /// builds a message using the justification and a sender
    #[allow(dead_code)]
    pub fn build_message(sender:Validator, justification: HashSet<&'a Message<'a>>, genesis_block: &'a Message<'a>, id: i64, validators_weights: &HashMap<Validator, Weight>) -> Message<'a>
    {
        let estimate = estimator(&justification, genesis_block, validators_weights);
        Message{ sender, justification, estimate, id}
    }
}
