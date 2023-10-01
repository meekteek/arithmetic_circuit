use crate::enums::{CustomU64, ExprVal};
use log::debug;
use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    rc::Rc,
};

/// Represents a node in the arithmetic circuit.
///
/// A node can either hold a specific value or be an expression
/// based on other nodes. Each node can also have children,
/// which are other nodes that contribute to its value or expression.
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Node {
    pub value: CustomU64,
    pub children: Option<Vec<Rc<RefCell<Node>>>>,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            CustomU64::Val(val) => write!(f, "{}", val),
            CustomU64::Expr(expr) => match expr {
                ExprVal::Add(val) => match val {
                    Some(val) => write!(f, "Add(Unevaluated with val {})", val),
                    None => write!(f, "Add(Unevaluated)"),
                },
                ExprVal::Mul(val) => match val {
                    Some(val) => write!(f, "Mul(Unevaluated with val {})", val),
                    None => write!(f, "Mul(Unevaluated))"),
                },
                ExprVal::Input => write!(f, "Input"),
            },
        }
    }
}

impl Node {
    /// Creates a new input node.
    ///
    pub(crate) fn init() -> Self {
        Node {
            value: CustomU64::Expr(ExprVal::Input),
            children: Some(vec![]),
        }
    }

    /// Creates a new node with a specific value.
    ///
    /// # Arguments
    ///
    /// * `value`: The value to initialize the node with.
    pub(crate) fn new(value: u64) -> Self {
        Node {
            value: CustomU64::Val(value),
            children: Some(vec![]),
        }
    }

    /// Recursively evaluates the expressions of a node's children.
    ///
    ///
    /// If a child node contains an addition (`Add`) or multiplication (`Mul`) expression
    /// with a value, it is resolved to a definite value (`Val`).
    ///
    pub(crate) fn evaluate_children(&mut self) {
        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                let mut child = child.borrow_mut();
                child.evaluate_children();
                match &child.value {
                    CustomU64::Expr(ExprVal::Add(Some(val))) => {
                        child.value = CustomU64::Val(*val);
                    }
                    CustomU64::Expr(ExprVal::Mul(Some(val))) => {
                        child.value = CustomU64::Val(*val);
                    }
                    _ => {
                        panic!("An input node is a child. This is not possible.");
                    }
                }
            }
        }
    }

    /// Combines two nodes using addition.
    ///
    /// If either node represents an unevaluated expression,
    /// the result will also be an unevaluated expression and new node will
    /// be added as a child to the unevaluated node.
    /// A constraint is also added during the addition process
    ///
    /// # Arguments
    ///
    /// * `a`: The first node.
    /// * `b`: The second node.
    ///
    /// # Returns
    ///
    /// A new node representing the sum of the two input nodes.
    pub(crate) fn add(mut a: Node, mut b: Node) -> Node {
        let mut node = Node::default();
        match (&a.value, &b.value) {
            (CustomU64::Val(a_val), CustomU64::Val(b_val)) => {
                node = Node::new(a_val + b_val);
            }
            (CustomU64::Val(a_val), CustomU64::Expr(_)) => {
                node = Node {
                    value: CustomU64::Expr(ExprVal::Add(Some(*a_val))),
                    children: Some(vec![]),
                };
                if let Some(children) = &mut b.children {
                    children.push(Rc::new(RefCell::new(node.clone())));
                }
            }
            (CustomU64::Expr(_), CustomU64::Val(b_val)) => {
                node = Node {
                    value: CustomU64::Expr(ExprVal::Add(Some(*b_val))),
                    children: Some(vec![]),
                };
                if let Some(children) = &mut a.children {
                    children.push(Rc::new(RefCell::new(node.clone())));
                }
            }
            (CustomU64::Expr(_), CustomU64::Expr(_)) => {
                node = Node {
                    value: CustomU64::Expr(ExprVal::Add(None)),
                    children: Some(vec![]),
                };
                if let Some(children) = &mut a.children {
                    children.push(Rc::new(RefCell::new(node.clone())));
                }
                if let Some(children) = &mut b.children {
                    children.push(Rc::new(RefCell::new(node.clone())));
                }
            }
        }
        debug!("add node: {} generated from {} and {}", node, a, b);
        node
    }

    /// Combines two nodes using multiplication.
    ///
    /// This method follows similar logic to `add`, but with multiplication.
    /// A constraint is also added during the multiplication process
    ///
    /// # Arguments
    ///
    /// * `a`: The first node.
    /// * `b`: The second node.
    ///
    /// # Returns
    ///
    /// A new node representing the product of the two input nodes.
    pub(crate) fn mul(mut a: Node, mut b: Node) -> Node {
        let mut node = Node::default();
        match (&a.value, &b.value) {
            (CustomU64::Val(a_val), CustomU64::Val(b_val)) => {
                node = Node::new(a_val * b_val);
            }
            (CustomU64::Val(a_val), CustomU64::Expr(_)) => {
                node = Node {
                    value: CustomU64::Expr(ExprVal::Mul(Some(*a_val))),
                    children: Some(vec![]),
                };
                if let Some(children) = &mut b.children {
                    children.push(Rc::new(RefCell::new(node.clone())));
                }
            }
            (CustomU64::Expr(_), CustomU64::Val(b_val)) => {
                node = Node {
                    value: CustomU64::Expr(ExprVal::Mul(Some(*b_val))),
                    children: Some(vec![]),
                };
                if let Some(children) = &mut a.children {
                    children.push(Rc::new(RefCell::new(node.clone())));
                }
            }
            (CustomU64::Expr(_), CustomU64::Expr(_)) => {
                node = Node {
                    value: CustomU64::Expr(ExprVal::Mul(None)),
                    children: Some(vec![]),
                };
                if let Some(children) = &mut a.children {
                    children.push(Rc::new(RefCell::new(node.clone())));
                }
                if let Some(children) = &mut b.children {
                    children.push(Rc::new(RefCell::new(node.clone())));
                }
            }
        }
        debug!("mul node: {} generated from {} and {}", node.clone(), a, b);
        node
    }
}
