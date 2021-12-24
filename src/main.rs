use enigma::Enigma;
use enigma::Rotor;
use enigma::Reflector;
use enigma::Plugboard;

use std::io::{self, Write};

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Enigma")
                    .version("0.1")
                    .author("Jonathan Boyd")
                    .about("A rust implementation of the M3 Enigma machine.")
                    .arg(Arg::new("reflector")
                        .short('R')
                        .long("reflector")
                        .value_name("REFLECTOR")
                        .possible_values(&["Beta", "Gamma", "A", "B", "C", "ThinB", "ThinC", "ETW"])
                        .help("Sets the reflector type.")
                        .takes_value(true)
                        .multiple_occurrences(false)
                        .multiple_values(false)
                        .required(true))
                    .arg(Arg::new("rotors")
                        .short('r')
                        .long("rotors")
                        .value_name("ROTORS")
                        .possible_values(&["I", "II", "III", "IV", "V", "VI", "VII", "VIII"])
                        .help("Sets the rotors in use, starting in the leftmost position.")
                        .takes_value(true)
                        .multiple_occurrences(false)
                        .multiple_values(true)
                        .min_values(3)
                        .max_values(3)
                        .required(true))
                    .arg(Arg::new("rings")
                        .long("rings")
                        .value_name("RINGS")
                        .help("Sets the ring setting for the rotors. Valid values are numbers in the range 1 to 26.")
                        .takes_value(true)
                        .multiple_occurrences(false)
                        .multiple_values(true)
                        .min_values(3)
                        .max_values(3)
                        .required(true))
                    .arg(Arg::new("positions")
                        .short('p')
                        .long("positions")
                        .value_name("POSITIONS")
                        .help("Sets the intial positions for the rotors. Valid values are letters in the range A to Z.")
                        .takes_value(true)
                        .multiple_occurrences(false)
                        .multiple_values(true)
                        .min_values(3)
                        .max_values(3)
                        .required(true))
                    .arg(Arg::new("plugs")
                        .short('s')
                        .long("plugs")
                        .value_name("PLUGS")
                        .help("Adds a set of plugs to the plugboard.")
                        .long_help("Adds a set of plugs to the plugboard. Valid values are pairs of letters such as 'AL' for linking the letter 'A' to the letter 'L'.")
                        .takes_value(true)
                        .multiple_occurrences(false)
                        .multiple_values(true)
                        .max_values(10))
                    .get_matches();

    let reflector_arg = matches.value_of("reflector").unwrap();
    let reflector = Reflector::get_reflector_type_from_string(&reflector_arg).unwrap();

    let rotor_args: Vec<&str> = matches.values_of("rotors").unwrap().collect();
    let ring_settings: Vec<&str> = matches.values_of("rings").unwrap().collect();
    let positions: Vec<&str> = matches.values_of("positions").unwrap().collect();

    let plug_args = match matches.values_of("plugs") {
        Some(p) => p.collect(),
        None => Vec::new(),
    };

    println!("Reflector: {:?}", reflector);
    println!("Rotors: {:?}", rotor_args);
    println!("Ring Settings: {:?}", ring_settings);
    println!("Positions: {:?}", positions);
    println!("Plugs: {:?}", plug_args);

    let mut rotors: Vec<Rotor> = Vec::new();

    for i in 0..rotor_args.len() {

        let rotor_type = Rotor::get_rotor_type_from_string(&rotor_args[i]).unwrap();
        let position = positions[i].chars().nth(0).unwrap();
        let ring = ring_settings[i].parse::<usize>().unwrap();

        match Rotor::new(rotor_type, position, ring) {
            Ok(r) => rotors.push(r),
            Err(err) => panic!("{}", err),
        };
    }

    let mut plugs: Vec<(char, char)> = Vec::new();

    for i in 0..plug_args.len() {
        let mut pair = plug_args[i].chars();
        plugs.push((pair.next().unwrap(), pair.next().unwrap()));
    }

    let plugboard = Plugboard::new(&plugs).unwrap();
    let mut enigma = Enigma::new(reflector, rotors, plugboard);

    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        print!(">");

        std::io::stdout().flush().unwrap();

        match stdin.read_line(&mut buffer) {
            Ok(_) => (),
            Err(err) => panic!("Failed to read console input! Error: {}", err),
        }

        if buffer.trim().eq("exit") {
            break;
        }

        match enigma.encrypt(buffer.trim()) {
            Ok(r) => println!("{}", r),
            Err(err) => eprintln!("[ERROR]: {}", err),
        }

        buffer.clear();
    }
}