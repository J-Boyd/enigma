use enigma::Enigma;
use enigma::Rotor;
use enigma::RotorType;
use enigma::ReflectorType;
use enigma::Reflector;
use enigma::Plugboard;

use std::io;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    println!("Reflector:");
    let reflector_type = read_reflector_type(&stdin);

    println!("Rotors:");
    let rotor_types = read_rotor_types(&stdin);

    println!("Ring Settings:");
    let mut ring_settings = read_ring_settings(&stdin);

    while ring_settings.len() != rotor_types.len() {
        ring_settings.clear();

        eprintln!("Incorrect number of ring settings!");
        ring_settings = read_ring_settings(&stdin);
    }

    println!("Positions:");
    let mut positions = read_rotor_positions(&stdin);

    while positions.len() != rotor_types.len() {
        positions.clear();

        eprintln!("Incorrect number of positions!");
        positions = read_rotor_positions(&stdin);
    }

    let mut rotors: Vec<Rotor> = Vec::with_capacity(rotor_types.len());

    for i in 0..rotor_types.len() {
        match Rotor::new(rotor_types[i], positions[i], ring_settings[i]) {
            Ok(r) => rotors.push(r),
            Err(err) => panic!("{}", err), // TODO - Could do a loop around types, positions, ring settings and continue if we fail here.
        };
    }

    println!("Plugs:");
    let plugs = read_plugs(&stdin);

    let plugboard = Plugboard::new(&plugs).unwrap();
    let mut enigma = Enigma::new(reflector_type, rotors, plugboard);

    println!();

    loop {
        println!(">");

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

fn read_reflector_type(stdin: &io::Stdin) -> ReflectorType {
    let mut buffer = String::new();

    match stdin.read_line(&mut buffer) {
        Ok(_) => (),
        Err(err) => panic!("Failed to read console input! Error: {}", err),
    }

    let t = match Reflector::get_reflector_type_from_string(buffer.trim()) {
        Ok(t) => return t,
        Err(err) => {
            eprintln!("{}", err);
            read_reflector_type(stdin)
        }
    };

    t
}

fn read_rotor_types(stdin: &io::Stdin) -> Vec<RotorType> {
    let mut buffer = String::new();

    match stdin.read_line(&mut buffer) {
        Ok(_) => (),
        Err(err) => panic!("Failed to read console input! Error: {}", err),
    };

    let split = buffer.split_whitespace();

    let mut rotor_types: Vec<RotorType> = Vec::new();

    for rt in split {
        match Rotor::get_rotor_type_from_string(rt) {
            Ok(t) => rotor_types.push(t),
            Err(err) => {
                rotor_types.clear();

                eprintln!("{}", err);
                rotor_types = read_rotor_types(&stdin);
            },
        }
    }

    rotor_types
}

fn read_ring_settings(stdin: &io::Stdin) -> Vec<usize> {
    let mut ring_settings: Vec<usize> = Vec::new();
    let mut buffer = String::new();

    match stdin.read_line(&mut buffer) {
        Ok(_) => (),
        Err(err) => panic!("Failed to read console input! Error: {}", err),
    }
    
    let split = buffer.split_whitespace();

    for r in split {
        match r.parse::<usize>() {
            Ok(r) => { 
                ring_settings.push(r);
            },
            Err(err) => {
                ring_settings.clear();

                eprintln!("{}", err);
                ring_settings = read_ring_settings(&stdin);
            },
        };
    }

    ring_settings
}

fn read_rotor_positions(stdin: &io::Stdin) -> Vec<char> {
    let mut positions: Vec<char> = Vec::new();
    let mut buffer = String::new();

    match stdin.read_line(&mut buffer) {
        Ok(_) => (),
        Err(err) => panic!("Failed to read console input! Error: {}", err),
    }

    let split = buffer.split_whitespace();

    for p in split {
        match p.chars().next() {
            Some(c) => positions.push(c),
            None => {
                positions.clear();

                eprintln!("Failed to read rotor position!");
                positions = read_rotor_positions(&stdin);
            },
        }
    }

    positions
}

fn read_plugs(stdin: &io::Stdin) -> Vec<(char, char)> {
    let mut plugs: Vec<(char, char)> = Vec::new();
    let mut buffer = String::new();

    match stdin.read_line(&mut buffer) {
        Ok(_) => (),
        Err(err) => panic!("Failed to read console input! Error: {}", err),
    }

    let split = buffer.split_whitespace();

    for pair in split {
        let mut iter = pair.chars();

        let p1 =  match iter.next() {
            Some(c) => c,
            None => {
                plugs.clear();

                eprintln!("Failed to read plugs!");
                plugs = read_plugs(&stdin);

                break;
            },
        };

        let p2 =  match iter.next() {
            Some(c) => c,
            None => {
                plugs.clear();

                eprintln!("Failed to read plugs!");
                plugs = read_plugs(&stdin);

                break;
            },
        };

        plugs.push((p1, p2));
    }

    plugs
}