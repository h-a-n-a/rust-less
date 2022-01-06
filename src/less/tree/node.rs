pub struct Node {
  parent: Box<Option<Node>>,
  visibilityBlocks: Option<usize>,
  nodeVisible: Option<bool>,
  rootNode: Box<Option<Node>>,
  parsed: Option<Box<dyn Fn()>>,
}

impl Node {}

struct A {}

