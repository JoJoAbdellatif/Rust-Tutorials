// Rust lifetime elision rules are a set of rules the compiler uses to automatically infer lifetimes when they are not explicitly annotated.
// These rules do not give compilation errors if they apply to the code.
// There are three main rules (in order) that the compiler applies only to function and method signatures.

// Rule 1:
// Each reference in an argument gets its own lifetime specifier.
// The following function is equivalent to writing:
    // "pub fn rule1<'a , 'b>(f_name: &'a str , _l_name: &'b str) -> String{"
pub fn rule1(f_name: &str , _l_name: &str) -> String{
    f_name.to_string()
}

// Rule 2: 
// If there is exactly one input reference, then that lifetime is assigned to all output references.
pub fn rule2(f_name: &str) -> &str{
    f_name
}

// Rule 3:
// If there are multiple input references, but the method uses "&self" or "&mut self", then the lifetime of self is assigned to all output references.

struct _Person <'a> {
    name: &'a str,
}

impl<'a> _Person<'a> {
    fn _get_name(&self, _prefix: &str) -> &str {
        self.name
    }
}