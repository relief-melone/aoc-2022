use std::{collections::{ HashSet, BTreeSet }, hash::Hash, time::Instant };

use input_reader::read_lines;

mod input_reader;

static MAX_TIME_ELAPSED:u8 = 30;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve  {
    name: (char, char),
    flow_rate: u16,
    possible_destinations: Vec<(char,char)>
}
impl Valve {
    fn parse_from_line(line: String) -> Self {
        let split_line= line.split(' ').collect::<Vec<&str>>();
        let name_str = split_line[1].to_string();
        let mut name_iter = name_str.chars();
        let name = (name_iter.next().unwrap(), name_iter.next().unwrap());
        let flow_rate = *&split_line[4]
            .split('=')
            .map(|s| { s[..s.len()-1].to_string() })
            .collect::<Vec<String>>()[1]
            .parse::<u16>()
            .unwrap();

        let possible_destinations = (split_line[9..])
        .iter()
        .map(|&raw_name| {
            let mut i = raw_name.chars();
            (i.next().unwrap(), i.next().unwrap()) 
        })
        .collect::<Vec<(char,char)>>();

        Self {
            name,
            flow_rate,
            possible_destinations,
        }
    }
}

#[derive(Debug, Clone)]
// #[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct VolcanoState {
    open_valves: BTreeSet<(char,char)>,
    current_positon: (char,char),
    current_flow: u16,
    pressure_released: u16,
    time_left: u8
}
impl Hash for VolcanoState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // self.open_valves.hash(state);
        // self.current_positon.hash(state);
        self.current_flow.hash(state);
        self.pressure_released.hash(state);
        self.time_left.hash(state);
    }
}
impl PartialEq for VolcanoState {
    fn eq(&self, other: &Self) -> bool {
        // self.open_valves == other.open_valves
        self.current_positon == other.current_positon        
        // && self.current_flow == other.current_flow
        // self.time_left == other.time_left
    }
}
impl Eq for VolcanoState {}

impl VolcanoState {
    fn new() -> Self {
        Self {
            open_valves: BTreeSet::new(),
            current_positon: ('A','A'),
            pressure_released: 0,
            time_left: MAX_TIME_ELAPSED,
            current_flow: 0,
        }
    }

    fn pressure_tick(&mut self){
        self.pressure_released += self.current_flow;
    }

    fn move_to(&self, valve_to_move_to: &(char, char), graph: &VolcanoStateGraph) -> Self {
        let mut s = self.clone();
        s.current_flow = self.get_total_flow_rate(graph);        
        s.pressure_tick();

        let all_valves_opened = graph.valves_with_flow
        .iter()
        .fold(true, |acc, v|{
            if !acc {
                false
            } else {
                self.open_valves.contains(&v.name)
            }
        });

        let current_positon = if all_valves_opened {
            self.current_positon.clone()
        } else {
            valve_to_move_to.clone()
        };
        s.current_positon = current_positon;
        s.time_left = self.time_left - 1;

        s
    }

    fn open_valve(&self, valve: &(char,char), graph: &VolcanoStateGraph) -> Self {
        let mut s = self.clone();
        s.current_flow = self.get_total_flow_rate(graph);
        s.pressure_tick();        
        s.open_valves.insert(valve.clone());
        s.time_left = self.time_left - 1;

        s
    }

    fn get_total_flow_rate(&self, graph: &VolcanoStateGraph) -> u16{
        self.open_valves.iter().fold(0, |acc, cur| {
            acc + graph.get_valve(cur).flow_rate
        })
    }

}

#[derive(Debug)]
struct VolcanoStateGraph {
    valves: Vec<Valve>,
    visited_states: HashSet<VolcanoState>,
    to_visit: Vec<VolcanoState>,
    valves_with_flow: Vec<Valve>
}
impl VolcanoStateGraph {
    fn new(input: Vec<String>) -> Self{
        let valves = input
        .iter()
        .map(|l| {
            Valve::parse_from_line(l.to_string())
        })
        .collect::<Vec<Valve>>();
        let initial_state = VolcanoState::new();
        
        let valves_with_flow = valves
        .clone()
        .into_iter()
        .filter(|v| v.flow_rate > 0)
        .collect::<Vec<Valve>>();

        Self {
            valves,
            visited_states: HashSet::new(),
            to_visit: vec![initial_state],
            valves_with_flow
        }
    }

    fn get_valve(&self, valve_name: &(char,char)) -> Valve {
        self.valves.iter().find(|&v| v.name == *valve_name ).unwrap().clone()
    }

    fn get_current_valve(&self, state: &VolcanoState) -> Valve {
        self.get_valve(&state.current_positon)
    }

    fn get_max_pressure(&self) -> u16 {
        self
        .visited_states
        .iter()
        .fold(0, |acc,vs|{
            if vs.pressure_released > acc {
                vs.pressure_released
            } else {
                acc
            }
        })
    }

    fn run(&mut self){

        // let mut i:usize = 0;
        
        while let Some(s) = self.to_visit.pop() {
            self.visited_states.insert(s.clone());

            // if i % 100000 == 0 {
            //     println!("{}: \tto_visit: {:?}, current_pressure: {}, valves_open: {}, time_left: {}, max_pressure: {}, visited: {}", i , self.to_visit.len(), s.pressure_released, s.open_valves.len(), s.time_left, self.get_max_pressure(), self.visited_states.len());
            // }

            if s.time_left == 0 { continue; }

            let current_valve = self.get_current_valve(&s);
            // i += 1;

            self
            .get_valve(&s.current_positon)
            .possible_destinations
            .iter()
            .for_each(|pd|{
                let new_states = match (
                    s.open_valves.contains(&current_valve.name), 
                    current_valve.flow_rate, 
                ) {                    
                    (true, _, )
                    | (_,0 ) => 
                        vec![
                            s.move_to(pd, self)
                        ],
                    (false, _) =>
                        vec![
                            s.move_to(pd, self),
                            s.open_valve(&current_valve.name, self)
                        ]
                };
                
                for new_state in new_states {
                    if !self.visited_states.contains(&new_state) {
                        self.to_visit.push(new_state.clone());
                    }
                }
            });
        }
    }
}

pub fn part_01(path: Option<&str>) -> u16{
    let started = Instant::now();
    let input = read_lines(path.unwrap_or("assets/input.txt"));
    let mut graph = VolcanoStateGraph::new(input);

    graph.run();

    println!("Part 01 - max pressure release: {} ", graph.get_max_pressure());
    println!("Part 01 - execution time: {:?} ", Instant::now() -  started);

    graph.get_max_pressure()
    
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn volcano_state_01(){
        let mut state_01 = VolcanoState{
            current_positon: ('A','A'),
            open_valves: BTreeSet::new(),
            pressure_released: 50,
            current_flow: 30,
            time_left: 40,
        };
        state_01.open_valves.insert(('B','B'));
        state_01.open_valves.insert(('A','A'));
        state_01.open_valves.insert(('C','C'));
        state_01.open_valves.remove(&('C','C'));
        

        let mut state_02 = VolcanoState{
            current_positon: ('A','A'),
            open_valves: BTreeSet::new(),
            pressure_released: 50,
            current_flow: 30,
            time_left: 40,
        };
        state_02.open_valves.insert(('A','A'));
        state_02.open_valves.insert(('B','B'));

        let mut hs:HashSet<VolcanoState> = HashSet::new();
        hs.insert(state_01.clone());

        assert!(hs.contains(&state_02));

    }
    
    #[test]
    fn part_01_01(){
        let max_pressure_released = part_01(Some("assets/input_test_01.txt"));        

        assert_eq!(max_pressure_released, 1651)
    }
}