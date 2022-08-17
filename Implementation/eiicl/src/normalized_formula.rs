use tptp::{common, fof};

#[derive(Hash, Debug, PartialEq, Clone)]
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

#[derive(Hash, Debug, Clone, PartialEq)]
pub struct AtomizedSubformula {
    pub name: String,
    pub arguments: Vec<PlainTerm>,
}

impl AtomizedSubformula {
    pub fn to_tptp(&self) -> fof::AtomicFormula {
        if self.name == "false" {
            fof::AtomicFormula::Defined(fof::DefinedAtomicFormula::Plain(fof::DefinedPlainFormula(fof::DefinedPlainTerm::Constant(
                common::DefinedConstant(common::DefinedFunctor(common::AtomicDefinedWord(common::DollarWord(common::LowerWord("false")))))
            ))))
        } else if self.name == "true" {
            fof::AtomicFormula::Defined(fof::DefinedAtomicFormula::Plain(fof::DefinedPlainFormula(fof::DefinedPlainTerm::Constant(
                common::DefinedConstant(common::DefinedFunctor(common::AtomicDefinedWord(common::DollarWord(common::LowerWord("true")))))
            ))))
        } else {
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
}


#[derive(Hash, Debug, Clone, PartialEq)]
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
    PlainAxiom {
        formula : AtomizedSubformula
    }
}

impl<'a> NormalizedFormula<'a> {
    pub fn to_tptp(&self) -> fof::Formula {
        let exists = match self {
            NormalizedFormula::LeftAnd { left: _, right }
            | NormalizedFormula::LeftOr { left: _, right }
            | NormalizedFormula::DoubleImplication { left: _, right }
            | NormalizedFormula::LeftForall {
                variables: _,
                left: _,
                right,
            }
            | NormalizedFormula::LeftExists {
                variables: _,
                left: _,
                right,
            }
            | NormalizedFormula::LeftAtomicFormula { left: _, right } => fof::AndFormula(
                right
                    .arguments
                    .split_last()
                    .map(|x| x.1)
                    .unwrap_or(&[])
                    .iter()
                    .map(|x| {
                        fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(Box::new(
                            fof::AtomicFormula::Plain(fof::PlainAtomicFormula(
                                fof::PlainTerm::Function(
                                    common::Functor(common::AtomicWord::Lower(common::LowerWord(
                                        "exists",
                                    ))),
                                    Box::new(fof::Arguments(vec![
                                        x.to_tptp(),
                                        right.arguments.last().unwrap().to_tptp(),
                                    ])),
                                ),
                            )),
                        )))
                    })
                    .collect(),
            ),
            NormalizedFormula::RightAnd { left, right: _ }
            | NormalizedFormula::RightOr { left, right: _ }
            | NormalizedFormula::RightForall {
                variables: _,
                left,
                right: _,
            }
            | NormalizedFormula::RightExists {
                variables: _,
                left,
                right: _,
            }
            | NormalizedFormula::RightAtomicFormula { left, right: _ } => fof::AndFormula(
                left.arguments
                    .split_last()
                    .map(|x| x.1)
                    .unwrap_or(&[])
                    .iter()
                    .map(|x| {
                        fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(Box::new(
                            fof::AtomicFormula::Plain(fof::PlainAtomicFormula(
                                fof::PlainTerm::Function(
                                    common::Functor(common::AtomicWord::Lower(common::LowerWord(
                                        "exists",
                                    ))),
                                    Box::new(fof::Arguments(vec![
                                        x.to_tptp(),
                                        left.arguments.last().unwrap().to_tptp(),
                                    ])),
                                ),
                            )),
                        )))
                    })
                    .collect(),
            ),
            NormalizedFormula::PlainAxiom { formula } => fof::AndFormula(
                formula
                    .arguments
                    .split_last()
                    .map(|x| x.1)
                    .unwrap_or(&[])
                    .iter()
                    .map(|x| {
                        fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(Box::new(
                            fof::AtomicFormula::Plain(fof::PlainAtomicFormula(
                                fof::PlainTerm::Function(
                                    common::Functor(common::AtomicWord::Lower(common::LowerWord(
                                        "exists",
                                    ))),
                                    Box::new(fof::Arguments(vec![
                                        x.to_tptp(),
                                        formula.arguments.last().unwrap().to_tptp(),
                                    ])),
                                ),
                            )),
                        )))
                    }).collect()
                )
        };
        let formula = match self {
            NormalizedFormula::LeftAnd { left, right } => fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
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
            })),
            NormalizedFormula::RightAnd { left, right } => fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
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
            })),
            NormalizedFormula::LeftOr { left, right } => fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
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
            })),
            NormalizedFormula::RightOr { left, right } => fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
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
            })),
            NormalizedFormula::DoubleImplication { left, right } => fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
                left: Box::new(fof::UnitFormula::Unitary(
                    fof::UnitaryFormula::Parenthesised(Box::new(fof::LogicFormula::Binary(
                        fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
                            left: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                                Box::new(left.0.to_tptp()),
                            ))),
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
            })),
            NormalizedFormula::LeftForall {
                variables,
                left,
                right,
            } => fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
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
                            fof::UnitaryFormula::Parenthesised(Box::new(
                                fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
                                    left : Box::new(fof::UnitFormula::Unitary(
                                        fof::UnitaryFormula::Parenthesised(Box::new(
                                            fof::LogicFormula::Binary(fof::BinaryFormula::Assoc(
                                        fof::BinaryAssoc::And(fof::AndFormula(
                                            variables
                                                .iter()
                                                .map(|x| {
                                                    fof::UnitFormula::Unitary(
                                                        fof::UnitaryFormula::Atomic(Box::new(
                                                            fof::AtomicFormula::Plain(
                                                                fof::PlainAtomicFormula(
                                                                    fof::PlainTerm::Function(
                                                                        common::Functor(
                                                                            common::AtomicWord::Lower(
                                                                                common::LowerWord(
                                                                                    "exists",
                                                                                ),
                                                                            ),
                                                                        ),
                                                                        Box::new(fof::Arguments(vec![
                                                                            fof::Term::Variable(
                                                                                common::Variable(
                                                                                    common::UpperWord(
                                                                                        x,
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                            left
                                                                                .arguments
                                                                                .last()
                                                                                .unwrap()
                                                                                .to_tptp(),
                                                                        ])),
                                                                    ),
                                                                ),
                                                            ),
                                                        )),
                                                    )
                                                })
                                                .collect(),
                                        )),
                                    )))))),
                                    op : common::NonassocConnective::LRImplies,
                                    right : Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                                        Box::new(left.to_tptp()),
                                    ))), 
                                }),
                            )),
                        ))),
                    },
                ))),
                op: common::NonassocConnective::LRImplies,
                right: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                    Box::new(right.to_tptp()),
                ))),
            })),
            NormalizedFormula::RightForall {
                left,
                variables,
                right,
            } => fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
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
                            fof::UnitaryFormula::Parenthesised(Box::new(
                                fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
                                    left : Box::new(fof::UnitFormula::Unitary(
                                        fof::UnitaryFormula::Parenthesised(Box::new(
                                            fof::LogicFormula::Binary(fof::BinaryFormula::Assoc(
                                        fof::BinaryAssoc::And(fof::AndFormula(
                                            variables
                                                .iter()
                                                .map(|x| {
                                                    fof::UnitFormula::Unitary(
                                                        fof::UnitaryFormula::Atomic(Box::new(
                                                            fof::AtomicFormula::Plain(
                                                                fof::PlainAtomicFormula(
                                                                    fof::PlainTerm::Function(
                                                                        common::Functor(
                                                                            common::AtomicWord::Lower(
                                                                                common::LowerWord(
                                                                                    "exists",
                                                                                ),
                                                                            ),
                                                                        ),
                                                                        Box::new(fof::Arguments(vec![
                                                                            fof::Term::Variable(
                                                                                common::Variable(
                                                                                    common::UpperWord(
                                                                                        x,
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                            right
                                                                                .arguments
                                                                                .last()
                                                                                .unwrap()
                                                                                .to_tptp(),
                                                                        ])),
                                                                    ),
                                                                ),
                                                            ),
                                                        )),
                                                    )
                                                })
                                                .collect(),
                                        )),
                                    )))))),
                                    op : common::NonassocConnective::LRImplies,
                                    right : Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                                        Box::new(right.to_tptp()),
                                    ))), 
                                }),
                            )),
                        ))),
                    },
                ))),
            })),
            NormalizedFormula::LeftExists {
                variables,
                left,
                right,
            } => fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
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
                            fof::UnitaryFormula::Parenthesised(Box::new(
                                fof::LogicFormula::Binary(fof::BinaryFormula::Assoc(
                                    fof::BinaryAssoc::And(fof::AndFormula(
                                        variables
                                            .iter()
                                            .map(|x| {
                                                fof::UnitFormula::Unitary(
                                                    fof::UnitaryFormula::Atomic(Box::new(
                                                        fof::AtomicFormula::Plain(
                                                            fof::PlainAtomicFormula(
                                                                fof::PlainTerm::Function(
                                                                    common::Functor(
                                                                        common::AtomicWord::Lower(
                                                                            common::LowerWord(
                                                                                "exists",
                                                                            ),
                                                                        ),
                                                                    ),
                                                                    Box::new(fof::Arguments(vec![
                                                                        fof::Term::Variable(
                                                                            common::Variable(
                                                                                common::UpperWord(
                                                                                    x,
                                                                                ),
                                                                            ),
                                                                        ),
                                                                        right
                                                                            .arguments
                                                                            .last()
                                                                            .unwrap()
                                                                            .to_tptp(),
                                                                    ])),
                                                                ),
                                                            ),
                                                        ),
                                                    )),
                                                )
                                            })
                                            .chain(vec![fof::UnitFormula::Unitary(
                                                fof::UnitaryFormula::Atomic(Box::new(
                                                    left.to_tptp(),
                                                )),
                                            )])
                                            .collect(),
                                    )),
                                )),
                            )),
                        )),
                    },
                ))),
                op: common::NonassocConnective::LRImplies,
                right: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                    Box::new(right.to_tptp()),
                ))),
            })),
            NormalizedFormula::RightExists {
                left,
                variables,
                right,
            } => fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
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
                            fof::UnitaryFormula::Parenthesised(Box::new(
                                fof::LogicFormula::Binary(fof::BinaryFormula::Assoc(
                                    fof::BinaryAssoc::And(fof::AndFormula(
                                        variables
                                            .iter()
                                            .map(|x| {
                                                fof::UnitFormula::Unitary(
                                                    fof::UnitaryFormula::Atomic(Box::new(
                                                        fof::AtomicFormula::Plain(
                                                            fof::PlainAtomicFormula(
                                                                fof::PlainTerm::Function(
                                                                    common::Functor(
                                                                        common::AtomicWord::Lower(
                                                                            common::LowerWord(
                                                                                "exists",
                                                                            ),
                                                                        ),
                                                                    ),
                                                                    Box::new(fof::Arguments(vec![
                                                                        fof::Term::Variable(
                                                                            common::Variable(
                                                                                common::UpperWord(
                                                                                    x,
                                                                                ),
                                                                            ),
                                                                        ),
                                                                        left.arguments
                                                                            .last()
                                                                            .unwrap()
                                                                            .to_tptp(),
                                                                    ])),
                                                                ),
                                                            ),
                                                        ),
                                                    )),
                                                )
                                            })
                                            .chain(vec![fof::UnitFormula::Unitary(
                                                fof::UnitaryFormula::Atomic(Box::new(
                                                    right.to_tptp(),
                                                )),
                                            )])
                                            .collect(),
                                    )),
                                )),
                            )),
                        )),
                    },
                ))),
            })),
            NormalizedFormula::LeftAtomicFormula { left, right } => fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
                left: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                    Box::new(left.clone()),
                ))),
                op: common::NonassocConnective::LRImplies,
                right: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                    Box::new(right.to_tptp()),
                ))),
            })),
            NormalizedFormula::RightAtomicFormula { left, right } => fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
                left: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                    Box::new(left.to_tptp()),
                ))),
                op: common::NonassocConnective::LRImplies,
                right: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                    Box::new(right.clone()),
                ))),
            })),
            NormalizedFormula::PlainAxiom { formula } => fof::LogicFormula::Unitary(
                fof::UnitaryFormula::Atomic(Box::new(fof::AtomicFormula::Plain(fof::PlainAtomicFormula(fof::PlainTerm::Function(
                    common::Functor(common::AtomicWord::SingleQuoted(common::SingleQuoted(&formula.name))),
                    Box::new(fof::Arguments(formula.arguments.iter().map(|x| x.to_tptp()).collect()))
                ))))),
            ),
        };
        if exists.0.is_empty() {
            return fof::Formula(formula);
        } else {
            return fof::Formula(fof::LogicFormula::Binary(fof::BinaryFormula::Nonassoc(
                fof::BinaryNonassoc {
                    left: Box::new(fof::UnitFormula::Unitary(
                        fof::UnitaryFormula::Parenthesised(Box::new(fof::LogicFormula::Binary(
                            fof::BinaryFormula::Assoc(fof::BinaryAssoc::And(exists)),
                        ))),
                    )),
                    op: common::NonassocConnective::LRImplies,
                    right: Box::new(fof::UnitFormula::Unitary(
                        fof::UnitaryFormula::Parenthesised(Box::new(formula)),
                    )),
                },
            )));
        }
    }
}
