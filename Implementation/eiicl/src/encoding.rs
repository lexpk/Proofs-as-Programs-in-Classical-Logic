use std::collections::HashMap;
use tptp::visitor::Visitor;
use tptp::{common, fof, top, TPTPIterator};

#[derive(Hash, Debug)]
pub enum PlainTerm {
    Variable {
        name: String,
    },
    Function {
        name: String,
        arguments: Vec<PlainTerm>,
    },
}

impl PlainTerm {
    fn to_tptp(&self) -> fof::Term {
        match self {
            PlainTerm::Variable { name } => {
                fof::Term::Variable(common::Variable(common::UpperWord(name)))
            }
            PlainTerm::Function { name, arguments } => fof::Term::Function(Box::new(
                fof::FunctionTerm::Plain(fof::PlainTerm::Function(
                    common::Functor(common::AtomicWord::SingleQuoted(common::SingleQuoted(name))),
                    Box::new(fof::Arguments(
                        arguments.iter().map(|x| x.to_tptp()).collect(),
                    )),
                )),
            )),
        }
    }
}

#[derive(Hash, Debug)]
pub struct AtomizedSubformula {
    name: String,
    arguments: Vec<PlainTerm>,
}

impl AtomizedSubformula {
    fn to_tptp(&self) -> fof::AtomicFormula {
        fof::AtomicFormula::Plain(fof::PlainAtomicFormula(fof::PlainTerm::Function(
            common::Functor(common::AtomicWord::SingleQuoted(common::SingleQuoted(
                &self.name,
            ))),
            Box::new(fof::Arguments(
                self.arguments.iter().map(|x| x.to_tptp()).collect(),
            )),
        )))
    }
}

#[derive(Hash, Debug)]
pub enum NormalizedFormula<'a> {
    LeftAnd {
        left: Vec<AtomizedSubformula>,
        right: AtomizedSubformula,
    },
    RightAnd {
        left: AtomizedSubformula,
        right: Vec<AtomizedSubformula>,
    },
    LeftOr {
        left: Vec<AtomizedSubformula>,
        right: AtomizedSubformula,
    },
    RightOr {
        left: AtomizedSubformula,
        right: Vec<AtomizedSubformula>,
    },
    DoubleImplication {
        left: (AtomizedSubformula, AtomizedSubformula),
        right: AtomizedSubformula,
    },
    LeftForall {
        variables: Vec<String>,
        left: AtomizedSubformula,
        right: AtomizedSubformula,
    },
    RightForall {
        left: AtomizedSubformula,
        variables: Vec<String>,
        right: AtomizedSubformula,
    },
    LeftExists {
        variables: Vec<String>,
        left: AtomizedSubformula,
        right: AtomizedSubformula,
    },
    RightExists {
        left: AtomizedSubformula,
        variables: Vec<String>,
        right: AtomizedSubformula,
    },
    LeftAtomicFormula {
        left: fof::AtomicFormula<'a>,
        right: AtomizedSubformula,
    },
    RightAtomicFormula {
        left: AtomizedSubformula,
        right: fof::AtomicFormula<'a>,
    },
}

impl<'a> NormalizedFormula<'a> {
    pub fn to_tptp(&self) -> fof::Formula {
        fof::Formula(fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(
            match self {
                NormalizedFormula::LeftAnd { left, right } => fof::BinaryNonassoc {
                    left: Box::new(fof::UnitFormula::Unitary(
                        fof::UnitaryFormula::Parenthesised(Box::new(fof::LogicFormula::Binary(
                            fof::BinaryFormula::Assoc(fof::BinaryAssoc::And(fof::AndFormula(
                                left.iter()
                                    .map(|x| {
                                        fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                                            Box::new(x.to_tptp()),
                                        ))
                                    })
                                    .collect(),
                            ))),
                        ))),
                    )),
                    op: common::NonassocConnective::LRImplies,
                    right: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                        Box::new(right.to_tptp()),
                    ))),
                },
                NormalizedFormula::RightAnd { left, right } => fof::BinaryNonassoc {
                    left: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                        Box::new(left.to_tptp()),
                    ))),
                    op: common::NonassocConnective::LRImplies,
                    right: Box::new(fof::UnitFormula::Unitary(
                        fof::UnitaryFormula::Parenthesised(Box::new(fof::LogicFormula::Binary(
                            fof::BinaryFormula::Assoc(fof::BinaryAssoc::And(fof::AndFormula(
                                right
                                    .iter()
                                    .map(|x| {
                                        fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                                            Box::new(x.to_tptp()),
                                        ))
                                    })
                                    .collect(),
                            ))),
                        ))),
                    )),
                },
                NormalizedFormula::LeftOr { left, right } => fof::BinaryNonassoc {
                    left: Box::new(fof::UnitFormula::Unitary(
                        fof::UnitaryFormula::Parenthesised(Box::new(fof::LogicFormula::Binary(
                            fof::BinaryFormula::Assoc(fof::BinaryAssoc::Or(fof::OrFormula(
                                left.iter()
                                    .map(|x| {
                                        fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                                            Box::new(x.to_tptp()),
                                        ))
                                    })
                                    .collect(),
                            ))),
                        ))),
                    )),
                    op: common::NonassocConnective::LRImplies,
                    right: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                        Box::new(right.to_tptp()),
                    ))),
                },
                NormalizedFormula::RightOr { left, right } => fof::BinaryNonassoc {
                    left: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                        Box::new(left.to_tptp()),
                    ))),
                    op: common::NonassocConnective::LRImplies,
                    right: Box::new(fof::UnitFormula::Unitary(
                        fof::UnitaryFormula::Parenthesised(Box::new(fof::LogicFormula::Binary(
                            fof::BinaryFormula::Assoc(fof::BinaryAssoc::Or(fof::OrFormula(
                                right
                                    .iter()
                                    .map(|x| {
                                        fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                                            Box::new(x.to_tptp()),
                                        ))
                                    })
                                    .collect(),
                            ))),
                        ))),
                    )),
                },
                NormalizedFormula::DoubleImplication { left, right } => fof::BinaryNonassoc {
                    left: Box::new(fof::UnitFormula::Unitary(
                        fof::UnitaryFormula::Parenthesised(Box::new(fof::LogicFormula::Binary(
                            fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
                                left: Box::new(fof::UnitFormula::Unitary(
                                    fof::UnitaryFormula::Atomic(Box::new(left.0.to_tptp())),
                                )),
                                op: common::NonassocConnective::LRImplies,
                                right: Box::new(fof::UnitFormula::Unitary(
                                    fof::UnitaryFormula::Atomic(Box::new(left.1.to_tptp())),
                                )),
                            }),
                        ))),
                    )),
                    op: common::NonassocConnective::LRImplies,
                    right: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                        Box::new(right.to_tptp()),
                    ))),
                },
                NormalizedFormula::LeftForall {
                    variables,
                    left,
                    right,
                } => fof::BinaryNonassoc {
                    left: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Quantified(
                        fof::QuantifiedFormula {
                            quantifier: fof::Quantifier::Forall,
                            bound: fof::VariableList(
                                variables
                                    .iter()
                                    .map(|x| common::Variable(common::UpperWord(x)))
                                    .collect(),
                            ),
                            formula: Box::new(fof::UnitFormula::Unitary(
                                fof::UnitaryFormula::Atomic(Box::new(left.to_tptp())),
                            )),
                        },
                    ))),
                    op: common::NonassocConnective::LRImplies,
                    right: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                        Box::new(right.to_tptp()),
                    ))),
                },
                NormalizedFormula::RightForall {
                    left,
                    variables,
                    right,
                } => fof::BinaryNonassoc {
                    left: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                        Box::new(left.to_tptp()),
                    ))),
                    op: common::NonassocConnective::LRImplies,
                    right: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Quantified(
                        fof::QuantifiedFormula {
                            quantifier: fof::Quantifier::Forall,
                            bound: fof::VariableList(
                                variables
                                    .iter()
                                    .map(|x| common::Variable(common::UpperWord(x)))
                                    .collect(),
                            ),
                            formula: Box::new(fof::UnitFormula::Unitary(
                                fof::UnitaryFormula::Atomic(Box::new(right.to_tptp())),
                            )),
                        },
                    ))),
                },
                NormalizedFormula::LeftExists {
                    variables,
                    left,
                    right,
                } => fof::BinaryNonassoc {
                    left: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Quantified(
                        fof::QuantifiedFormula {
                            quantifier: fof::Quantifier::Exists,
                            bound: fof::VariableList(
                                variables
                                    .iter()
                                    .map(|x| common::Variable(common::UpperWord(x)))
                                    .collect(),
                            ),
                            formula: Box::new(fof::UnitFormula::Unitary(
                                fof::UnitaryFormula::Atomic(Box::new(left.to_tptp())),
                            )),
                        },
                    ))),
                    op: common::NonassocConnective::LRImplies,
                    right: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                        Box::new(right.to_tptp()),
                    ))),
                },
                NormalizedFormula::RightExists {
                    left,
                    variables,
                    right,
                } => fof::BinaryNonassoc {
                    left: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                        Box::new(left.to_tptp()),
                    ))),
                    op: common::NonassocConnective::LRImplies,
                    right: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Quantified(
                        fof::QuantifiedFormula {
                            quantifier: fof::Quantifier::Exists,
                            bound: fof::VariableList(
                                variables
                                    .iter()
                                    .map(|x| common::Variable(common::UpperWord(x)))
                                    .collect(),
                            ),
                            formula: Box::new(fof::UnitFormula::Unitary(
                                fof::UnitaryFormula::Atomic(Box::new(right.to_tptp())),
                            )),
                        },
                    ))),
                },
                NormalizedFormula::LeftAtomicFormula { left, right } => fof::BinaryNonassoc {
                    left: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                        Box::new(left.clone()),
                    ))),
                    op: common::NonassocConnective::LRImplies,
                    right: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                        Box::new(right.to_tptp()),
                    ))),
                },
                NormalizedFormula::RightAtomicFormula { left, right } => fof::BinaryNonassoc {
                    left: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                        Box::new(left.to_tptp()),
                    ))),
                    op: common::NonassocConnective::LRImplies,
                    right: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                        Box::new(right.clone()),
                    ))),
                },
            },
        )))
    }
}

#[derive(Debug)]
pub struct Encoding<'a> {
    pub axioms: Vec<NormalizedFormula<'a>>,
    pub goal: Option<AtomizedSubformula>,
    variables: HashMap<String, Vec<String>>,
    polarity: bool,
}

impl<'a> Encoding<'a> {
    pub fn new() -> Encoding<'a> {
        Encoding {
            axioms: vec![],
            goal: None,
            variables: HashMap::new(),
            polarity: true,
        }
    }
    pub fn from_tptp(bytes: &[u8]) -> Encoding {
        let mut encoding = Encoding::new();
        let mut parser = TPTPIterator::<()>::new(bytes);
        for result in &mut parser {
            let input = result.expect("syntax error");
            encoding.visit_tptp_input(&input);
        }
        return encoding;
    }

    pub fn to_tptp(&self) -> Vec<top::FofAnnotated> {
        let mut result = vec![];
        for axiom in self.axioms.iter() {
            result.push(top::FofAnnotated(top::Annotated {
                name: common::Name::Integer(common::Integer("1")),
                role: top::FormulaRole(common::LowerWord("axiom")),
                formula: Box::new(axiom.to_tptp()),
                annotations: top::Annotations(None),
            }))
        }
        match self.goal {
            Some(ref goal) => result.push(top::FofAnnotated(top::Annotated {
                name: common::Name::Integer(common::Integer("goal")),
                role: top::FormulaRole(common::LowerWord("conjecture")),
                formula: Box::new(fof::Formula(fof::LogicFormula::Unitary(fof::UnitaryFormula::Atomic(Box::new(goal.to_tptp()))))),
                annotations: top::Annotations(None),
            })),
            None => {},
        }
        return result;
    }
}

impl<'a> Visitor<'a> for Encoding<'a> {
    fn visit_atomic_word(&mut self, atomic_word: &tptp::common::AtomicWord<'a>) {
        match atomic_word {
            tptp::common::AtomicWord::Lower(lower_word) => self.visit_lower_word(lower_word),
            tptp::common::AtomicWord::SingleQuoted(single_quoted) => {
                self.visit_single_quoted(single_quoted)
            }
        }
    }

    fn visit_variable(&mut self, variable: &tptp::common::Variable<'a>) {
        self.visit_upper_word(&variable.0);
        self.variables
            .insert(variable.to_string(), vec![variable.to_string()]);
    }

    fn visit_fof_arguments(&mut self, fof_arguments: &fof::Arguments<'a>) {
        let mut variables = vec![];
        for fof_term in &*fof_arguments.0 {
            self.visit_fof_term(fof_term);
            variables.extend(self.variables.get(&fof_term.to_string()).unwrap().clone());
        }
        variables.sort_unstable();
        variables.dedup();
        self.variables.insert(fof_arguments.to_string(), variables);
    }

    fn visit_fof_system_term(&mut self, fof_system_term: &fof::SystemTerm<'a>) {
        match fof_system_term {
            fof::SystemTerm::Constant(constant) => self.visit_system_constant(constant),
            fof::SystemTerm::Function(functor, fof_arguments) => {
                self.visit_system_functor(functor);
                self.visit_fof_arguments(fof_arguments);
                self.variables.insert(
                    fof_system_term.to_string(),
                    self.variables
                        .get(&fof_arguments.to_string())
                        .unwrap()
                        .clone(),
                );
            }
        }
    }

    fn visit_fof_plain_term(&mut self, fof_plain_term: &fof::PlainTerm<'a>) {
        match fof_plain_term {
            fof::PlainTerm::Constant(constant) => self.visit_constant(constant),
            fof::PlainTerm::Function(functor, fof_arguments) => {
                self.visit_functor(functor);
                self.visit_fof_arguments(fof_arguments);
                self.variables.insert(
                    fof_plain_term.to_string(),
                    self.variables
                        .get(&fof_arguments.to_string())
                        .unwrap()
                        .clone(),
                );
            }
        }
    }

    fn visit_fof_defined_plain_term(&mut self, fof_defined_plain_term: &fof::DefinedPlainTerm<'a>) {
        match fof_defined_plain_term {
            fof::DefinedPlainTerm::Constant(constant) => self.visit_defined_constant(constant),
            fof::DefinedPlainTerm::Function(functor, fof_arguments) => {
                self.visit_defined_functor(functor);
                self.visit_fof_arguments(fof_arguments);
                self.variables.insert(
                    fof_defined_plain_term.to_string(),
                    self.variables
                        .get(&fof_arguments.to_string())
                        .unwrap()
                        .clone(),
                );
            }
        }
    }

    fn visit_fof_term(&mut self, fof_term: &fof::Term<'a>) {
        match fof_term {
            fof::Term::Function(fof_function_term) => {
                self.visit_fof_function_term(fof_function_term)
            }
            fof::Term::Variable(variable) => {
                self.visit_variable(variable);
                self.variables
                    .insert(fof_term.to_string(), vec![variable.to_string()]);
            }
        }
    }

    fn visit_fof_defined_infix_formula(
        &mut self,
        fof_defined_infix_formula: &fof::DefinedInfixFormula<'a>,
    ) {
        let mut variables = vec![];
        self.visit_fof_term(&fof_defined_infix_formula.left);
        variables.extend(
            self.variables
                .get(&fof_defined_infix_formula.left.to_string())
                .unwrap()
                .clone(),
        );
        self.visit_defined_infix_pred(fof_defined_infix_formula.op);
        self.visit_fof_term(&fof_defined_infix_formula.right);
        variables.extend(
            self.variables
                .get(&fof_defined_infix_formula.right.to_string())
                .unwrap()
                .clone(),
        );
        variables.sort_unstable();
        variables.dedup();
        self.variables
            .insert(fof_defined_infix_formula.to_string(), variables.clone());
    }

    fn visit_fof_atomic_formula(&mut self, fof_atomic_formula: &fof::AtomicFormula<'a>) {
        match fof_atomic_formula {
            fof::AtomicFormula::Plain(fof_plain_atomic_formula) => {
                self.visit_fof_plain_atomic_formula(fof_plain_atomic_formula)
            }
            fof::AtomicFormula::Defined(fof_defined_atomic_formula) => {
                self.visit_fof_defined_atomic_formula(fof_defined_atomic_formula)
            }
            fof::AtomicFormula::System(fof_system_atomic_formula) => {
                self.visit_fof_system_atomic_formula(fof_system_atomic_formula)
            }
        }
        let atomized = AtomizedSubformula {
            name: fof_atomic_formula.to_string(),
            arguments: self
                .variables
                .get(&fof_atomic_formula.to_string())
                .unwrap()
                .clone()
                .into_iter()
                .chain(vec![String::from("U")].into_iter())
                .map(|x| PlainTerm::Variable { name: x })
                .collect(),
        };
        match self.polarity {
            true => self.axioms.push(NormalizedFormula::LeftAtomicFormula {
                left: extend_atom(fof_atomic_formula),
                right: atomized,
            }),
            false => self.axioms.push(NormalizedFormula::RightAtomicFormula {
                left: atomized,
                right: extend_atom(fof_atomic_formula),
            }),
        }
    }

    fn visit_fof_infix_unary(&mut self, fof_infix_unary: &fof::InfixUnary<'a>) {
        self.visit_fof_unary_formula(&fof::UnaryFormula::Unary(
            common::UnaryConnective,
            Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                Box::new(fof::AtomicFormula::Defined(
                    fof::DefinedAtomicFormula::Infix(fof::DefinedInfixFormula {
                        left: fof_infix_unary.left.clone(),
                        op: common::DefinedInfixPred(common::InfixEquality),
                        right: fof_infix_unary.right.clone(),
                    }),
                )),
            ))),
        ));
    }

    fn visit_fof_binary_nonassoc(&mut self, fof_binary_nonassoc: &fof::BinaryNonassoc<'a>) {
        match self.polarity {
            true => match fof_binary_nonassoc.op {
                common::NonassocConnective::LRImplies => {
                    let mut variables = vec![];
                    self.polarity = false;
                    self.visit_fof_unit_formula(&fof_binary_nonassoc.left);
                    variables.extend(
                        self.variables
                            .get(&fof_binary_nonassoc.left.to_string())
                            .unwrap()
                            .clone(),
                    );
                    self.polarity = true;
                    self.visit_fof_unit_formula(&fof_binary_nonassoc.right);
                    variables.extend(
                        self.variables
                            .get(&fof_binary_nonassoc.right.to_string())
                            .unwrap()
                            .clone(),
                    );
                    variables.sort_unstable();
                    variables.dedup();
                    self.variables
                        .insert(fof_binary_nonassoc.to_string(), variables);
                    self.axioms.push(NormalizedFormula::DoubleImplication {
                        left: (
                            AtomizedSubformula {
                                name: fof_binary_nonassoc.left.to_string(),
                                arguments: self
                                    .variables
                                    .get(&fof_binary_nonassoc.left.to_string())
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .map(|x| PlainTerm::Variable { name: x })
                                    .chain(
                                        vec![PlainTerm::Function {
                                            name: String::from("f_")
                                                + &fof_binary_nonassoc.to_string()
                                                + "",
                                            arguments: vec![PlainTerm::Variable {
                                                name: String::from("U"),
                                            }],
                                        }]
                                        .into_iter(),
                                    )
                                    .collect(),
                            },
                            AtomizedSubformula {
                                name: fof_binary_nonassoc.right.to_string(),
                                arguments: self
                                    .variables
                                    .get(&fof_binary_nonassoc.right.to_string())
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .map(|x| PlainTerm::Variable { name: x })
                                    .chain(
                                        vec![PlainTerm::Function {
                                            name: String::from("f_")
                                                + &fof_binary_nonassoc.to_string(),
                                            arguments: vec![PlainTerm::Variable {
                                                name: String::from("U"),
                                            }],
                                        }]
                                        .into_iter(),
                                    )
                                    .collect(),
                            },
                        ),
                        right: AtomizedSubformula {
                            name: fof_binary_nonassoc.to_string(),
                            arguments: self
                                .variables
                                .get(&fof_binary_nonassoc.to_string())
                                .unwrap()
                                .clone()
                                .into_iter()
                                .chain(vec![String::from("U")].into_iter())
                                .map(|x| PlainTerm::Variable { name: x })
                                .collect(),
                        },
                    })
                }
                common::NonassocConnective::RLImplies => {
                    self.visit_fof_binary_nonassoc(&fof::BinaryNonassoc {
                        left: fof_binary_nonassoc.right.clone(),
                        op: common::NonassocConnective::LRImplies,
                        right: fof_binary_nonassoc.left.clone(),
                    });
                }
                common::NonassocConnective::Equivalent => {
                    self.visit_fof_binary_nonassoc(&fof::BinaryNonassoc {
                        left: fof_binary_nonassoc.left.clone(),
                        op: common::NonassocConnective::LRImplies,
                        right: fof_binary_nonassoc.right.clone(),
                    });
                    self.visit_fof_binary_nonassoc(&fof::BinaryNonassoc {
                        left: fof_binary_nonassoc.right.clone(),
                        op: common::NonassocConnective::LRImplies,
                        right: fof_binary_nonassoc.left.clone(),
                    });
                }
                common::NonassocConnective::NotEquivalent => {
                    self.visit_fof_unary_formula(&fof::UnaryFormula::Unary(
                        common::UnaryConnective,
                        Box::new(fof::UnitFormula::Unitary(
                            fof::UnitaryFormula::Parenthesised(Box::new(
                                fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(
                                    fof::BinaryNonassoc {
                                        left: fof_binary_nonassoc.left.clone(),
                                        op: common::NonassocConnective::Equivalent,
                                        right: fof_binary_nonassoc.right.clone(),
                                    },
                                )),
                            )),
                        )),
                    ))
                }
                common::NonassocConnective::NotOr => {
                    self.visit_fof_unary_formula(&fof::UnaryFormula::Unary(
                        common::UnaryConnective,
                        Box::new(fof::UnitFormula::Unitary(
                            fof::UnitaryFormula::Parenthesised(Box::new(
                                fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(
                                    fof::BinaryNonassoc {
                                        left: fof_binary_nonassoc.left.clone(),
                                        op: common::NonassocConnective::Equivalent,
                                        right: fof_binary_nonassoc.right.clone(),
                                    },
                                )),
                            )),
                        )),
                    ))
                }
                common::NonassocConnective::NotAnd => {
                    self.visit_fof_unary_formula(&fof::UnaryFormula::Unary(
                        common::UnaryConnective,
                        Box::new(fof::UnitFormula::Unitary(
                            fof::UnitaryFormula::Parenthesised(Box::new(
                                fof::LogicFormula::Binary(fof::BinaryFormula::Assoc(
                                    fof::BinaryAssoc::And(fof::AndFormula(vec![
                                        *fof_binary_nonassoc.left.clone(),
                                        *fof_binary_nonassoc.right.clone(),
                                    ])),
                                )),
                            )),
                        )),
                    ))
                }
            },
            false => match fof_binary_nonassoc.op {
                common::NonassocConnective::LRImplies => {
                    let mut variables = vec![];
                    self.polarity = true;
                    self.visit_fof_unit_formula(&fof_binary_nonassoc.left);
                    variables.extend(
                        self.variables
                            .get(&fof_binary_nonassoc.left.to_string())
                            .unwrap()
                            .clone(),
                    );
                    self.polarity = false;
                    self.visit_fof_unit_formula(&fof_binary_nonassoc.right);
                    variables.extend(
                        self.variables
                            .get(&fof_binary_nonassoc.right.to_string())
                            .unwrap()
                            .clone(),
                    );
                    variables.sort_unstable();
                    variables.dedup();
                    self.variables
                        .insert(fof_binary_nonassoc.to_string(), variables);
                    self.axioms.push(NormalizedFormula::LeftAnd {
                        left: vec![
                            AtomizedSubformula {
                                name: fof_binary_nonassoc.left.to_string(),
                                arguments: self
                                    .variables
                                    .get(&fof_binary_nonassoc.left.to_string())
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .chain(vec![String::from("U")].into_iter())
                                    .map(|x| PlainTerm::Variable { name: x })
                                    .collect(),
                            },
                            AtomizedSubformula {
                                name: fof_binary_nonassoc.right.to_string(),
                                arguments: self
                                    .variables
                                    .get(&fof_binary_nonassoc.right.to_string())
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .chain(vec![String::from("U")].into_iter())
                                    .map(|x| PlainTerm::Variable { name: x })
                                    .collect(),
                            },
                        ],
                        right: AtomizedSubformula {
                            name: fof_binary_nonassoc.to_string(),
                            arguments: self
                                .variables
                                .get(&fof_binary_nonassoc.to_string())
                                .unwrap()
                                .clone()
                                .into_iter()
                                .chain(vec![String::from("U")].into_iter())
                                .map(|x| PlainTerm::Variable { name: x })
                                .collect(),
                        },
                    })
                }
                common::NonassocConnective::RLImplies => {
                    self.visit_fof_binary_nonassoc(&fof::BinaryNonassoc {
                        left: fof_binary_nonassoc.right.clone(),
                        op: common::NonassocConnective::LRImplies,
                        right: fof_binary_nonassoc.left.clone(),
                    });
                }
                common::NonassocConnective::Equivalent => {
                    self.visit_fof_binary_nonassoc(&fof::BinaryNonassoc {
                        left: fof_binary_nonassoc.left.clone(),
                        op: common::NonassocConnective::LRImplies,
                        right: fof_binary_nonassoc.right.clone(),
                    });
                    self.visit_fof_binary_nonassoc(&fof::BinaryNonassoc {
                        left: fof_binary_nonassoc.right.clone(),
                        op: common::NonassocConnective::LRImplies,
                        right: fof_binary_nonassoc.left.clone(),
                    });
                }
                common::NonassocConnective::NotEquivalent => {
                    self.visit_fof_unary_formula(&fof::UnaryFormula::Unary(
                        common::UnaryConnective,
                        Box::new(fof::UnitFormula::Unitary(
                            fof::UnitaryFormula::Parenthesised(Box::new(
                                fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(
                                    fof::BinaryNonassoc {
                                        left: fof_binary_nonassoc.left.clone(),
                                        op: common::NonassocConnective::Equivalent,
                                        right: fof_binary_nonassoc.right.clone(),
                                    },
                                )),
                            )),
                        )),
                    ))
                }
                common::NonassocConnective::NotOr => {
                    self.visit_fof_unary_formula(&fof::UnaryFormula::Unary(
                        common::UnaryConnective,
                        Box::new(fof::UnitFormula::Unitary(
                            fof::UnitaryFormula::Parenthesised(Box::new(
                                fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(
                                    fof::BinaryNonassoc {
                                        left: fof_binary_nonassoc.left.clone(),
                                        op: common::NonassocConnective::Equivalent,
                                        right: fof_binary_nonassoc.right.clone(),
                                    },
                                )),
                            )),
                        )),
                    ))
                }
                common::NonassocConnective::NotAnd => {
                    self.visit_fof_unary_formula(&fof::UnaryFormula::Unary(
                        common::UnaryConnective,
                        Box::new(fof::UnitFormula::Unitary(
                            fof::UnitaryFormula::Parenthesised(Box::new(
                                fof::LogicFormula::Binary(fof::BinaryFormula::Assoc(
                                    fof::BinaryAssoc::And(fof::AndFormula(vec![
                                        *fof_binary_nonassoc.left.clone(),
                                        *fof_binary_nonassoc.right.clone(),
                                    ])),
                                )),
                            )),
                        )),
                    ))
                }
            },
        }
    }

    fn visit_fof_or_formula(&mut self, fof_or_formula: &fof::OrFormula<'a>) {
        let mut variables = vec![];
        for fof_unit_formula in &*fof_or_formula.0 {
            self.visit_fof_unit_formula(fof_unit_formula);
            variables.extend(
                self.variables
                    .get(&fof_unit_formula.to_string())
                    .unwrap()
                    .clone(),
            );
        }
        variables.sort_unstable();
        variables.dedup();
        self.variables
            .insert(fof_or_formula.to_string(), variables.clone());
        match self.polarity {
            true => self.axioms.push(NormalizedFormula::LeftOr {
                left: fof_or_formula
                    .0
                    .iter()
                    .map(|x| AtomizedSubformula {
                        name: x.to_string(),
                        arguments: self
                            .variables
                            .get(&x.to_string())
                            .unwrap()
                            .clone()
                            .into_iter()
                            .chain(vec![String::from("U")].into_iter())
                            .map(|x| PlainTerm::Variable { name: x })
                            .collect(),
                    })
                    .collect(),
                right: AtomizedSubformula {
                    name: fof_or_formula.to_string(),
                    arguments: self
                        .variables
                        .get(&fof_or_formula.to_string())
                        .unwrap()
                        .clone()
                        .into_iter()
                        .chain(vec![String::from("U")].into_iter())
                        .map(|x| PlainTerm::Variable { name: x })
                        .collect(),
                },
            }),
            false => self.axioms.push(NormalizedFormula::RightOr {
                left: AtomizedSubformula {
                    name: fof_or_formula.to_string(),
                    arguments: self
                        .variables
                        .get(&fof_or_formula.to_string())
                        .unwrap()
                        .clone()
                        .into_iter()
                        .chain(vec![String::from("U")].into_iter())
                        .map(|x| PlainTerm::Variable { name: x })
                        .collect(),
                },
                right: fof_or_formula
                    .0
                    .iter()
                    .map(|x| AtomizedSubformula {
                        name: x.to_string(),
                        arguments: self
                            .variables
                            .get(&x.to_string())
                            .unwrap()
                            .clone()
                            .into_iter()
                            .chain(vec![String::from("U")].into_iter())
                            .map(|x| PlainTerm::Variable { name: x })
                            .collect(),
                    })
                    .collect(),
            }),
        }
    }

    fn visit_fof_and_formula(&mut self, fof_and_formula: &fof::AndFormula<'a>) {
        let mut variables = vec![];
        for fof_unit_formula in &*fof_and_formula.0 {
            self.visit_fof_unit_formula(fof_unit_formula);
            variables.extend(
                self.variables
                    .get(&fof_unit_formula.to_string())
                    .unwrap()
                    .clone(),
            );
        }
        variables.sort_unstable();
        variables.dedup();
        self.variables
            .insert(fof_and_formula.to_string(), variables.clone());
        match self.polarity {
            true => self.axioms.push(NormalizedFormula::LeftAnd {
                left: fof_and_formula
                    .0
                    .iter()
                    .map(|x| AtomizedSubformula {
                        name: x.to_string(),
                        arguments: self
                            .variables
                            .get(&x.to_string())
                            .unwrap()
                            .clone()
                            .into_iter()
                            .chain(vec![String::from("U")].into_iter())
                            .map(|x| PlainTerm::Variable { name: x })
                            .collect(),
                    })
                    .collect(),
                right: AtomizedSubformula {
                    name: fof_and_formula.to_string(),
                    arguments: self
                        .variables
                        .get(&fof_and_formula.to_string())
                        .unwrap()
                        .clone()
                        .into_iter()
                        .chain(vec![String::from("U")].into_iter())
                        .map(|x| PlainTerm::Variable { name: x })
                        .collect(),
                },
            }),
            false => self.axioms.push(NormalizedFormula::RightAnd {
                left: AtomizedSubformula {
                    name: fof_and_formula.to_string(),
                    arguments: self
                        .variables
                        .get(&fof_and_formula.to_string())
                        .unwrap()
                        .clone()
                        .into_iter()
                        .chain(vec![String::from("U")].into_iter())
                        .map(|x| PlainTerm::Variable { name: x })
                        .collect(),
                },
                right: fof_and_formula
                    .0
                    .iter()
                    .map(|x| AtomizedSubformula {
                        name: x.to_string(),
                        arguments: self
                            .variables
                            .get(&x.to_string())
                            .unwrap()
                            .clone()
                            .into_iter()
                            .chain(vec![String::from("U")].into_iter())
                            .map(|x| PlainTerm::Variable { name: x })
                            .collect(),
                    })
                    .collect(),
            }),
        }
    }

    fn visit_fof_unary_formula(&mut self, fof_unary_formula: &fof::UnaryFormula<'a>) {
        match fof_unary_formula {
            fof::UnaryFormula::Unary(unary_connective, fof_unit_formula) => {
                self.polarity = !self.polarity;
                self.visit_unary_connective(*unary_connective);
                self.visit_fof_unit_formula(fof_unit_formula);
                self.variables.insert(
                    fof_unary_formula.to_string(),
                    self.variables
                        .get(&fof_unit_formula.to_string())
                        .unwrap()
                        .clone(),
                );
            }
            fof::UnaryFormula::InfixUnary(ref fof_infix_unary) => {
                self.visit_fof_infix_unary(fof_infix_unary)
            }
        }
    }

    fn visit_fof_quantified_formula(
        &mut self,
        fof_quantified_formula: &fof::QuantifiedFormula<'a>,
    ) {
        let mut variables: Vec<String> = vec![];
        self.visit_fof_quantifier(fof_quantified_formula.quantifier);
        self.visit_fof_variable_list(&fof_quantified_formula.bound);
        let namelist: Vec<String> = fof_quantified_formula
            .bound
            .0
            .iter()
            .map(|x| x.to_string())
            .collect();
        self.visit_fof_unit_formula(&fof_quantified_formula.formula);
        for v in self
            .variables
            .get(&fof_quantified_formula.formula.to_string())
            .unwrap()
            .iter()
        {
            if !namelist.contains(v) {
                variables.push(v.clone());
            }
        }
        variables.sort_unstable();
        variables.dedup();
        self.variables
            .insert(fof_quantified_formula.to_string(), variables.clone());
        match self.polarity {
            true => match fof_quantified_formula.quantifier {
                fof::Quantifier::Forall => self.axioms.push(NormalizedFormula::LeftForall {
                    variables: fof_quantified_formula
                        .bound
                        .0
                        .iter()
                        .map(|x| x.to_string())
                        .collect(),
                    left: AtomizedSubformula {
                        name: fof_quantified_formula.formula.to_string(),
                        arguments: self
                            .variables
                            .get(&fof_quantified_formula.formula.to_string())
                            .unwrap()
                            .clone()
                            .into_iter()
                            .chain(vec![String::from("_")].into_iter())
                            .into_iter()
                            .map(|x| PlainTerm::Variable { name: x })
                            .chain(
                                vec![PlainTerm::Function {
                                    name: String::from("f_")
                                        + &fof_quantified_formula.to_string()
                                        + "",
                                    arguments: vec![PlainTerm::Variable {
                                        name: String::from("U"),
                                    }],
                                }]
                                .into_iter(),
                            )
                            .collect(),
                    },
                    right: AtomizedSubformula {
                        name: fof_quantified_formula.to_string(),
                        arguments: self
                            .variables
                            .get(&fof_quantified_formula.to_string())
                            .unwrap()
                            .clone()
                            .into_iter()
                            .chain(vec![String::from("U")].into_iter())
                            .map(|x| PlainTerm::Variable { name: x })
                            .collect(),
                    },
                }),
                fof::Quantifier::Exists => self.axioms.push(NormalizedFormula::LeftExists {
                    variables: fof_quantified_formula
                        .bound
                        .0
                        .iter()
                        .map(|x| x.to_string())
                        .collect(),
                    left: AtomizedSubformula {
                        name: fof_quantified_formula.formula.to_string(),
                        arguments: self
                            .variables
                            .get(&fof_quantified_formula.formula.to_string())
                            .unwrap()
                            .clone()
                            .into_iter()
                            .chain(vec![String::from("U")].into_iter())
                            .map(|x| PlainTerm::Variable { name: x })
                            .collect(),
                    },
                    right: AtomizedSubformula {
                        name: fof_quantified_formula.to_string(),
                        arguments: self
                            .variables
                            .get(&fof_quantified_formula.to_string())
                            .unwrap()
                            .clone()
                            .into_iter()
                            .chain(vec![String::from("U")].into_iter())
                            .map(|x| PlainTerm::Variable { name: x })
                            .collect(),
                    },
                }),
            },
            false => match fof_quantified_formula.quantifier {
                fof::Quantifier::Forall => self.axioms.push(NormalizedFormula::RightForall {
                    left: AtomizedSubformula {
                        name: fof_quantified_formula.to_string(),
                        arguments: self
                            .variables
                            .get(&fof_quantified_formula.to_string())
                            .unwrap()
                            .clone()
                            .into_iter()
                            .chain(vec![String::from("U")].into_iter())
                            .map(|x| PlainTerm::Variable { name: x })
                            .collect(),
                    },
                    variables: fof_quantified_formula
                        .bound
                        .0
                        .iter()
                        .map(|x| x.to_string())
                        .collect(),
                    right: AtomizedSubformula {
                        name: fof_quantified_formula.formula.to_string(),
                        arguments: self
                            .variables
                            .get(&fof_quantified_formula.formula.to_string())
                            .unwrap()
                            .clone()
                            .into_iter()
                            .chain(vec![String::from("U")].into_iter())
                            .map(|x| PlainTerm::Variable { name: x })
                            .collect(),
                    },
                }),
                fof::Quantifier::Exists => self.axioms.push(NormalizedFormula::RightExists {
                    left: AtomizedSubformula {
                        name: fof_quantified_formula.to_string(),
                        arguments: self
                            .variables
                            .get(&fof_quantified_formula.to_string())
                            .unwrap()
                            .clone()
                            .into_iter()
                            .chain(vec![String::from("U")].into_iter())
                            .map(|x| PlainTerm::Variable { name: x })
                            .collect(),
                    },
                    variables: fof_quantified_formula
                        .bound
                        .0
                        .iter()
                        .map(|x| x.to_string())
                        .collect(),
                    right: AtomizedSubformula {
                        name: fof_quantified_formula.formula.to_string(),
                        arguments: self
                            .variables
                            .get(&fof_quantified_formula.formula.to_string())
                            .unwrap()
                            .clone()
                            .into_iter()
                            .chain(vec![String::from("U")].into_iter())
                            .map(|x| PlainTerm::Variable { name: x })
                            .collect(),
                    },
                }),
            },
        }
    }

    fn visit_fof_unitary_formula(&mut self, fof_unitary_formula: &fof::UnitaryFormula<'a>) {
        match fof_unitary_formula {
            fof::UnitaryFormula::Quantified(fof_quantified_formula) => {
                self.visit_fof_quantified_formula(fof_quantified_formula);
            }
            fof::UnitaryFormula::Atomic(fof_atomic_formula) => {
                self.visit_fof_atomic_formula(fof_atomic_formula);
            }
            fof::UnitaryFormula::Parenthesised(boxed_fof_logic_formula) => {
                self.visit_fof_logic_formula(boxed_fof_logic_formula);
                self.variables.insert(
                    fof_unitary_formula.to_string(),
                    self.variables
                        .get(&boxed_fof_logic_formula.to_string())
                        .unwrap()
                        .clone(),
                );
            }
        }
    }

    fn visit_fof_annotated(&mut self, fof_annotated: &tptp::top::FofAnnotated<'a>) {
        match fof_annotated.0.role.0 .0 {
            "axiom" => {
                self.polarity = false;
                self.visit_fof_formula(fof_annotated.0.formula.as_ref());
            }
            "conjecture" => {
                self.polarity = true;
                self.visit_fof_formula(fof_annotated.0.formula.as_ref());
                self.goal = Some(AtomizedSubformula {
                    name: fof_annotated.0.formula.to_string(),
                    arguments: self
                        .variables
                        .get(&fof_annotated.0.formula.to_string())
                        .unwrap()
                        .clone()
                        .into_iter()
                        .map(|x| PlainTerm::Variable { name: x })
                        .collect(),
                });
            }
            _ => panic!(
                "\"{}\" is neither axiom nor conjecture",
                fof_annotated.to_string()
            ),
        }
    }
}

fn extend_atom<'a>(fof_atomic_formula: &fof::AtomicFormula<'a>) -> fof::AtomicFormula<'a> {
    match fof_atomic_formula {
        fof::AtomicFormula::Plain(plain_atomic_formula) => {
            fof::AtomicFormula::Plain(fof::PlainAtomicFormula(match plain_atomic_formula.0 {
                fof::PlainTerm::Constant(ref constant) => {
                    fof::PlainTerm::Constant(constant.clone())
                }
                fof::PlainTerm::Function(ref functor, ref arguments_box) => {
                    fof::PlainTerm::Function(
                        functor.clone(),
                        Box::new(fof::Arguments(
                            (*arguments_box.clone())
                                .0
                                .into_iter()
                                .chain(
                                    vec![fof::Term::Variable(common::Variable(common::UpperWord(
                                        "U",
                                    )))]
                                    .into_iter(),
                                )
                                .collect(),
                        )),
                    )
                }
            }))
        }
        fof::AtomicFormula::Defined(defined_atomic_formula) => match defined_atomic_formula {
            fof::DefinedAtomicFormula::Plain(ref defined_plain_formula) => {
                fof::AtomicFormula::Defined(fof::DefinedAtomicFormula::Plain(
                    fof::DefinedPlainFormula(match defined_plain_formula.0 {
                        fof::DefinedPlainTerm::Constant(ref defined_constant) => {
                            fof::DefinedPlainTerm::Constant(defined_constant.clone())
                        }
                        fof::DefinedPlainTerm::Function(ref functor, ref arguments_box) => {
                            fof::DefinedPlainTerm::Function(
                                functor.clone(),
                                Box::new(fof::Arguments(
                                    (*arguments_box.clone())
                                        .0
                                        .into_iter()
                                        .chain(
                                            vec![fof::Term::Variable(common::Variable(
                                                common::UpperWord("U"),
                                            ))]
                                            .into_iter(),
                                        )
                                        .collect(),
                                )),
                            )
                        }
                    }),
                ))
            }
            fof::DefinedAtomicFormula::Infix(ref defined_infix_formula) => {
                fof::AtomicFormula::Plain(fof::PlainAtomicFormula(fof::PlainTerm::Function(
                    common::Functor(common::AtomicWord::SingleQuoted(common::SingleQuoted("eq"))),
                    Box::new(fof::Arguments(vec![
                        *defined_infix_formula.left.clone(),
                        *defined_infix_formula.right.clone(),
                        fof::Term::Variable(common::Variable(common::UpperWord("U"))),
                    ])),
                )))
            }
        },
        fof::AtomicFormula::System(system_atomic_formula) => {
            fof::AtomicFormula::System(fof::SystemAtomicFormula(match system_atomic_formula.0 {
                fof::SystemTerm::Constant(ref constant) => {
                    fof::SystemTerm::Constant(constant.clone())
                }
                fof::SystemTerm::Function(ref functor, ref arguments_box) => {
                    fof::SystemTerm::Function(
                        functor.clone(),
                        Box::new(fof::Arguments(
                            (*arguments_box.clone())
                                .0
                                .into_iter()
                                .chain(
                                    vec![fof::Term::Variable(common::Variable(common::UpperWord(
                                        "U",
                                    )))]
                                    .into_iter(),
                                )
                                .collect(),
                        )),
                    )
                }
            }))
        }
    }
}
