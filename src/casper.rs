use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};

pub type Validator = u64;
pub type Weight = f64;


#[derive(Debug, Eq, PartialEq)]
pub struct Message<'a>{
    // id is here for uniqueness, comparison, and ordering 
    id: i64,
    sender: Validator,
    estimate: Option<&'a Message<'a>>,
    justification: HashSet<&'a Message<'a>>,
}

impl<'a> Message<'a>{

    /// Sender id 0 is taken for the genesis block
    pub fn genesis<I>(id: &mut I)-> Message<'a>
        where I: Iterator<Item=i64>
    {
        Message{
            sender:0,
            estimate: None,
            justification: HashSet::new(),
            id: id.next().unwrap(),
        }
    }
    
    /// builds a message using the justification and a sender
    pub fn build_message<I>(sender:Validator, justification: HashSet<&'a Message<'a>>, genesis_block: &'a Message<'a>, id: &mut I, validators_weights: &HashMap<Validator, Weight>) -> Message<'a>
        where I: Iterator<Item=i64>
    {
        let estimate = estimator(&justification, genesis_block, validators_weights);
        Message{ sender, justification, estimate, id: id.next().unwrap()}
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

fn score<'a>(block: &'a Message<'a>, validators_weights: &HashMap<Validator, Weight>, latest_message_per_validator: &HashMap<Validator, &Message>) -> f64{
    let mut score: f64 = 0.0;
    
    for  (validator, last_message) in latest_message_per_validator.iter(){
        let mut current_message: Option<&Message> = Some(last_message);
        loop{
            match current_message {
                Some(m) => {
                    if m.id == block.id{
                        score += validators_weights.get(validator).unwrap();
                        break;
                    }else{
                        current_message = m.estimate;
                    }
                },
                None => break,
            }
        }
    }

    score
}

/// Estimates the next block
/// For now, returns any item because I want this code to compile
fn estimator<'a>(justification: &HashSet<&'a Message<'a>>, genesis_block: &'a Message<'a>, validators_weights: &HashMap<Validator, Weight>)-> Option<&'a Message<'a>>{
    let mut b: &'a Message<'a> = genesis_block;
    let mut b_id: i64 = b.id;

    let mut latest_message_per_validator: HashMap<Validator, &Message> = HashMap::new();

    for validator in validators_weights.keys(){
        let mut latest_messages: Vec<_> = justification
            .iter()
            .filter(
                |m|
                m.sender == *validator
            )
            .collect();

        if latest_messages.len() > 0 {
            latest_messages.sort_unstable_by(
                |a, b|
                b.id.cmp(&a.id)
                );
           latest_message_per_validator.insert(*validator, latest_messages[0]);
        }
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
                    (child, score(child, validators_weights, &latest_message_per_validator))
                )
            .collect();


        sorted_by_score_and_hash.sort_unstable_by(|a, b|{
                // sort by bigger score. and then by smaller hash value (here we consider id is the
                // hash of the message)
                if a.1 == b.1{
                    return a.0.id.cmp(&b.0.id);
                } else {
                    return b.1.partial_cmp(&a.1).unwrap();
                }
            });

        if sorted_by_score_and_hash.len() > 0 {
            b = sorted_by_score_and_hash[0].0; 
            b_id = b.id;
        }
        else {
            break;
        }
    }

    Some(b)
}

