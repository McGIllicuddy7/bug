pub fn extract_string_literals(st: &str) -> Vec<&str> {
    unsafe {
        let base = st.as_ptr();
        let mut out = Vec::new();
        let mut ptr = st.as_ptr();
        let mut in_str = false;
        for (idx, c) in st.char_indices() {
            if c == '"' {
                if in_str {
                    let tmp =
                        std::slice::from_raw_parts(ptr, idx - (ptr as usize - base as usize) + 1);
                    if !tmp.is_empty() {
                        out.push(std::str::from_utf8_unchecked(tmp));
                    }
                    ptr = base.add(idx + 1);
                } else {
                    let tmp = std::slice::from_raw_parts(ptr, idx - (ptr as usize - base as usize));
                    if !tmp.is_empty() {
                        out.push(std::str::from_utf8_unchecked(tmp));
                    }
                    ptr = base.add(idx);
                }
                in_str = !in_str;
            }
        }
        let tmp = std::slice::from_raw_parts(ptr, st.len() - (ptr as usize - base as usize));
        if !tmp.is_empty() {
            out.push(std::str::from_utf8_unchecked(tmp));
        }

        out
    }
}
