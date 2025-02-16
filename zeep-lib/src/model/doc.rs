use crate::model::{node::RustNode, TargetNamespace};
use std::rc::Rc;

#[derive(Default)]
pub struct RustDocument {
    pub target_namespaces: Vec<Rc<TargetNamespace>>,
    pub current_target_namespace: Option<Rc<TargetNamespace>>,
    pub nodes: Vec<RustNode>,
}

impl RustDocument {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn switch_to_target_namespace(&mut self, namespace: &str) {
        // check if the namespace is already in the list
        if !self.target_namespaces.iter().any(|ns| ns.namespace == namespace) {
            let tns = Rc::new(TargetNamespace {
                abbreviation: make_abbreviated_namespace(namespace, &self.target_namespaces),
                namespace: namespace.to_string(),
            });
            self.target_namespaces.push(tns.clone());
            self.current_target_namespace = Some(tns);
        }
    }
}

fn make_abbreviated_namespace(namespace: &str, existing_namespaces: &[Rc<TargetNamespace>]) -> String {
    fn take_three_chars_max(namespace: &str) -> String {
        namespace.chars().take(3).collect()
    }

    let append: Option<u8> = None;

    loop {
        let mut abbreviation = if let Some(last_segment) = namespace.split('/').next_back() {
            if let Some(slashed) = last_segment.split('-').next_back() {
                take_three_chars_max(slashed)
            } else {
                take_three_chars_max(last_segment)
            }
        } else {
            take_three_chars_max(namespace)
        };

        if let Some(append) = append {
            abbreviation.push_str(&append.to_string());
        }

        let abbreviation = abbreviation.to_lowercase();

        if !existing_namespaces.iter().any(|ns| ns.abbreviation == abbreviation) {
            return abbreviation;
        }

        let append = match append {
            None => 1,
            Some(n) => n + 1,
        };

        assert_ne!(append, 255, "Too many namespaces with the same abbreviation");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_abbreviated_namespace() {
        let abbr = make_abbreviated_namespace("https://example.com/a", &[]);
        assert_eq!(abbr, "a");

        let abbr = make_abbreviated_namespace("http://www.w3.org/2001/XMLSchema", &[]);
        assert_eq!(abbr, "xml");

        let abbr = make_abbreviated_namespace("http://www.w3.org/2001/XMLSchema-instance", &[]);
        assert_eq!(abbr, "ins");

        let existing_namespaces = vec![Rc::new(TargetNamespace {
            namespace: "http://www.w3.org/2001/XMLSchema-instance".to_string(),
            abbreviation: abbr,
        })];

        let abbr = make_abbreviated_namespace("http://www.w3.org/2001/XMLSchema-instance", &existing_namespaces);
        assert_eq!(abbr, "ins1");
    }
}
