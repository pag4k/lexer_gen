//use crate::finite_accepters::*;

extern crate alloc;

use crate::finite_automaton::*;
use alloc::collections::btree_map::BTreeMap;
use alloc::vec::Vec;
use core::fmt::Display;

/// DotGraph ADT
pub struct DotGraph {
    pub code: Vec<u8>,
}

impl DotGraph {
    /// Generate a .gv file based on the section of NFA specified in arguments.
    pub fn from_nfa(
        nfa: &impl NFA<usize>,
        //tokens: &HashMap<usize, TokenType>,
        //backtrack: &HashSet<usize>,
        //(first_state, last_state): (usize, usize),
    ) -> DotGraph {
        let states: Vec<usize> = nfa.states().to_vec();
        let alphabet: Vec<u8> = nfa.alphabet().to_vec();

        let mut dot_graph = DotGraph { code: Vec::new() };
        dot_graph.add_line("digraph finite_state_machine {");
        dot_graph.add_line("\trankdir=LR;");
        dot_graph.add_line("\tsize=\"8,5\"");

        dot_graph.add_line("");

        for state in states.iter().filter(|&&state| nfa.is_final_state(state)) {
            let line = format!(
                "\tnode [shape = rectangle, label=\"{} -> {}\", fontsize=12] token{};",
                state, state, state
            );
            dot_graph.add_line(&line);
        }

        dot_graph.add_line("");

        for &state in &states {
            let node = if nfa.is_final_state(state) {
                "doublecircle"
            } else {
                "circle"
            };
            let color = "black";
            //  if backtrack.contains(&state) {
            //     "red"
            // } else {
            //     "black"
            // };
            let line = format!(
                "\tnode [shape = {}, label=\"{}\", fontsize=12, color={}] {};",
                node, state, color, state
            );
            dot_graph.add_line(&line);
        }

        dot_graph.add_line("");

        let line = "\tnode [shape = point, color=black] q0;";
        dot_graph.add_line(&line);
        let line = format!("\tq0\t->\t{};", nfa.initial_state());
        dot_graph.add_line(&line);

        dot_graph.add_line("");

        for &from in &states {
            if let Some(to_iter) = nfa.next(from, None) {
                for to in to_iter {
                    let line = format!("\t{}\t->\t{}\t[ label = \"{}\" ];", from, to, 'ε');
                    dot_graph.add_line(&line);
                }
            }
            for &input in &alphabet {
                if let Some(to_iter) = nfa.next(from, Some(input)) {
                    for to in to_iter {
                        let line =
                            format!("\t{}\t->\t{}\t[ label = \"{}\" ];", from, to, input as char);
                        dot_graph.add_line(&line);
                    }
                }
            }
        }

        //for (from, to) in dfa.function.iter()
        //.filter(|(from, _)| first_state <= from.0 && from.0 <= last_state)
        //{
        //   let line = format!("\t{}\t->\t{}\t[ label = \"{}\" ];", from.0, to, from.1);
        //dot_graph.add_line(&line);
        //}
        dot_graph.add_line("}");

        dot_graph
    }
    pub fn from_dfa(
        dfa: &impl DFA<usize>,
        final_states_to_token: &BTreeMap<usize, impl Display>,
        backtrack_states: &[usize],
    ) -> DotGraph {
        let states: Vec<usize> = dfa.states().to_vec();
        let alphabet: Vec<u8> = dfa.alphabet().to_vec();

        let mut dot_graph = DotGraph { code: Vec::new() };
        dot_graph.add_line("digraph finite_state_machine {");
        dot_graph.add_line("\trankdir=LR;");
        dot_graph.add_line("\tsize=\"8,5\"");

        dot_graph.add_line("");

        for state in states.iter().filter(|&&state| dfa.is_final_state(state)) {
            let line = format!(
                "\tnode [shape = rectangle, label=\"{} -> {}\", fontsize=12] token{};",
                state, final_states_to_token[state], state
            );
            dot_graph.add_line(&line);
        }

        dot_graph.add_line("");

        for &state in &states {
            let node = if dfa.is_final_state(state) {
                "doublecircle"
            } else {
                "circle"
            };
            let color = if backtrack_states.contains(&state) {
                "red"
            } else {
                "black"
            };
            let line = format!(
                "\tnode [shape = {}, label=\"{}\", fontsize=12, color={}] {};",
                node, state, color, state
            );
            dot_graph.add_line(&line);
        }

        dot_graph.add_line("");

        let line = "\tnode [shape = point, color=black] q0;";
        dot_graph.add_line(&line);
        let line = format!("\tq0\t->\t{};", dfa.initial_state());
        dot_graph.add_line(&line);

        dot_graph.add_line("");

        for &from in &states {
            let mut transitions: BTreeMap<usize, Vec<u8>> = Default::default();
            for &input in &alphabet {
                if let Some(to) = dfa.next(from, input) {
                    transitions.entry(to).or_default().push(input);
                }
            }
            for (to, inputs) in transitions {
                let line = format!(
                    "\t{}\t->\t{}\t[ label = \"{}\" ];",
                    from,
                    to,
                    // TODO: Include the to_escape here.
                    combine_inputs(&inputs)
                );
                dot_graph.add_line(&line);
            }
        }

        //for (from, to) in dfa.function.iter()
        //.filter(|(from, _)| first_state <= from.0 && from.0 <= last_state)
        //{
        //   let line = format!("\t{}\t->\t{}\t[ label = \"{}\" ];", from.0, to, from.1);
        //dot_graph.add_line(&line);
        //}
        dot_graph.add_line("}");

        dot_graph
    }

    pub fn from_dfa2(dfa: &dyn DFA<usize>) -> DotGraph {
        let mut dot_graph = DotGraph { code: Vec::new() };
        let states: Vec<usize> = dfa.states().to_vec();
        let alphabet: Vec<u8> = dfa.alphabet().to_vec();
        dot_graph.add_line("digraph finite_state_machine {");
        dot_graph.add_line("\trankdir=LR;");
        dot_graph.add_line("\tsize=\"8,5\"");

        dot_graph.add_line("");

        for state in dfa
            .states()
            .iter()
            .filter(|&&state| dfa.is_final_state(state))
        {
            let line = format!(
                "\tnode [shape = rectangle, label=\"{} -> {}\", fontsize=12] token{};",
                state, state, state
            );
            dot_graph.add_line(&line);
        }

        dot_graph.add_line("");

        for &state in dfa.states() {
            let node = if dfa.is_final_state(state) {
                "doublecircle"
            } else {
                "circle"
            };
            let color = "black";
            //  if backtrack.contains(&state) {
            //     "red"
            // } else {
            //     "black"
            // };
            let line = format!(
                "\tnode [shape = {}, label=\"{}\", fontsize=12, color={}] {};",
                node, state, color, state
            );
            dot_graph.add_line(&line);
        }

        dot_graph.add_line("");

        let line = "\tnode [shape = point, color=black] q0;";
        dot_graph.add_line(&line);
        let line = format!("\tq0\t->\t{};", dfa.initial_state());
        dot_graph.add_line(&line);

        dot_graph.add_line("");

        for &from in &states {
            let mut transitions: BTreeMap<usize, Vec<u8>> = Default::default();
            for &input in &alphabet {
                if let Some(to) = dfa.next(from, input) {
                    transitions.entry(to).or_default().push(input);
                }
            }
            for (to, inputs) in transitions {
                let line = format!(
                    "\t{}\t->\t{}\t[ label = \"{}\" ];",
                    from,
                    to,
                    // TODO: Include the to_escape here.
                    combine_inputs(&inputs)
                );
                dot_graph.add_line(&line);
            }
        }

        dot_graph.add_line("}");

        dot_graph
    }

    /// Helper function to add a line to the output String.
    fn add_line(&mut self, line: &str) {
        self.code.extend(line.as_bytes());
        self.code.push(b'\n');
    }
}

fn to_escape(byte: u8) -> String {
    match byte {
        9 => String::from("\\t"),
        10 => String::from("\\n"),
        12 => String::from("\\f"),
        13 => String::from("\\r"),
        34 => String::from("\\\""),
        92 => String::from("\\\\"),
        _ => (byte as char).to_string(),
    }
}

// FIXME: This works, but it break the dot file with all the symbols.
fn combine_inputs(inputs: &[u8]) -> String {
    let mut string: String = Default::default();
    for &byte in inputs {
        string.push_str(&to_escape(byte));
    }
    string
}
