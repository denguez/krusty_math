use super::*;
use crate::cli::*;

pub fn init() -> Module<Cmd> {
    Module {
        name: "Divisibility",
        operations: vec![
            Cmd::binary("Quotient", "a : b = ?", quotient),
            Cmd::binary("Rest", "a % b = ?", rest),
            Cmd::of("Rest (Fermat theorem)", &fermat_formula(), fermat_theorem, 
                vec![int("a"), uint("n"), uint("p")],
            ),
            Cmd::of("Rest (Euler theorem)", &euler_formula(), euler_rest,
                vec![int("a"), uint("n"), uint("p")],
            ),
            Cmd::binary("MCD", "mcd(a, b) = ?", mcd),
            Cmd::binary("MCM", "mcm(a, b) = ?", mcm),
            Cmd::of("Modular inverse", &mod_inverse_expression(), mod_inverse, 
                vec![int("a"), uint("p")],
            ),
            Cmd::binary("Bezout", "au + bv = mcd(a, b)", bezout),
            Cmd::of("Diophantine equation", "ax + by = c", diophantine, 
                vec![int("a"), int("b"), int("c")],
            ),
            Cmd::of("Euler totient function", &euler_totient_expression(), euler_totient,
                vec![uint("n")],
            ),
            Cmd::of("Chinese algorithm", "", chinese_algorithm,
                vec![uint_list("modulos"), int_list("rests")],
            ),
            Cmd::of("Convert to decimal", "", convert_to_decimal, 
                vec![uint("base"), str("number")],
            ),
            Cmd::of("Convert to base", "", convert_to_base, 
                vec![int("number"), uint("base")],
            ),
        ]
    }
}


pub fn quotient(args: ArgList) {
    match args.unwrap() {
        Two(Int(a), Int(b)) => {
            println!("=> {} / {} = {}", a, b, division::quotient(a, b))
        }
        _ => {}
    }
}

pub fn rest(args: ArgList) {
    match args.unwrap() {
        Two(Int(a), Int(b)) => {
            println!("=> {} % {} = {}", a, b, division::rest(a, b))
        },
        _ => {}
    }
}

pub fn convert_to_decimal(args: ArgList) {
    match args.unwrap() {
        Two(UInt(radix), Str(n)) => {
            println!("=> base {} to decimal", radix);
            println!("=> {} => {}", n, base::to_decimal(&n, radix));
        },
        _ => {}
    }
}

pub fn convert_to_base(args: ArgList) {
    match args.unwrap() {
        Two(Int(n), UInt(radix)) => {
            println!("=> decimal to base {}", radix);
            println!("=> {} => {}", n, base::to_radix(n, radix));
        },
        _ => {}
    }
}

pub fn mcd(args: ArgList) {
    match args.unwrap() {
        Two(Int(a), Int(b)) => {
            println!("=> mcd({}, {}) = {}", a, b, euclid::mcd(a, b))
        },
        _ => {}
    }
}

pub fn mcm(args: ArgList) {
    match args.unwrap() {
        Two(Int(a), Int(b)) => {
            println!("=> mcm({}, {}) = {}", a, b, euclid::mcm(a, b))
        },
        _ => {}
    }
}

pub fn bezout(args: ArgList) {
    match args.unwrap() {
        Two(Int(a), Int(b)) => {
            let mut eq = bezout::equation(a, b);
            println!("{}u + {}v = mcd({}, {})", a, b, a, b);
            println!("{}", eq);
        
            let (u, v) = eq.solve();
            println!("=> {}({}) + {}({}) = {}", a, u, b, v, euclid::mcd(a, b));
            println!("=> u = {}\n=> v = {}", u, v);
        },
        _ => {}
    }
}

pub fn diophantine(args: ArgList) {
    match args.unwrap() {
        Three(Int(a), Int(b), Int(c)) => {
            println!("=> {}x + {}y = {}", a, b, c);
            println!("=> {}", diophantine::solve(a, b, c));
        },
        _ => {}
    }
}

pub fn mod_inverse(args: ArgList) {
    match args.unwrap() {
        Two(Int(n), UInt(radix)) => {
            match modular::inverse(n, radix) {
                InverseMod::Class(r) => {
                    println!("=> [{}]^-1 mod {} = {}", n, radix, r)
                }
                InverseMod::NotCoprime(a, b) =>  println!("=> {} and {} are not coprime", a, b),
            }
        },
        _ => {}
    }
}

fn chinese_algorithm(args: ArgList) {
    match args.unwrap() {
        Two(UIntList(mods), IntList(rests)) => {
            for i in 0..mods.len() {
                println!(" · x {} {} (mod {})", EQUIV.to_string(), rests[i], mods[i]);
            }
            match modular::chinese_algorithm(&mods, &rests) {
                ChineseAlg::Result(rest, modulo) => {
                    println!("=> x {} {} (mod {})", EQUIV.to_string(), rest, modulo);
                    println!("=> x = {} + {}t", rest, modulo);
                },
                ChineseAlg::NonCoprimes(vec) => {
                    println!("=> modulos are not coprime 2 by 2 {:?}", vec);
                }
            }
        },
        _ => {}
    }
}

fn fermat_theorem(args: ArgList) {
    match args.unwrap() {
        Three(Int(base), UInt(exp), UInt(modulo)) => {
            match modular::fermat_rest(base, exp, modulo) {
                FermatResult::Ok(rest) => {
                    println!("=> {}^{} {} {} (mod {})", base, exp, EQUIV.to_string(), rest, modulo);
                }, 
                FermatResult::NotPrime => println!("=> p: {} isn't prime", modulo)
            };
        }, _ => {}
    }
}

fn euler_totient(args: ArgList) {
    match args.unwrap() {
        One(UInt(n)) => {
            println!("=> {}({}) = {}", PHI.to_string(), n, prime::euler_totient(n));
        },
        _ => {}
    }
}

fn euler_rest(args: ArgList) {
    match args.unwrap() {
        Three(Int(base), UInt(exp), UInt(modulo)) => {
            let rest = modular::euler_rest(base, exp, modulo);
            println!("=> {}^{} {} {} (mod {})", base, exp, EQUIV.to_string(), rest, modulo);
        }, _ => {}
    }
}