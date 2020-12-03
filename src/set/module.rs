use crate::cli::*;
use super::*;

pub fn init() -> Module<Cmd> {
    Module {
        name: "Set theory",
        operations: vec![
            Cmd::of("Power set", "P(A)", power_set, vec![str_list("elements")]),
            Cmd::of("Number of subsets of r size", "C(n, r)", subsets_of_len,
                vec![uint("n"), uint("r")]
            ),
            Cmd::of("Cartesian product", "AxB", power_set, 
                vec![str_list("A"), str_list("B")]
            ),
        ]
    }
}

fn power_set(args: ArgList) {
    match args.unwrap() {
        One(StrList(el)) => {
            let set = Set::elements(el);
            println!("=> {}", set);
            println!("=> {}", Set::power(set))
        },
        _ => {}
    }
}

fn subsets_of_len(args: ArgList) {
    match args.unwrap() {
        Two(UInt(n), UInt(r)) => {
            println!("=> C({}, {}) = {}", n, r, set::subsets_of_len(n, r))
        },
        _ => {}
    }
}

