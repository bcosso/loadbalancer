use crate::configs::Peer;

pub enum LbAlgotithms {
    RoundRobin,
    WeightedRoundRobin,
}

pub fn get_next_node_with_lb_strategy(alg: LbAlgotithms, peers :Vec<Peer>, index_node: &mut u8) -> Peer {
    match alg{
        LbAlgotithms::RoundRobin => execute_round_robin(peers, index_node),
        //TODO: Implement weighted
        LbAlgotithms::WeightedRoundRobin => execute_round_robin(peers, index_node),
    } 
}

fn execute_round_robin(peers :Vec<Peer>, index_node: &mut u8) -> Peer{
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
