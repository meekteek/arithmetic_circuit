use crate::enums::{Constraints, CustomU64, ExprVal};
use crate::Node;
use env_logger;
use log::{debug, info};
use std::vec;

/// Builder is used for constructing and managing circuits.
///
/// it is responsible for aggregating nodes (as inputs),
/// managing constraints (gates created through addition or mulitplication and manually added ones),
/// and maintaining a reptresentation of the full computaion graph.
///
pub struct Builder {
    inputs: Vec<Node>,
    pub(crate) constraints: Vec<Constraints>,
    full_graph: Vec<Node>,
}
impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    pub fn new() -> Self {
        env_logger::try_init().unwrap_or_default();
        Builder {
            inputs: vec![],
            constraints: vec![],
            full_graph: vec![],
        }
    }

    /// given the function `F(x,y) = x^2 + y^2 + 5`,
    /// `x` and `y` are inputs and can be initialized using this method.
    ///
    /// # Example
    ///
    /// ```
    /// use arithmetic_circuit::builder::Builder;
    /// let mut builder = Builder::new();
    /// let x = builder.init();
    /// let y = builder.init();
    /// ```
    pub fn init(&mut self) -> Node {
        let node = Node::init();
        self.inputs.push(node.clone());
        self.full_graph.push(node.clone());
        debug!("Initialized input node: {}", node);
        node
    }

    /// Initializes constants in the graph.
    ///
    /// given the function `F(x,y) = x^2 + y^2 + 5`,
    /// `5` is a constant and can be initialized using this method.
    ///
    /// # Arguments
    ///
    /// * `value`: The constant value to be initialized.
    ///
    /// # Example
    ///
    /// ```
    /// use arithmetic_circuit::builder::Builder;
    /// let mut builder = Builder::new();
    /// let five = builder.constant(5);
    /// ```
    pub fn constant(&mut self, value: u64) -> Node {
        let node = Node::new(value);
        self.full_graph.push(node.clone());
        debug!("Initialized node with constant value: {}", node);
        node
    }

    /// Adds two nodes in the graph, producing a new node as the result.
    /// There will also be a new constraint added to the graph.
    ///
    /// If one of the nodes represents an unevaluated expression, the result
    /// will also be an unevaluated expression. If both nodes are evaluated values,
    /// the result will be an evaluated sum of both nodes.
    ///
    /// # Arguments
    ///
    /// * `a`: The first node.
    /// * `b`: The second node.
    ///
    /// # Returns
    ///
    /// A new node representing the sum of the two input nodes.
    pub fn add(&mut self, a: Node, b: Node) -> Node {
        let node = Node::add(a.clone(), b.clone());
        self.constraints.push(Constraints::Add(a, b, node.clone()));
        self.full_graph.push(node.clone());
        node
    }

    /// Multiplies two nodes in the graph, producing a new node as the result.
    /// There will also be a new constraint added to the graph.
    /// This follows similar logic as the `add` method.
    ///
    /// # Arguments
    ///
    /// * `a`: The first node.
    /// * `b`: The second node.
    ///
    /// # Returns
    ///
    /// A new node representing the product of the two input nodes.
    pub fn mul(&mut self, a: Node, b: Node) -> Node {
        let node = Node::mul(a.clone(), b.clone());
        self.constraints.push(Constraints::Mul(a, b, node.clone()));
        self.full_graph.push(node.clone());
        node
    }

    /// Creates assertion that two nodes are equal in value.
    ///
    /// This creates a new constraint of equality between two nodes that will be checked when
    /// `check_constraints` is called.
    ///
    /// # Arguments
    ///
    /// * `a`: The first node.
    /// * `b`: The second node.
    ///
    pub fn assert_equal(&mut self, a: Node, b: Node) {
        self.constraints.push(Constraints::Eq(a.clone(), b.clone()));
        debug!("equality constraint between {:?} and {:?} added", a, b);
    }

    /// Evaluates the nodes using the provided inputs.
    ///
    /// This assigns the provided inputs to the input nodes and then evaluates the
    /// arithmetic expressions represented by child nodes.
    ///
    /// # Arguments
    ///
    /// * `input`: A slice of values meant to be assigned to input nodes. number of inputs supplied
    /// must equal number of input nodes.
    ///
    /// # Behavior
    ///
    /// The method does the following:
    /// 1. Assigns values from the `input` slice to the input nodes.
    /// 2. Iterates through the input nodes and their children and evaluates the children node's
    /// values or partially evaluates them.
    /// 3. Lastly, it calls `evaluate_children` on all partially evaluated nodes to ensure the
    /// graph is evaluated completely.
    ///
    /// This ensures that all nodes in the graph have definite values assigned after the
    /// function completes.
    pub fn fill_nodes(&mut self, input: Vec<u64>) {
        if input.len() != self.inputs.len() {
            panic!(
                "number of input arguments supplied does not match number of inputs for function"
            );
        }
        input.iter().enumerate().for_each(|(index, value)| {
            self.inputs[index].value = CustomU64::Val(*value);
            debug!(
                "input node #{} now has value: {}",
                index, self.inputs[index]
            );
        });

        let mut partial_evals = vec![];
        self.inputs.iter().for_each(|node| {
            if let Some(children_nodes) = node.children.clone() {
                children_nodes.iter().for_each(|child_node| {
                    match &child_node.borrow().value {
                        CustomU64::Expr(ExprVal::Add(Some(c_value))) => {
                            if let CustomU64::Val(node_value) = node.value {
                                child_node.borrow_mut().value =
                                    CustomU64::Val(*c_value + node_value);
                            }
                        }
                        CustomU64::Expr(ExprVal::Add(None)) => {
                            if let CustomU64::Val(node_value) = node.value {
                                child_node.borrow_mut().value = CustomU64::Val(node_value);
                                partial_evals.push(child_node.clone());
                            }
                        }
                        CustomU64::Expr(ExprVal::Mul(Some(c_value))) => {
                            if let CustomU64::Val(node_val) = node.value {
                                child_node.borrow_mut().value =
                                    CustomU64::Val(*c_value * node_val);
                            }
                        }
                        CustomU64::Expr(ExprVal::Mul(None)) => {
                            if let CustomU64::Val(node_value) = node.value {
                                child_node.borrow_mut().value = CustomU64::Val(node_value);
                                partial_evals.push(child_node.clone());
                            }
                        }
                        _ => { panic!("The input variable should have already been populated with a value");}
                    }
                })
            }
        });
        partial_evals.iter_mut().for_each(|node| {
            node.borrow_mut().evaluate_children();
        });
    }

    /// Checks if all constraints in the circuit hold true.
    ///
    /// Constraints to be checked include those generated from node operations (addition, multiplication)
    /// and any manually asserted using `assert_equal`.
    ///
    /// # Returns
    ///
    /// Returns `true` if all constraints hold, otherwise `false`.
    pub fn check_constraints(&mut self) -> bool {
        self.constraints
            .iter()
            .all(|constraint| constraint.is_valid());
        info!("all constraints hold true");
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // represent x^2 + x + 5 in an arithmetic circuit
    #[test]
    fn test_sample_polynomial() {
        let mut builder = Builder::new();
        let x = builder.init();
        let x_squared = builder.mul(x.clone(), x.clone());
        let five = builder.constant(5);
        let x_squared_plus_5 = builder.add(x_squared, five);
        let _ = builder.add(x_squared_plus_5, x);
        builder.fill_nodes(vec![5]);
        assert_eq!(builder.check_constraints(), true);
    }
}
