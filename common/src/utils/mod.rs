pub mod validator;

fn join_vec<S: ToString>(l: Vec<S>, sep: &str) -> String {
    l.iter().fold(
        String::new(),
        |a, b| if !a.is_empty() { a + sep } else { a } + &b.to_string(),
    )
}
