use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Height {
    Short,
    Medium,
    Tall
}

struct Tree {
    color: String,
    height: Height,
}

struct TreeFactory {
    trees: HashMap<String, Rc<Tree>>,
}

impl TreeFactory {
    fn new() -> Self {
        Self {
            trees: HashMap::new(),
        }
    }

    fn get_tree(&mut self, color: String, height: Height) -> Rc<Tree> {
        let key = format!("{}:{:?}", color, height);

        if let Some(t) = self.trees.get(&key) {
            return Rc::clone(t);
        }

        let new_tree = Rc::new(Tree {color, height});
        self.trees.insert(key, Rc::clone(&new_tree));
        new_tree
    }
}

pub struct Forest {
    trees: Vec<Rc<Tree>>,
    factory: TreeFactory
}

impl Forest {
    pub fn new() -> Self {
        Self {
            trees: vec![],
            factory: TreeFactory::new(),
        }
    }

    pub fn add_tree(&mut self, color: String, height: Height) {
        let new_tree = self.factory.get_tree(color, height);
        self.trees.push(new_tree);
    }
    
    #[allow(dead_code)]
    fn display(&self) {
        for tree in &self.trees {
            println!("color: {}, height: {:?}", tree.color, tree.height);
        }
    }

    pub fn ret_all_short(&self) -> u32 {
        let mut short_tree_num = 0;
        for tree in &self.trees {
            if tree.height == Height::Short {
                short_tree_num += 1;
            }
        }
        short_tree_num
    }
}
