extern crate alloc;

pub mod dot_generator;
pub mod finite_automaton;
mod regular_expression;

use alloc::collections::btree_set::BTreeSet;
use alloc::vec::Vec;
//use std::fs::File;
//use std::io::prelude::*;
use std::time::*;

pub fn generate_lexer(
    source: Vec<&'static str>,
) -> (finite_automaton::SetDFA, Vec<Vec<usize>>, Vec<usize>) {
    let global_start_time = SystemTime::now();
    let start_time = SystemTime::now();
    let regex = source
        .iter()
        .map(|&string| {
            regular_expression::regex(string.chars().collect::<Vec<char>>())
                .into_iter()
                .map(|c| c as u8)
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();
    println!(
        "Processed regular expressions in {} ms.",
        start_time.elapsed().unwrap().as_millis()
    );
    let start_time = SystemTime::now();
    let (nfa, final_states) = finite_automaton::set_nfa::SetNFA::from_regex(&regex);
    if final_states.len() != source.len() {
        println!(
            "Warning, expected {} final states, but NFA has {}.",
            source.len(),
            final_states.len()
        );
    }
    println!(
        "Generated NFA in {} ms.",
        start_time.elapsed().unwrap().as_millis()
    );
    let start_time = SystemTime::now();
    //let dot_graph = dot_generator::DotGraph::from_nfa(&nfa);
    //let mut file = File::create("nfa.dot").unwrap();
    //file.write_all(&dot_graph.code).unwrap();
    let (dfa, nfa_to_dfa_map) = finite_automaton::set_dfa::SetDFA::from_nfa(nfa);
    //dbg!(&nfa_to_dfa_map);

    println!(
        "Generated DFA in {} ms.",
        start_time.elapsed().unwrap().as_millis()
    );
    let start_time = SystemTime::now();
    let final_states2: Vec<BTreeSet<usize>> = final_states
        .into_iter()
        .map(|nfa_state| {
            nfa_to_dfa_map
                .clone()
                .into_iter()
                .enumerate()
                .filter_map(|(dfa_state, nfa_states)| {
                    // Here I guess it does a linear search, so it will take the first token?
                    if nfa_states.contains(&nfa_state) {
                        Some(dfa_state)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();
    //dbg!(&final_states2);
    let mut dedup_final_states: Vec<BTreeSet<usize>> = Default::default();
    // FIXME: Very inefficient.
    for (i, final_set) in final_states2.iter().enumerate() {
        let mut new_set = final_set.clone();
        for previous_final_states in final_states2.iter().take(i) {
            new_set = new_set
                .difference(&previous_final_states)
                .cloned()
                .collect();
        }
        dedup_final_states.push(new_set);
    }
    let final_states2 = dedup_final_states;
    //dbg!(&final_states2);

    for (i, final_state) in final_states2.iter().enumerate() {
        if final_state.is_empty() {
            println!("Warning, regex '{}' has no final state.", source[i]);
        }
    }
    println!(
        "Converted final states in {} ms.",
        start_time.elapsed().unwrap().as_millis()
    );
    let start_time = SystemTime::now();
    /*let dot_graph = dot_generator::DotGraph::from_dfa2(&dfa);
    let mut file = File::create("dfa.dot").unwrap();
    file.write_all(&dot_graph.code).unwrap();*/
    let (mut dfa, dfa_to_hopcroft_map) =
        finite_automaton::set_dfa::SetDFA::hopcroft(&dfa, &final_states2);
    println!(
        "Minimized DFA with Hopcroft algorithm in {} ms.",
        start_time.elapsed().unwrap().as_millis()
    );
    let start_time = SystemTime::now();
    let final_states: Vec<Vec<usize>> = final_states2
        .into_iter()
        .map(|final_dfa_states| {
            dfa_to_hopcroft_map
                .iter()
                .filter_map(|(dfa_states, &hopcroft_state)| {
                    if !final_dfa_states.is_disjoint(dfa_states) {
                        Some(hopcroft_state)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();
    //dbg!(&final_states);
    /*
    let mut final_states_to_token: BTreeMap<usize, usize> = Default::default();
    for (i, states) in final_states4.into_iter().enumerate() {
        for state in states {
            final_states_to_token.insert(state, i);
        }
    }
    */
    for (i, final_state) in final_states.iter().enumerate() {
        if final_state.is_empty() {
            println!("Warning, regex '{}' has no final state.", source[i]);
        }
    }
    println!(
        "Converted final states in {} ms.",
        start_time.elapsed().unwrap().as_millis()
    );
    let start_time = SystemTime::now();
    dfa.remove_trap();
    println!(
        "Removed trap states in {} ms.",
        start_time.elapsed().unwrap().as_millis()
    );
    let start_time = SystemTime::now();
    let backtrack_states = finite_automaton::set_dfa::get_backtrack_states(&dfa);
    println!(
        "Found backtrack states in {} ms.",
        start_time.elapsed().unwrap().as_millis()
    );

    println!(
        "Total time: {} ms.",
        global_start_time.elapsed().unwrap().as_millis()
    );
    (dfa, final_states, backtrack_states)
}
