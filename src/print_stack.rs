use std::io::Write;

#[derive(Debug, PartialEq)]
pub enum Node {
    Open,
    Continue,
    Terminal,
    Empty,
}

pub struct PrintStack {
    nodes: Vec<Node>,
}

impl PrintStack {
    pub fn new() -> PrintStack {
        PrintStack { nodes: Vec::new() }
    }

    pub fn extend(&self, status: Node) -> PrintStack {
        let mut new_status: Vec<Node> = self
            .nodes
            .iter()
            .map(|status| match status {
                Node::Open => Node::Continue,
                Node::Continue => Node::Continue,
                Node::Terminal => Node::Empty,
                Node::Empty => Node::Empty,
            })
            .collect();

        new_status.push(status);

        PrintStack { nodes: new_status }
    }

    pub fn print(&self, mut out: impl Write) {
        self.nodes.iter().for_each(|status| {
            let s = match *status {
                Node::Open => "├──",
                Node::Continue => "│  ",
                Node::Terminal => "└──",
                Node::Empty => "   ",
            };
            write!(out, "{}", s).unwrap();
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use super::*;

    #[test]
    fn test_new_stack_is_empty() {
        assert!(PrintStack::new().nodes.is_empty());
    }

    #[test]
    fn test_stack_extend_maps_open_to_continue() {
        let stack = PrintStack::new().extend(Node::Open).extend(Node::Open);
        assert_eq!(stack.nodes, vec![Node::Continue, Node::Open]);
    }

    #[test]
    fn test_stack_extend_maps_continue_to_continue() {
        let stack = PrintStack::new().extend(Node::Continue).extend(Node::Open);
        assert_eq!(stack.nodes, vec![Node::Continue, Node::Open]);
    }

    #[test]
    fn test_stack_extend_maps_terminal_to_empty() {
        let stack = PrintStack::new().extend(Node::Terminal).extend(Node::Open);
        assert_eq!(stack.nodes, vec![Node::Empty, Node::Open]);
    }

    #[test]
    fn test_stack_extend_maps_empty_to_empty() {
        let stack = PrintStack::new().extend(Node::Empty).extend(Node::Open);
        assert_eq!(stack.nodes, vec![Node::Empty, Node::Open]);
    }

    #[test]
    fn test_stack_print() {
        let mut out = Vec::new();

        let stack1 = PrintStack::new();
        stack1.print(&mut out);
        assert_eq!(from_utf8(&out).unwrap(), "");

        let stack2 = stack1.extend(Node::Open);
        out.clear();
        stack2.print(&mut out);
        assert_eq!(from_utf8(&out).unwrap(), "├──");

        let stack3 = stack2.extend(Node::Open);
        out.clear();
        stack3.print(&mut out);
        assert_eq!(from_utf8(&out).unwrap(), "│  ├──");

        let stack4 = stack3.extend(Node::Terminal);
        out.clear();
        stack4.print(&mut out);
        assert_eq!(from_utf8(&out).unwrap(), "│  │  └──");
    }
}
