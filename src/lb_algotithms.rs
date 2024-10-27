use std::fmt;
use crate::configs::Peer;

#[derive(Debug)]
pub enum LbAlgotithms {
    RoundRobin,
    WeightedRoundRobin,
}

trait LBAlgorithm{
    fn execute_lb_algorithm(&self, peers :Vec<Peer>, index_node: &mut u8) -> Peer;
}

struct RoundRobin{}

impl RoundRobin{
    pub fn new() -> Self {
        RoundRobin{}
    }
}

impl LBAlgorithm for RoundRobin {
    fn execute_lb_algorithm(&self, peers :Vec<Peer>, index_node: &mut u8) -> Peer{
        let p:Peer;

        if (*index_node as usize) < peers.len() - 1{
            (*index_node) += 1;
        }else {
            (*index_node) = 0;
        }

        p = peers[(*index_node) as usize].clone();

        // for peer in &mut peers{
        //     println!("{}",&peer.name) 
        // }

        return p;
    }
}

impl fmt::Display for LbAlgotithms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn create_instance(name: &str) -> Box<dyn LBAlgorithm> {
     match name {
         "RoundRobin" => Box::new(RoundRobin::new()),
         _ => Box::new(RoundRobin::new())
         // ...
     }
}

pub fn get_next_node_with_lb_strategy(alg: LbAlgotithms, peers :Vec<Peer>, index_node: &mut u8) -> Peer {
    let alg_choice = create_instance(&alg.to_string());
    alg_choice.execute_lb_algorithm(peers, index_node)
}

