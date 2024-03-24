#[derive(Debug)]
enum People {
    Stanley {
        age: u32,
        ppsize: u32,
    },
    Chen {
        age: u32,
        ppsize: u32,
        holyness: u32,
    },
    Kuan,
    Lai,
    Poon,
}

pub fn three() -> i32 {
    3
}

pub fn bullshit() -> bool {
    let s = People::Stanley {
        age: 20,
        ppsize: 10,
    };

    let c = People::Chen {
        age: 20,
        ppsize: 10,
        holyness: 10000,
    };

    let mut peeps = Vec::new();
    peeps.push(s);
    peeps.push(c);

    // print peeps
    for p in peeps.iter() {
        println!("{:?}", p);
    }

    for p in peeps {
        match p {
            People::Chen {
                age,
                ppsize,
                holyness,
            } => {
                println!("haha chen fuk u");
            }
            People::Stanley { age, ppsize } => {
                println!("no u");
            }
            _ => {}
        }
    }

    true
}
