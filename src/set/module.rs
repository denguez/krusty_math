use crate::cli::*;
use super::*;

lazy_static! {
    static ref SUBSET: String = String::from('\u{2282}');
}

pub fn init() -> Module<Cmd> {
    Module {
        name: "Set theory",
        operations: vec![
            Cmd::of("Power set", "P(A)", power_set, vec![str_list("A")]),
            Cmd::of("Cartesian product", "AxB", cartesian_prod, 
                vec![str_list("A"), str_list("B")]
            ),
            Cmd::of("Number of subsets", "C(n, r)", subsets_of_len,
                vec![uint("n"), uint("r")]
            ),
            Cmd::of("Number of correspondences", &("F ".to_string() + &SUBSET + " P(AxB)"), n_correspondences, 
                vec![str_list("A"), str_list("B")]
            ),
        ]
    }
}

fn power_set(args: ArgList) {
    match args.unwrap() {
        One(StrList(el)) => {
            let set = Set::of_elements(el);
            println!("=> A = {}", &set);

            let power = Set::power(set);
            println!("=> P(A) = {{{}", power);
            println!("=> |P(A)| = {}", power.cardinal())
        },
        _ => {}
    }
}

fn cartesian_prod(args: ArgList) {
    match args.unwrap() {
        Two(StrList(a), StrList(b)) => {
            let set_a = Set::of_elements(a);
            let set_b = Set::of_elements(b);
            println!("=> A = {}", set_a);
            println!("=> B = {}", set_b);

            let prod = cartesian::product(set_a, set_b);
            println!("=> AxB = {}", prod);
            println!("=> |AxB| = {}", prod.cardinal())
        },
        _ => {}
    }
}

pub fn n_correspondences(args: ArgList) {
    match args.unwrap() {
        Two(StrList(a), StrList(b)) => {
            println!("# correspondences = |P(AxB)|");

            let set_a = Set::of_elements(a);
            let set_b = Set::of_elements(b);
            println!("=> A = {}", set_a);
            println!("=> B = {}", set_b);

            let prod = cartesian::product(set_a, set_b);
            println!("=> AxB = {}", prod);
            println!("=> |AxB| = {}", prod.cardinal());

            let corr = cartesian::power_size(prod);
            println!("=> |P(AxB)| = {}", corr);
        }
        _ => {}
    }
}

fn subsets_of_len(args: ArgList) {
    match args.unwrap() {
        Two(UInt(n), UInt(r)) => {
            println!("=> C({}, {}) = {}", n, r, Set::<String>::of_size(n as usize).subsets_of_size(r))
        },
        _ => {}
    }
}

