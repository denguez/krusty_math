lazy_static! {
    pub static ref EQUIV: String = '\u{2261}'.to_string();
    pub static ref PHI: String = '\u{03C6}'.to_string();
    static ref SUP_PARETHESES_RIGHT: String = '\u{207D}'.to_string();
    static ref SUP_PARETHESES_LEFT: String = '\u{207E}'.to_string();
    static ref SUP_MINUS: String = '\u{207B}'.to_string();
    static ref SUP_ONE: String = '\u{00B9}'.to_string();
    static ref SUP_P: String = '\u{1D56}'.to_string();
    static ref SUP_N: String = '\u{207F}'.to_string();
    static ref SUP_PHI: String = '\u{1D60}'.to_string();
    static ref SUB_P: String = '\u{209A}'.to_string();
}


// [a]^-1 mod p
pub fn mod_inverse_expression() -> String {
    "[a]".to_string() + &SUP_MINUS + &SUP_ONE + &EQUIV + &SUB_P + "?"
}

// a^{p-1} mod p = 1
pub fn fermat_formula() -> String {
    let a = "a".to_string();
    a + &SUP_PARETHESES_RIGHT + &SUP_P + &SUP_MINUS + &SUP_ONE + &SUP_PARETHESES_LEFT + &EQUIV + &SUB_P + "1"
    + "\n=> a" + &SUP_N + &EQUIV + &SUB_P + "?"
}

// phi(n)
pub fn euler_totient_expression() -> String {
    PHI.to_string() + "(n) = ?"
}

// a^phi(p) mod p = 1
pub fn euler_formula() -> String {
    let a = "a".to_string();
    a + &SUP_PHI + &SUP_PARETHESES_RIGHT + &SUP_P + &SUP_PARETHESES_LEFT + &EQUIV + &SUB_P + "1"
    + "\n=> a" + &SUP_N + &EQUIV + &SUB_P + "?"
}


