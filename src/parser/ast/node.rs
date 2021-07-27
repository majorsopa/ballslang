use crate::parser::ast::node_id::NodeId;
use crate::parser::ast::node_type::NodeType;
use std::fmt::{Display, Formatter};
use crate::lexer::token::Token;


#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) children: Vec<NodeId>,

    pub(crate) root: bool,

    pub(crate) node_type: NodeType,

    pub(crate) data: Option<Token>,

    pub(crate) id: NodeId,
}

impl Node {
    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    /*
    pub fn set_data(&mut self, token: Token) {
        self.data = Some(token);
    }


    pub fn add_id(&mut self, amount: usize) {
        self.id.add_value(amount);
    }

    pub fn sub_id(&mut self, amount: usize) {
        self.id.sub_value(amount);
    }

    pub fn set_id(&mut self, amount: usize) {
        self.id.set_value(amount);
    }
     */
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let root_bool_string = String::from(match &self.root {
            true => "true",
            false => "false",
        });

        let mut children_string = String::from('[');
        for child in &self.children {
            children_string.push_str(&*format!("{}, ", child.index.to_string()))
        }
        children_string.push(']');

        let data_string = String::from(match &self.data {
            None => "None".to_string(),
            Some(t) => t.to_string(),
        });

        let node_id_string = String::from(&self.id.to_string());



        write!(
            f,
            "Node {{\n\t\tRoot:{root},\n\t\tChildren:{children_vec},\n\t\tNode Type:{node_type},\n\t\tData:{data},\n\t\tNode Id:{node_id},\n\t}},\n",
            root = root_bool_string,
            children_vec = children_string,
            node_type = self.node_type,
            data = data_string,
            node_id = node_id_string,
        )
    }
}
