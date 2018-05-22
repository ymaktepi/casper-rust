use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};
use std::fmt;


/// A validator ID
pub type Validator = u64;

/// A validator weight
pub type Weight = f64;


/// Casper message
#[derive(Debug, Eq, PartialEq)]
pub struct Message<'a>{
    /// id is here for uniqueness, comparison, and ordering
    id: i64,
    /// The validator that sent this message
    sender: Validator,
    /// The message that is estimated, or None if this message is the genesis block
    estimate: Option<&'a Message<'a>>,
    /// The messages that the validator has received before
    justification: HashSet<&'a Message<'a>>,
}

impl<'a> Message<'a>{

    /// Creates the genesis block
    /// Sender id 0 is taken for the genesis block
    pub fn genesis<I>(id: &mut I)-> Message<'a>
        where I: Iterator<Item=i64>
    {
        Message{
            sender:0,
            // No estimate
            estimate: None,
            // Justification is empty
            justification: HashSet::new(),
            // we take the next (supposedly first) value of the iterator as id
            id: id.next().unwrap(),
        }
    }

    /// builds a message using the justification and a sender
    /// param sender: the validator who sent the message
    /// param justification: all the messages the validator has received
    /// param genesis_block: the gensesis_block
    /// param id: an iterator that has the next id
    /// param validators_weights: a mapping validator -> weight
    pub fn new<I>(sender:Validator, justification: HashSet<&'a Message<'a>>, genesis_block: &'a Message<'a>, id: &mut I, validators_weights: &HashMap<Validator, Weight>) -> Message<'a>
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

impl<'a> fmt::Display for Message<'a>{
   fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Message id: {}, sender: {}, estimate: {:?}", self.id, self.sender, self.estimate )?;
        Ok(())
    }
}

/// computes the score of a message
/// param block: the block to compute the score for
/// param validators_weights: mapping validator -> weight
/// param latest_message_per_validator: a mapping validator -> latest message (computed from the
///     justification)
fn score<'a>(block: &'a Message<'a>, validators_weights: &HashMap<Validator, Weight>, latest_message_per_validator: &HashMap<Validator, &Message>) -> f64{
    let mut score: f64 = 0.0;

    // for each validator, check if the last message is in the blockchain of "block"
    // if it's the case, add the weight of the validator to the score
    for  (validator, last_message) in latest_message_per_validator.iter(){
        let mut current_message: Option<&Message> = Some(last_message);
        loop{
            match current_message {
                Some(m) => {
                    if m.id == block.id{
                        score += validators_weights.get(validator).unwrap();
                        // current block is reached, break loop
                        break;
                    }else{
                        current_message = m.estimate;
                    }
                },
                // genesis block is reached, break loop
                None => break,
            }
        }
    }

    score
}

/// Estimates the next block
/// param justification: the justification of the message
/// param genesis_block: the genesis block
/// param validators_weights: a mapping validator -> weight
/// returns an Option<Message>
fn estimator<'a>(justification: &HashSet<&'a Message<'a>>, genesis_block: &'a Message<'a>, validators_weights: &HashMap<Validator, Weight>)-> Option<&'a Message<'a>>{
    let mut b: &'a Message<'a> = genesis_block;
    let mut b_id: i64 = b.id;

    let mut latest_message_per_validator: HashMap<Validator, &Message> = HashMap::new();

    // compute the last message for each validator
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

    // GHOST
    loop {
        // get the id of the current block
        let b_id_cmp = b_id;

        // get all children of b
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

        // get the score of each child
        let mut sorted_by_score_and_hash:Vec<_> = b_children
            .map(
                |child|
                    (child, score(child, validators_weights, &latest_message_per_validator))
                )
            .collect();

        // sort by score (and hash if max score is not unique)
        sorted_by_score_and_hash.sort_unstable_by(|a, b|{
                // sort by bigger score. and then by smaller hash value (here we consider id is the
                // hash of the message)
                if a.1 == b.1{
                    return a.0.id.cmp(&b.0.id);
                } else {
                    return b.1.partial_cmp(&a.1).unwrap();
                }
            });

        // if we have a next block, loop again
        if sorted_by_score_and_hash.len() > 0 {
            b = sorted_by_score_and_hash[0].0;
            b_id = b.id;
        }
        // no next block, we have our complete blockchain
        else {
            break;
        }
    }

    Some(b)
}

