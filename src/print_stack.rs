#[derive(Debug)]
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

    pub fn print(&self) {
        self.nodes.iter().for_each(|status| {
            let s = match *status {
                Node::Open => "├──",
                Node::Continue => "│  ",
                Node::Terminal => "└──",
                Node::Empty => "   ",
            };
            print!("{}", s);
        })
    }
}
