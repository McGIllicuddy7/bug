pub struct Tokenizer<'a> {
    tokens: Vec<&'a str>,
    index: usize,
}
impl<'a> Tokenizer<'a> {
    pub fn new(v: &'a str) -> Self {
        let lines: Vec<&str> = v.lines().collect();
        let lits: Vec<&str> = lines
            .iter()
            .flat_map(|a| crate::utils::extract_string_literals(a))
            .collect();
        let delims = [" ", "==", "<", ">", "*", "^", "+", "-", "/", "=", ","];
        let mut base = lits;
        for i in delims {
            let mut tmp = Vec::new();
            for j in base {
                if j.starts_with('"') || delims.contains(&j) {
                    tmp.push(j);
                } else {
                    if i == " " {
                        j.split_whitespace().for_each(|s| tmp.push(s));
                    } else {
                        j.split_inclusive(i).for_each(|s| {
                            if let Some(k) = s.strip_suffix(i) {
                                tmp.push(k);
                                tmp.push(i);
                            } else {
                                tmp.push(s);
                            }
                        });
                    }
                }
            }
            base = tmp;
        }
        Self {
            tokens: base,
            index: 0,
        }
    }
}
