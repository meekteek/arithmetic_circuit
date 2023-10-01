use crate::Node;

/// Represents a u64 or an expression.
///
/// This enum can either hold u64 or represent
/// a more complex arithmetic expression fit for our circuit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomU64 {
    Val(u64),
    Expr(ExprVal),
}
impl Default for CustomU64 {
    fn default() -> Self {
        CustomU64::Val(0)
    }
}

/// Represents types of arithmetic expressions or operations.
///
/// This enum captures addition and multiplication arithmetic operations
/// along with a possible 'Input' as a placeholder to be filled in later.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprVal {
    Add(Option<u64>),
    Mul(Option<u64>),
    Input,
}

/// Represents various constraints between nodes in the circuit.
///
/// Constraints are used to ensure the validity of the operations
/// performed on the nodes. These can also be thought of as gates.
/// These constraints are created when nodes undergo arithmetic operations or when
/// equality between nodes is requested in builder::assert_equal(..)
pub(crate) enum Constraints {
    Add(Node, Node, Node),
    Mul(Node, Node, Node),
    Eq(Node, Node),
}
impl Constraints {
    /// Checks if the constraint holds true or not.
    ///
    /// # Returns
    ///
    /// * `true` if the constraint is valid.
    /// * `false` otherwise.
    pub fn is_valid(&self) -> bool {
        match self {
            Constraints::Add(a, b, c) => {
                if let (CustomU64::Val(a), CustomU64::Val(b), CustomU64::Val(c)) =
                    (&a.value, &b.value, &c.value)
                {
                    *a + *b == *c
                } else {
                    false
                }
            }
            Constraints::Mul(a, b, c) => {
                if let (CustomU64::Val(a), CustomU64::Val(b), CustomU64::Val(c)) =
                    (&a.value, &b.value, &c.value)
                {
                    *a * *b == *c
                } else {
                    false
                }
            }
            Constraints::Eq(a, b) => {
                if let (CustomU64::Val(a), CustomU64::Val(b)) = (&a.value, &b.value) {
                    *a == *b
                } else {
                    false
                }
            }
        }
    }
}
