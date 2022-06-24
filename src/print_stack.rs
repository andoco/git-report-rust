use std::io::Write;

#[derive(Debug, PartialEq)]
pub enum Node {
    Open(String),
    Continue,
    Terminal(String),
    Empty,
}

pub struct PrintStack<'a> {
    out: &'a mut (dyn Write + 'a),
    nodes: Vec<Node>,
}

impl<'a> PrintStack<'a> {
    pub fn new(out: &'a mut dyn Write) -> PrintStack {
        PrintStack {
            out,
            nodes: Vec::new(),
        }
    }

    pub fn extend(&mut self, status: Node) -> PrintStack {
        let mut new_status: Vec<Node> = self
            .nodes
            .iter()
            .map(|status| match status {
                Node::Open(_) => Node::Continue,
                Node::Continue => Node::Continue,
                Node::Terminal(_) => Node::Empty,
                Node::Empty => Node::Empty,
            })
            .collect();

        new_status.push(status);

        PrintStack {
            out: self.out,
            nodes: new_status,
        }
    }

    pub fn print(&mut self) {
        for node in self.nodes.iter() {
            let s = match node {
                Node::Open(str) => format!("├── {}", str),
                Node::Continue => "│   ".to_string(),
                Node::Terminal(str) => format!("└── {}", str),
                Node::Empty => "    ".to_string(),
            };
            write!(self.out, "{}", s).unwrap();
        }
        writeln!(self.out).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::{io::sink, str::from_utf8};

    use super::*;

    #[test]
    fn test_new_stack_is_empty() {
        assert!(PrintStack::new(&mut sink()).nodes.is_empty());
    }

    #[test]
    fn test_stack_extend_maps_open_to_continue() {
        let mut out = sink();
        let mut stack0 = PrintStack::new(&mut out);
        let mut stack1 = stack0.extend(Node::Open("a".to_string()));
        let stack2 = stack1.extend(Node::Open("b".to_string()));

        assert_eq!(
            stack2.nodes,
            vec![Node::Continue, Node::Open("b".to_string())]
        );
    }

    #[test]
    fn test_stack_extend_maps_continue_to_continue() {
        let mut out = sink();
        let mut stack0 = PrintStack::new(&mut out);
        let mut stack1 = stack0.extend(Node::Continue);
        let stack2 = stack1.extend(Node::Open("a".to_string()));

        assert_eq!(
            stack2.nodes,
            vec![Node::Continue, Node::Open("a".to_string())]
        );
    }

    #[test]
    fn test_stack_extend_maps_terminal_to_empty() {
        let mut out = sink();
        let mut stack0 = PrintStack::new(&mut out);
        let mut stack1 = stack0.extend(Node::Terminal("a".to_string()));
        let stack2 = stack1.extend(Node::Open("b".to_string()));

        assert_eq!(stack2.nodes, vec![Node::Empty, Node::Open("b".to_string())]);
    }

    #[test]
    fn test_stack_extend_maps_empty_to_empty() {
        let mut out = sink();
        let mut stack0 = PrintStack::new(&mut out);
        let mut stack1 = stack0.extend(Node::Empty);
        let stack2 = stack1.extend(Node::Open("a".to_string()));

        assert_eq!(stack2.nodes, vec![Node::Empty, Node::Open("a".to_string())]);
    }

    #[test]
    fn test_stack_print() {
        let mut out = Vec::new();
        PrintStack::new(&mut out)
            .extend(Node::Open("a".to_string()))
            .extend(Node::Open("b".to_string()))
            .extend(Node::Terminal("c".to_string()))
            .print();

        assert_eq!(from_utf8(&out).unwrap(), "│   │   └── c\n");
    }
}
