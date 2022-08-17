#[path = "normalized_formula.rs"]
mod normalized_formula;

use std::collections::HashMap;
use std::ops::{Add, AddAssign};
use tptp::visitor::Visitor;
use tptp::{common, fof, top, TPTPIterator};

#[derive(Debug)]
pub struct Encoding<'a> {
    axioms: Vec<normalized_formula::NormalizedFormula<'a>>,
    goal: Option<normalized_formula::AtomizedSubformula>,
    variables: HashMap<String, Vec<String>>,
    pub f: HashMap<String, Vec<String>>,
    constants: Vec<String>,
    function_symbols: HashMap<String, Vec<String>>,
    relation_symbols: HashMap<String, Vec<String>>,
    polarity: bool,
}

impl<'a> Encoding<'a> {
    pub fn new() -> Encoding<'a> {
        Encoding {
            axioms: vec![],
            goal: None,
            variables: HashMap::new(),
            f: HashMap::new(),
            constants: Vec::new(),
            function_symbols: HashMap::new(),
            relation_symbols: HashMap::new(),
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
        for (f, args) in self.f.iter() {
            result.push(top::FofAnnotated(top::Annotated {
                name: common::Name::AtomicWord(common::AtomicWord::Lower(common::LowerWord("a"))),
                role: top::FormulaRole(common::LowerWord("axiom")),
                formula: Box::new(fof::Formula(fof::LogicFormula::Binary(
                    fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
                        left: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                            Box::new(fof::AtomicFormula::Plain(fof::PlainAtomicFormula(
                                fof::PlainTerm::Function(
                                    common::Functor(common::AtomicWord::Lower(common::LowerWord(
                                        "exists",
                                    ))),
                                    Box::new(fof::Arguments(vec![
                                        fof::Term::Variable(common::Variable(common::UpperWord(
                                            "X",
                                        ))),
                                        fof::Term::Variable(common::Variable(common::UpperWord(
                                            "CurrentWorld",
                                        ))),
                                    ])),
                                ),
                            ))),
                        ))),
                        op: common::NonassocConnective::LRImplies,
                        right: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                            Box::new(fof::AtomicFormula::Plain(fof::PlainAtomicFormula(
                                fof::PlainTerm::Function(
                                    common::Functor(common::AtomicWord::Lower(common::LowerWord(
                                        "exists",
                                    ))),
                                    Box::new(fof::Arguments(vec![
                                        fof::Term::Variable(common::Variable(common::UpperWord(
                                            "X",
                                        ))),
                                        fof::Term::Function(Box::new(fof::FunctionTerm::Plain(
                                            fof::PlainTerm::Function(
                                                common::Functor(common::AtomicWord::SingleQuoted(
                                                    common::SingleQuoted(f),
                                                )),
                                                Box::new(fof::Arguments(
                                                    args.iter()
                                                        .map(|x| {
                                                            fof::Term::Variable(common::Variable(
                                                                common::UpperWord(x),
                                                            ))
                                                        })
                                                        .chain(vec![fof::Term::Variable(
                                                            common::Variable(common::UpperWord(
                                                                "CurrentWorld",
                                                            )),
                                                        )])
                                                        .collect(),
                                                )),
                                            ),
                                        ))),
                                    ])),
                                ),
                            ))),
                        ))),
                    }),
                ))),
                annotations: top::Annotations(None),
            }));
            result.push(top::FofAnnotated(top::Annotated {
                name: common::Name::AtomicWord(common::AtomicWord::Lower(common::LowerWord("a"))),
                role: top::FormulaRole(common::LowerWord("axiom")),
                formula: Box::new(fof::Formula(fof::LogicFormula::Unitary(
                    fof::UnitaryFormula::Atomic(Box::new(fof::AtomicFormula::Defined(
                        fof::DefinedAtomicFormula::Infix(fof::DefinedInfixFormula {
                            left: Box::new(fof::Term::Function(Box::new(
                                fof::FunctionTerm::Plain(fof::PlainTerm::Function(
                                    common::Functor(common::AtomicWord::SingleQuoted(
                                        common::SingleQuoted(f),
                                    )),
                                    Box::new(fof::Arguments(
                                        args.iter()
                                            .map(|x| {
                                                fof::Term::Variable(common::Variable(
                                                    common::UpperWord(x),
                                                ))
                                            })
                                            .chain(vec![fof::Term::Variable(common::Variable(
                                                common::UpperWord("CurrentWorld"),
                                            ))])
                                            .collect(),
                                    )),
                                )),
                            ))),
                            op: common::DefinedInfixPred(common::InfixEquality),
                            right: Box::new(fof::Term::Function(Box::new(
                                fof::FunctionTerm::Plain(fof::PlainTerm::Function(
                                    common::Functor(common::AtomicWord::SingleQuoted(
                                        common::SingleQuoted(f),
                                    )),
                                    Box::new(fof::Arguments(
                                        args.iter()
                                            .map(|x| {
                                                fof::Term::Variable(common::Variable(
                                                    common::UpperWord(x),
                                                ))
                                            })
                                            .chain(vec![fof::Term::Function(Box::new(
                                                fof::FunctionTerm::Plain(fof::PlainTerm::Function(
                                                    common::Functor(
                                                        common::AtomicWord::SingleQuoted(
                                                            common::SingleQuoted(f),
                                                        ),
                                                    ),
                                                    Box::new(fof::Arguments(
                                                        args.iter()
                                                            .map(|x| {
                                                                fof::Term::Variable(
                                                                    common::Variable(
                                                                        common::UpperWord(x),
                                                                    ),
                                                                )
                                                            })
                                                            .chain(vec![fof::Term::Variable(
                                                                common::Variable(
                                                                    common::UpperWord(
                                                                        "CurrentWorld",
                                                                    ),
                                                                ),
                                                            )])
                                                            .collect(),
                                                    )),
                                                )),
                                            ))])
                                            .collect(),
                                    )),
                                )),
                            ))),
                        }),
                    ))),
                ))),
                annotations: top::Annotations(None),
            }));
        }
        for (f, args) in self.function_symbols.iter() {
            result.push(top::FofAnnotated(top::Annotated {
                name: common::Name::AtomicWord(common::AtomicWord::Lower(common::LowerWord("b"))),
                role: top::FormulaRole(common::LowerWord("axiom")),
                formula: Box::new(fof::Formula(fof::LogicFormula::Binary(
                    fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
                        left: Box::new(fof::UnitFormula::Unitary(
                            fof::UnitaryFormula::Parenthesised(Box::new(
                                fof::LogicFormula::Binary(fof::BinaryFormula::Assoc(
                                    fof::BinaryAssoc::And(fof::AndFormula(
                                        args.iter()
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
                                                                        fof::Term::Variable(
                                                                            common::Variable(
                                                                                common::UpperWord(
                                                                                    "CurrentWorld",
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    ])),
                                                                ),
                                                            ),
                                                        ),
                                                    )),
                                                )
                                            })
                                            .collect(),
                                    )),
                                )),
                            )),
                        )),
                        op: common::NonassocConnective::LRImplies,
                        right: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                            Box::new(fof::AtomicFormula::Plain(fof::PlainAtomicFormula(
                                fof::PlainTerm::Function(
                                    common::Functor(common::AtomicWord::Lower(common::LowerWord(
                                        "exists",
                                    ))),
                                    Box::new(fof::Arguments(vec![
                                        fof::Term::Function(Box::new(fof::FunctionTerm::Plain(
                                            fof::PlainTerm::Function(
                                                common::Functor(common::AtomicWord::SingleQuoted(
                                                    common::SingleQuoted(f),
                                                )),
                                                Box::new(fof::Arguments(
                                                    args.iter()
                                                        .map(|x| {
                                                            fof::Term::Variable(common::Variable(
                                                                common::UpperWord(x),
                                                            ))
                                                        })
                                                        .collect(),
                                                )),
                                            ),
                                        ))),
                                        fof::Term::Variable(common::Variable(common::UpperWord(
                                            "CurrentWorld",
                                        ))),
                                    ])),
                                ),
                            ))),
                        ))),
                    }),
                ))),
                annotations: top::Annotations(None),
            }))
        }
        for c in self.constants.iter() {
            result.push(top::FofAnnotated(top::Annotated {
                name: common::Name::AtomicWord(common::AtomicWord::Lower(common::LowerWord("c"))),
                role: top::FormulaRole(common::LowerWord("axiom")),
                formula: Box::new(fof::Formula(fof::LogicFormula::Binary(
                    fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
                        left: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Quantified(
                            fof::QuantifiedFormula {
                                quantifier: fof::Quantifier::Exists,
                                bound: fof::VariableList(vec![common::Variable(
                                    common::UpperWord("X"),
                                )]),
                                formula: Box::new(fof::UnitFormula::Unitary(
                                    fof::UnitaryFormula::Atomic(Box::new(
                                        fof::AtomicFormula::Plain(fof::PlainAtomicFormula(
                                            fof::PlainTerm::Function(
                                                common::Functor(common::AtomicWord::Lower(
                                                    common::LowerWord("exists"),
                                                )),
                                                Box::new(fof::Arguments(vec![
                                                    fof::Term::Variable(common::Variable(
                                                        common::UpperWord("X"),
                                                    )),
                                                    fof::Term::Variable(common::Variable(
                                                        common::UpperWord("CurrentWorld"),
                                                    )),
                                                ])),
                                            ),
                                        )),
                                    )),
                                )),
                            },
                        ))),
                        op: common::NonassocConnective::LRImplies,
                        right: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                            Box::new(fof::AtomicFormula::Plain(fof::PlainAtomicFormula(
                                fof::PlainTerm::Function(
                                    common::Functor(common::AtomicWord::Lower(common::LowerWord(
                                        "exists",
                                    ))),
                                    Box::new(fof::Arguments(vec![
                                        fof::Term::Variable(common::Variable(common::UpperWord(c))),
                                        fof::Term::Variable(common::Variable(common::UpperWord(
                                            "CurrentWorld",
                                        ))),
                                    ])),
                                ),
                            ))),
                        ))),
                    }),
                ))),
                annotations: top::Annotations(None),
            }))
        }
        for (r, r_args) in self.relation_symbols.iter() {
            for (f, f_args) in self.f.iter() {
                result.push(top::FofAnnotated(top::Annotated {
                    name: common::Name::AtomicWord(common::AtomicWord::Lower(common::LowerWord(
                        "d",
                    ))),
                    role: top::FormulaRole(common::LowerWord("axiom")),
                    formula: Box::new(fof::Formula(fof::LogicFormula::Binary(
                        fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
                            left: Box::new(fof::UnitFormula::Unitary(fof::UnitaryFormula::Atomic(
                                Box::new(fof::AtomicFormula::Plain(fof::PlainAtomicFormula(
                                    fof::PlainTerm::Function(
                                        common::Functor(common::AtomicWord::Lower(
                                            common::LowerWord(r),
                                        )),
                                        Box::new(fof::Arguments(
                                            r_args
                                                .iter()
                                                .map(|x| {
                                                    fof::Term::Variable(common::Variable(
                                                        common::UpperWord(x),
                                                    ))
                                                })
                                                .chain(vec![fof::Term::Variable(common::Variable(
                                                    common::UpperWord("CurrentWorld"),
                                                ))])
                                                .collect(),
                                        )),
                                    ),
                                ))),
                            ))),
                            op: common::NonassocConnective::LRImplies,
                            right: Box::new(fof::UnitFormula::Unitary(
                                fof::UnitaryFormula::Atomic(Box::new(fof::AtomicFormula::Plain(
                                    fof::PlainAtomicFormula(fof::PlainTerm::Function(
                                        common::Functor(common::AtomicWord::Lower(
                                            common::LowerWord(r),
                                        )),
                                        Box::new(fof::Arguments(
                                            r_args
                                                .iter()
                                                .map(|x| {
                                                    fof::Term::Variable(common::Variable(
                                                        common::UpperWord(x),
                                                    ))
                                                })
                                                .chain(vec![fof::Term::Function(Box::new(
                                                    fof::FunctionTerm::Plain(
                                                        fof::PlainTerm::Function(
                                                            common::Functor(
                                                                common::AtomicWord::SingleQuoted(
                                                                    common::SingleQuoted(f),
                                                                ),
                                                            ),
                                                            Box::new(fof::Arguments(
                                                                f_args
                                                                    .iter()
                                                                    .map(|x| {
                                                                        fof::Term::Variable(
                                                                            common::Variable(
                                                                                common::UpperWord(
                                                                                    x,
                                                                                ),
                                                                            ),
                                                                        )
                                                                    })
                                                                    .chain(vec![
                                                                        fof::Term::Variable(
                                                                            common::Variable(
                                                                                common::UpperWord(
                                                                                    "CurrentWorld",
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    ])
                                                                    .collect(),
                                                            )),
                                                        ),
                                                    ),
                                                ))])
                                                .collect(),
                                        )),
                                    )),
                                ))),
                            )),
                        }),
                    ))),
                    annotations: top::Annotations(None),
                }))
            }
        }
        for axiom in self.axioms.iter() {
            result.push(top::FofAnnotated(top::Annotated {
                name: common::Name::Integer(common::Integer("1")),
                role: top::FormulaRole(common::LowerWord("axiom")),
                formula: Box::new(axiom.to_tptp()),
                annotations: top::Annotations(None),
            }))
        }
        match self.goal {
            Some(ref goal) => {
                result.push(top::FofAnnotated(top::Annotated {
                    name: common::Name::Integer(common::Integer("1")),
                    role: top::FormulaRole(common::LowerWord("axiom")),
                    formula: Box::new(fof::Formula(fof::LogicFormula::Unitary(
                        fof::UnitaryFormula::Quantified(fof::QuantifiedFormula {
                            quantifier: fof::Quantifier::Exists,
                            bound: fof::VariableList(vec![common::Variable(common::UpperWord(
                                "X",
                            ))]),
                            formula: Box::new(fof::UnitFormula::Unitary(
                                fof::UnitaryFormula::Atomic(Box::new(fof::AtomicFormula::Plain(
                                    fof::PlainAtomicFormula(fof::PlainTerm::Function(
                                        common::Functor(common::AtomicWord::Lower(
                                            common::LowerWord("exists"),
                                        )),
                                        Box::new(fof::Arguments(vec![
                                            fof::Term::Variable(common::Variable(
                                                common::UpperWord("X"),
                                            )),
                                            fof::Term::Function(Box::new(
                                                fof::FunctionTerm::Plain(fof::PlainTerm::Constant(
                                                    common::Constant(common::Functor(
                                                        common::AtomicWord::Lower(
                                                            common::LowerWord("startworld"),
                                                        ),
                                                    )),
                                                )),
                                            )),
                                        ])),
                                    )),
                                ))),
                            )),
                        }),
                    ))),
                    annotations: top::Annotations(None),
                }));
                result.push(top::FofAnnotated(top::Annotated {
                    name: common::Name::AtomicWord(common::AtomicWord::Lower(common::LowerWord(
                        "goal",
                    ))),
                    role: top::FormulaRole(common::LowerWord("conjecture")),
                    formula: Box::new(fof::Formula(fof::LogicFormula::Unitary(
                        fof::UnitaryFormula::Atomic(Box::new(fof::AtomicFormula::Plain(
                            fof::PlainAtomicFormula(fof::PlainTerm::Function(
                                common::Functor(common::AtomicWord::SingleQuoted(
                                    common::SingleQuoted(&goal.name),
                                )),
                                Box::new(fof::Arguments(
                                    self.variables
                                        .get(&goal.name.to_string())
                                        .unwrap()
                                        .iter()
                                        .map(|x| {
                                            fof::Term::Variable(common::Variable(
                                                common::UpperWord(x),
                                            ))
                                        })
                                        .chain(vec![fof::Term::Function(Box::new(
                                            fof::FunctionTerm::Plain(fof::PlainTerm::Constant(
                                                common::Constant(common::Functor(
                                                    common::AtomicWord::Lower(common::LowerWord(
                                                        "startworld",
                                                    )),
                                                )),
                                            )),
                                        ))])
                                        .collect(),
                                )),
                            )),
                        ))),
                    ))),
                    annotations: top::Annotations(None),
                }));
            }
            None => {}
        }
        return result;
    }
}

impl<'a> Add for Encoding<'a> {
    type Output = Encoding<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Encoding::new();
        result.axioms = self
            .axioms
            .clone()
            .into_iter()
            .chain(
                rhs.axioms
                    .clone()
                    .into_iter()
                    .filter(|x| !self.axioms.contains(x)),
            )
            .collect();
        result.goal = match self.goal {
            Some(goal) => Some(goal),
            None => rhs.goal,
        };
        result.variables = self
            .variables
            .clone()
            .into_iter()
            .chain(rhs.variables.clone().into_iter())
            .collect();
        result.f = self
            .f
            .clone()
            .into_iter()
            .chain(rhs.f.clone().into_iter())
            .collect();
        result.constants = self
            .constants
            .clone()
            .into_iter()
            .chain(
                rhs.constants
                    .clone()
                    .into_iter()
                    .filter(|x| !self.constants.contains(x)),
            )
            .collect();
        result.function_symbols = self
            .variables
            .clone()
            .into_iter()
            .chain(rhs.function_symbols.clone().into_iter())
            .collect();
        result.relation_symbols = self
            .variables
            .clone()
            .into_iter()
            .chain(rhs.relation_symbols.clone().into_iter())
            .collect();
        return result;
    }
}

impl<'a> AddAssign for Encoding<'a> {
    fn add_assign(&mut self, rhs: Self) {
        let new_axioms: Vec<normalized_formula::NormalizedFormula> = rhs
            .axioms
            .clone()
            .into_iter()
            .filter(|x| !self.axioms.contains(x))
            .collect();
        self.axioms.extend(new_axioms);
        if self.goal.is_none() {
            self.goal = rhs.goal.clone();
        }
        self.variables.extend(rhs.variables.clone().into_iter());
        self.f.extend(rhs.f.clone().into_iter());
        let new_constants: Vec<String> = rhs
            .constants
            .clone()
            .into_iter()
            .filter(|x| !self.constants.contains(x))
            .collect();
        self.constants.extend(new_constants);
        self.function_symbols
            .extend(rhs.function_symbols.clone().into_iter());
        self.relation_symbols
            .extend(rhs.relation_symbols.clone().into_iter());
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

    fn visit_constant(&mut self, constant: &common::Constant<'a>) {
        self.variables.insert(constant.to_string(), vec![]);
        self.constants.push(constant.to_string());
        self.constants.sort_unstable();
        self.constants.dedup();
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
                self.visit_fof_function_term(fof_function_term);
                match **fof_function_term {
                    fof::FunctionTerm::Plain(ref plain_term) => match plain_term {
                        fof::PlainTerm::Constant(_) => {}
                        fof::PlainTerm::Function(ref functor, ref arguments) => {
                            self.function_symbols.insert(
                                functor.to_string(),
                                (0..arguments.0.len()).map(|x| format!("X_{}", x)).collect(),
                            );
                        }
                    },
                    fof::FunctionTerm::System(ref system_term) => match system_term {
                        fof::SystemTerm::Constant(_) => {}
                        fof::SystemTerm::Function(ref functor, ref arguments) => {
                            self.function_symbols.insert(
                                functor.to_string(),
                                (0..arguments.0.len()).map(|x| format!("X_{}", x)).collect(),
                            );
                        }
                    },
                    fof::FunctionTerm::Defined(_) => {}
                };
            }
            fof::Term::Variable(variable) => {
                self.visit_variable(variable);
                self.variables
                    .insert(fof_term.to_string(), vec![variable.to_string()]);
            }
        }
    }

    fn visit_fof_defined_atomic_formula(
        &mut self,
        fof_defined_atomic_formula: &fof::DefinedAtomicFormula<'a>,
    ) {
        match fof_defined_atomic_formula {
            fof::DefinedAtomicFormula::Plain(fof_defined_plain_formula) => {
                match fof_defined_plain_formula.0 {
                    fof::DefinedPlainTerm::Constant(_) => {
                        self.variables.insert(
                            remove_outer_parens(fof_defined_atomic_formula.to_string()),
                            vec![],
                        );
                    }
                    fof::DefinedPlainTerm::Function(_, ref box_arguments) => {
                        self.visit_fof_arguments(&box_arguments.clone());
                        self.variables.insert(
                            remove_outer_parens(fof_defined_atomic_formula.to_string()),
                            self.variables
                                .get(&remove_outer_parens(box_arguments.to_string()))
                                .unwrap()
                                .clone(),
                        );
                    }
                }
            }
            fof::DefinedAtomicFormula::Infix(fof_defined_infix_formula) => {
                self.visit_fof_defined_infix_formula(fof_defined_infix_formula);
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
                .get(&remove_outer_parens(
                    fof_defined_infix_formula.left.to_string(),
                ))
                .unwrap()
                .clone(),
        );
        self.visit_defined_infix_pred(fof_defined_infix_formula.op);
        self.visit_fof_term(&fof_defined_infix_formula.right);
        variables.extend(
            self.variables
                .get(&remove_outer_parens(
                    fof_defined_infix_formula.right.to_string(),
                ))
                .unwrap()
                .clone(),
        );
        variables.sort_unstable();
        variables.dedup();
        self.variables.insert(
            remove_outer_parens(fof_defined_infix_formula.to_string()),
            variables.clone(),
        );
    }

    fn visit_fof_atomic_formula(&mut self, fof_atomic_formula: &fof::AtomicFormula<'a>) {
        match fof_atomic_formula {
            fof::AtomicFormula::Plain(fof_plain_atomic_formula) => {
                self.visit_fof_plain_atomic_formula(fof_plain_atomic_formula);
                match fof_plain_atomic_formula.0 {
                    fof::PlainTerm::Constant(_) => {}
                    fof::PlainTerm::Function(ref functor, ref arguments) => {
                        self.relation_symbols.insert(
                            functor.to_string(),
                            (0..arguments.0.len()).map(|y| format!("Y_{}", y)).collect(),
                        );
                    }
                }
            }
            fof::AtomicFormula::Defined(fof_defined_atomic_formula) => {
                self.visit_fof_defined_atomic_formula(fof_defined_atomic_formula);
                self.variables.insert(
                    remove_outer_parens(fof_defined_atomic_formula.to_string()),
                    self.variables
                        .get(&remove_outer_parens(fof_defined_atomic_formula.to_string()))
                        .unwrap()
                        .clone(),
                );
                match self.polarity {
                    true => {
                        self.axioms
                            .push(normalized_formula::NormalizedFormula::LeftAtomicFormula {
                                left: fof_atomic_formula.clone(),
                                right: normalized_formula::AtomizedSubformula {
                                    name: fof_defined_atomic_formula.to_string(),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(
                                            fof_defined_atomic_formula.to_string(),
                                        ))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .chain(vec![normalized_formula::PlainTerm::Variable {
                                            name: String::from("CurrentWorld"),
                                        }])
                                        .collect(),
                                },
                            })
                    }
                    false => self.axioms.push(
                        normalized_formula::NormalizedFormula::RightAtomicFormula {
                            left: normalized_formula::AtomizedSubformula {
                                name: fof_defined_atomic_formula.to_string(),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(
                                        fof_defined_atomic_formula.to_string(),
                                    ))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .chain(vec![normalized_formula::PlainTerm::Variable {
                                        name: String::from("CurrentWorld"),
                                    }])
                                    .collect(),
                            },
                            right: fof_atomic_formula.clone(),
                        },
                    ),
                }
                return;
            }
            fof::AtomicFormula::System(fof_system_atomic_formula) => {
                self.visit_fof_system_atomic_formula(fof_system_atomic_formula);
                match fof_system_atomic_formula.0 {
                    fof::SystemTerm::Constant(_) => {}
                    fof::SystemTerm::Function(ref functor, ref arguments) => {
                        self.relation_symbols.insert(
                            functor.to_string(),
                            (0..arguments.0.len()).map(|y| format!("Y_{}", y)).collect(),
                        );
                    }
                }
            }
        }
        let atomized = normalized_formula::AtomizedSubformula {
            name: remove_outer_parens(fof_atomic_formula.to_string()),
            arguments: self
                .variables
                .get(&remove_outer_parens(fof_atomic_formula.to_string()))
                .unwrap()
                .clone()
                .into_iter()
                .chain(vec![String::from("CurrentWorld")].into_iter())
                .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                .collect(),
        };
        match self.polarity {
            true => self
                .axioms
                .push(normalized_formula::NormalizedFormula::LeftAtomicFormula {
                    left: extend_atom(fof_atomic_formula),
                    right: atomized,
                }),
            false => self
                .axioms
                .push(normalized_formula::NormalizedFormula::RightAtomicFormula {
                    left: atomized,
                    right: extend_atom(fof_atomic_formula),
                }),
        }
    }

    fn visit_fof_infix_unary(&mut self, fof_infix_unary: &fof::InfixUnary<'a>) {
        let equality = fof::AtomicFormula::Defined(fof::DefinedAtomicFormula::Infix(
            fof::DefinedInfixFormula {
                left: fof_infix_unary.left.clone(),
                op: common::DefinedInfixPred(common::InfixEquality),
                right: fof_infix_unary.right.clone(),
            },
        ));
        let polarity = self.polarity;
        self.polarity = !polarity;
        self.visit_fof_atomic_formula(&equality);
        self.variables.insert(
            remove_outer_parens(fof_infix_unary.to_string()),
            self.variables.get(&equality.to_string()).unwrap().clone(),
        );
        self.polarity = polarity;
        match self.polarity {
            true => {
                self.f.insert(
                    String::from("f_") + &remove_outer_parens(fof_infix_unary.to_string()),
                    (0..self
                        .variables
                        .get(&remove_outer_parens(fof_infix_unary.to_string()))
                        .unwrap()
                        .len())
                        .map(|z| format!("Z_{}", z))
                        .collect(),
                );
                self.axioms
                    .push(normalized_formula::NormalizedFormula::DoubleImplication {
                        left: (
                            normalized_formula::AtomizedSubformula {
                                name: remove_outer_parens(equality.to_string()),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(equality.to_string()))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .chain(
                                        vec![normalized_formula::PlainTerm::Function {
                                            name: String::from("f_")
                                                + &remove_outer_parens(fof_infix_unary.to_string()),
                                            arguments: self
                                                .variables
                                                .get(&(fof_infix_unary.to_string()))
                                                .unwrap()
                                                .clone()
                                                .into_iter()
                                                .chain(
                                                    vec![String::from("CurrentWorld")].into_iter(),
                                                )
                                                .map(|x| normalized_formula::PlainTerm::Variable {
                                                    name: x,
                                                })
                                                .collect(),
                                        }]
                                        .into_iter(),
                                    )
                                    .collect(),
                            },
                            normalized_formula::AtomizedSubformula {
                                name: String::from("false"),
                                arguments: vec![],
                            },
                        ),
                        right: normalized_formula::AtomizedSubformula {
                            name: remove_outer_parens(fof_infix_unary.to_string()),
                            arguments: self
                                .variables
                                .get(&remove_outer_parens(fof_infix_unary.to_string()))
                                .unwrap()
                                .clone()
                                .into_iter()
                                .chain(vec![String::from("CurrentWorld")].into_iter())
                                .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                .collect(),
                        },
                    });
            }
            false => self
                .axioms
                .push(normalized_formula::NormalizedFormula::LeftAnd {
                    left: vec![
                        normalized_formula::AtomizedSubformula {
                            name: remove_outer_parens(fof_infix_unary.to_string()),
                            arguments: self
                                .variables
                                .get(&remove_outer_parens(fof_infix_unary.to_string()))
                                .unwrap()
                                .clone()
                                .into_iter()
                                .chain(vec![String::from("CurrentWorld")].into_iter())
                                .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                .collect(),
                        },
                        normalized_formula::AtomizedSubformula {
                            name: remove_outer_parens(equality.to_string()),
                            arguments: self
                                .variables
                                .get(&remove_outer_parens(equality.to_string()))
                                .unwrap()
                                .clone()
                                .into_iter()
                                .chain(vec![String::from("CurrentWorld")].into_iter())
                                .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                .collect(),
                        },
                    ],
                    right: normalized_formula::AtomizedSubformula {
                        name: String::from("false"),
                        arguments: vec![],
                    },
                }),
        }
    }

    fn visit_fof_binary_nonassoc(&mut self, fof_binary_nonassoc: &fof::BinaryNonassoc<'a>) {
        let polarity = self.polarity;
        match fof_binary_nonassoc.op {
            common::NonassocConnective::LRImplies => match self.polarity {
                true => {
                    let mut variables = vec![];
                    self.polarity = false;
                    self.visit_fof_unit_formula(&fof_binary_nonassoc.left);
                    variables.extend(
                        self.variables
                            .get(&remove_outer_parens(fof_binary_nonassoc.left.to_string()))
                            .unwrap()
                            .clone(),
                    );
                    self.polarity = true;
                    self.visit_fof_unit_formula(&fof_binary_nonassoc.right);
                    variables.extend(
                        self.variables
                            .get(&remove_outer_parens(fof_binary_nonassoc.right.to_string()))
                            .unwrap()
                            .clone(),
                    );
                    self.polarity = polarity;
                    variables.sort_unstable();
                    variables.dedup();
                    self.variables.insert(
                        remove_outer_parens(fof_binary_nonassoc.to_string()),
                        variables,
                    );
                    self.f.insert(
                        String::from("f_") + &fof_binary_nonassoc.to_string(),
                        (0..self
                            .variables
                            .get(&remove_outer_parens(fof_binary_nonassoc.to_string()))
                            .unwrap()
                            .len())
                            .map(|z| format!("Z_{}", z))
                            .collect(),
                    );
                    self.axioms
                        .push(normalized_formula::NormalizedFormula::DoubleImplication {
                            left: (
                                normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(fof_binary_nonassoc.left.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(
                                            fof_binary_nonassoc.left.to_string(),
                                        ))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .chain(
                                            vec![normalized_formula::PlainTerm::Function {
                                                name: String::from("f_")
                                                    + &(fof_binary_nonassoc.to_string()),
                                                arguments: self
                                                    .variables
                                                    .get(&(fof_binary_nonassoc.to_string()))
                                                    .unwrap()
                                                    .clone()
                                                    .into_iter()
                                                    .chain(
                                                        vec![String::from("CurrentWorld")]
                                                            .into_iter(),
                                                    )
                                                    .map(|x| {
                                                        normalized_formula::PlainTerm::Variable {
                                                            name: x,
                                                        }
                                                    })
                                                    .collect(),
                                            }]
                                            .into_iter(),
                                        )
                                        .collect(),
                                },
                                normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(
                                        fof_binary_nonassoc.right.to_string(),
                                    ),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(
                                            fof_binary_nonassoc.right.to_string(),
                                        ))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .chain(
                                            vec![normalized_formula::PlainTerm::Function {
                                                name: String::from("f_")
                                                    + &(fof_binary_nonassoc.to_string()),
                                                arguments: self
                                                    .variables
                                                    .get(&(fof_binary_nonassoc.to_string()))
                                                    .unwrap()
                                                    .clone()
                                                    .into_iter()
                                                    .chain(
                                                        vec![String::from("CurrentWorld")]
                                                            .into_iter(),
                                                    )
                                                    .map(|x| {
                                                        normalized_formula::PlainTerm::Variable {
                                                            name: x,
                                                        }
                                                    })
                                                    .collect(),
                                            }]
                                            .into_iter(),
                                        )
                                        .collect(),
                                },
                            ),
                            right: normalized_formula::AtomizedSubformula {
                                name: remove_outer_parens(fof_binary_nonassoc.to_string()),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(fof_binary_nonassoc.to_string()))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .chain(vec![String::from("CurrentWorld")].into_iter())
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .collect(),
                            },
                        })
                }
                false => {
                    let mut variables = vec![];
                    self.polarity = true;
                    self.visit_fof_unit_formula(&fof_binary_nonassoc.left);
                    variables.extend(
                        self.variables
                            .get(&(fof_binary_nonassoc.left.to_string()))
                            .unwrap()
                            .clone(),
                    );
                    self.polarity = false;
                    self.visit_fof_unit_formula(&fof_binary_nonassoc.right);
                    variables.extend(
                        self.variables
                            .get(&remove_outer_parens(fof_binary_nonassoc.right.to_string()))
                            .unwrap()
                            .clone(),
                    );
                    self.polarity = polarity;
                    variables.sort_unstable();
                    variables.dedup();
                    self.variables.insert(
                        remove_outer_parens(fof_binary_nonassoc.to_string()),
                        variables,
                    );
                    self.axioms
                        .push(normalized_formula::NormalizedFormula::LeftAnd {
                            left: vec![
                                normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(fof_binary_nonassoc.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(fof_binary_nonassoc.to_string()))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                                normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(fof_binary_nonassoc.left.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(
                                            fof_binary_nonassoc.left.to_string(),
                                        ))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                            ],
                            right: normalized_formula::AtomizedSubformula {
                                name: remove_outer_parens(fof_binary_nonassoc.right.to_string()),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(
                                        fof_binary_nonassoc.right.to_string(),
                                    ))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .chain(vec![String::from("CurrentWorld")].into_iter())
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .collect(),
                            },
                        })
                }
            },
            common::NonassocConnective::RLImplies => {
                let lr = fof::BinaryNonassoc {
                    left: fof_binary_nonassoc.right.clone(),
                    op: common::NonassocConnective::LRImplies,
                    right: fof_binary_nonassoc.left.clone(),
                };
                self.visit_fof_binary_nonassoc(&lr);
                self.variables.insert(
                    fof_binary_nonassoc.to_string(),
                    self.variables
                        .get(&remove_outer_parens(lr.to_string()))
                        .unwrap()
                        .clone(),
                );
                self.polarity = polarity;
                match self.polarity {
                    true => self
                        .axioms
                        .push(normalized_formula::NormalizedFormula::LeftAnd {
                            left: vec![normalized_formula::AtomizedSubformula {
                                name: remove_outer_parens(lr.to_string()),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(lr.to_string()))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .chain(vec![String::from("CurrentWorld")].into_iter())
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .collect(),
                            }],
                            right: normalized_formula::AtomizedSubformula {
                                name: remove_outer_parens(fof_binary_nonassoc.to_string()),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(fof_binary_nonassoc.to_string()))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .chain(vec![String::from("CurrentWorld")].into_iter())
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .collect(),
                            },
                        }),
                    false => self
                        .axioms
                        .push(normalized_formula::NormalizedFormula::RightAnd {
                            left: normalized_formula::AtomizedSubformula {
                                name: remove_outer_parens(fof_binary_nonassoc.to_string()),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(fof_binary_nonassoc.to_string()))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .chain(vec![String::from("CurrentWorld")].into_iter())
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .collect(),
                            },
                            right: vec![normalized_formula::AtomizedSubformula {
                                name: remove_outer_parens(lr.to_string()),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(lr.to_string()))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .chain(vec![String::from("CurrentWorld")].into_iter())
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .collect(),
                            }],
                        }),
                }
            }
            common::NonassocConnective::Equivalent => {
                let lr = fof::BinaryNonassoc {
                    left: fof_binary_nonassoc.left.clone(),
                    op: common::NonassocConnective::LRImplies,
                    right: fof_binary_nonassoc.right.clone(),
                };
                let rl = fof::BinaryNonassoc {
                    left: fof_binary_nonassoc.right.clone(),
                    op: common::NonassocConnective::LRImplies,
                    right: fof_binary_nonassoc.left.clone(),
                };
                self.visit_fof_binary_nonassoc(&lr);
                self.visit_fof_binary_nonassoc(&rl);
                self.variables.insert(
                    fof_binary_nonassoc.to_string(),
                    self.variables
                        .get(&remove_outer_parens(lr.to_string()))
                        .unwrap()
                        .clone(),
                );
                self.polarity = polarity;
                match self.polarity {
                    true => self
                        .axioms
                        .push(normalized_formula::NormalizedFormula::LeftAnd {
                            left: vec![
                                normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(lr.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(lr.to_string()))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                                normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(rl.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(rl.to_string()))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                            ],
                            right: normalized_formula::AtomizedSubformula {
                                name: remove_outer_parens(fof_binary_nonassoc.to_string()),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(fof_binary_nonassoc.to_string()))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .chain(vec![String::from("CurrentWorld")].into_iter())
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .collect(),
                            },
                        }),
                    false => self
                        .axioms
                        .push(normalized_formula::NormalizedFormula::RightAnd {
                            left: normalized_formula::AtomizedSubformula {
                                name: remove_outer_parens(fof_binary_nonassoc.to_string()),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(fof_binary_nonassoc.to_string()))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .chain(vec![String::from("CurrentWorld")].into_iter())
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .collect(),
                            },
                            right: vec![
                                normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(lr.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(lr.to_string()))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                                normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(rl.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(rl.to_string()))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                            ],
                        }),
                }
            }
            common::NonassocConnective::NotEquivalent => {
                let equivalence = Box::new(fof::LogicFormula::Binary(
                    fof::BinaryFormula::Nonassoc(fof::BinaryNonassoc {
                        left: fof_binary_nonassoc.left.clone(),
                        op: common::NonassocConnective::Equivalent,
                        right: fof_binary_nonassoc.right.clone(),
                    }),
                ));
                self.visit_fof_unary_formula(&fof::UnaryFormula::Unary(
                    common::UnaryConnective,
                    Box::new(fof::UnitFormula::Unitary(
                        fof::UnitaryFormula::Parenthesised(equivalence.clone()),
                    )),
                ));
                self.variables.insert(
                    fof_binary_nonassoc.to_string(),
                    self.variables
                        .get(&remove_outer_parens(equivalence.to_string()))
                        .unwrap()
                        .clone(),
                );
                self.polarity = polarity;
                match self.polarity {
                    true => {
                        self.axioms
                            .push(normalized_formula::NormalizedFormula::DoubleImplication {
                                left: (
                                    normalized_formula::AtomizedSubformula {
                                        name: remove_outer_parens(equivalence.to_string()),
                                        arguments: self
                                            .variables
                                            .get(&remove_outer_parens(equivalence.to_string()))
                                            .unwrap()
                                            .clone()
                                            .into_iter()
                                            .chain(vec![String::from("CurrentWorld")].into_iter())
                                            .map(|x| normalized_formula::PlainTerm::Variable {
                                                name: x,
                                            })
                                            .collect(),
                                    },
                                    normalized_formula::AtomizedSubformula {
                                        name: String::from("false"),
                                        arguments: vec![],
                                    },
                                ),
                                right: normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(fof_binary_nonassoc.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(fof_binary_nonassoc.to_string()))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                            })
                    }
                    false => self
                        .axioms
                        .push(normalized_formula::NormalizedFormula::LeftAnd {
                            left: vec![
                                normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(equivalence.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(equivalence.to_string()))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                                normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(fof_binary_nonassoc.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(fof_binary_nonassoc.to_string()))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                            ],
                            right: normalized_formula::AtomizedSubformula {
                                name: String::from("false"),
                                arguments: vec![],
                            },
                        }),
                }
            }
            common::NonassocConnective::NotOr => {
                let or = Box::new(fof::LogicFormula::Binary(fof::BinaryFormula::Assoc(
                    fof::BinaryAssoc::Or(fof::OrFormula(vec![
                        *fof_binary_nonassoc.left.clone(),
                        *fof_binary_nonassoc.right.clone(),
                    ])),
                )));
                self.visit_fof_unary_formula(&fof::UnaryFormula::Unary(
                    common::UnaryConnective,
                    Box::new(fof::UnitFormula::Unitary(
                        fof::UnitaryFormula::Parenthesised(or.clone()),
                    )),
                ));
                self.variables.insert(
                    fof_binary_nonassoc.to_string(),
                    self.variables
                        .get(&remove_outer_parens(or.to_string()))
                        .unwrap()
                        .clone(),
                );
                self.polarity = polarity;
                match self.polarity {
                    true => {
                        self.axioms
                            .push(normalized_formula::NormalizedFormula::DoubleImplication {
                                left: (
                                    normalized_formula::AtomizedSubformula {
                                        name: remove_outer_parens(or.to_string()),
                                        arguments: self
                                            .variables
                                            .get(&remove_outer_parens(or.to_string()))
                                            .unwrap()
                                            .clone()
                                            .into_iter()
                                            .chain(vec![String::from("CurrentWorld")].into_iter())
                                            .map(|x| normalized_formula::PlainTerm::Variable {
                                                name: x,
                                            })
                                            .collect(),
                                    },
                                    normalized_formula::AtomizedSubformula {
                                        name: String::from("false"),
                                        arguments: vec![],
                                    },
                                ),
                                right: normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(fof_binary_nonassoc.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(fof_binary_nonassoc.to_string()))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                            })
                    }
                    false => self
                        .axioms
                        .push(normalized_formula::NormalizedFormula::LeftAnd {
                            left: vec![
                                normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(or.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(or.to_string()))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                                normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(fof_binary_nonassoc.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(fof_binary_nonassoc.to_string()))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                            ],
                            right: normalized_formula::AtomizedSubformula {
                                name: String::from("false"),
                                arguments: vec![],
                            },
                        }),
                }
            }
            common::NonassocConnective::NotAnd => {
                let and = Box::new(fof::LogicFormula::Binary(fof::BinaryFormula::Assoc(
                    fof::BinaryAssoc::And(fof::AndFormula(vec![
                        *fof_binary_nonassoc.left.clone(),
                        *fof_binary_nonassoc.right.clone(),
                    ])),
                )));
                self.visit_fof_unary_formula(&fof::UnaryFormula::Unary(
                    common::UnaryConnective,
                    Box::new(fof::UnitFormula::Unitary(
                        fof::UnitaryFormula::Parenthesised(and.clone()),
                    )),
                ));
                self.variables.insert(
                    fof_binary_nonassoc.to_string(),
                    self.variables
                        .get(&remove_outer_parens(and.to_string()))
                        .unwrap()
                        .clone(),
                );
                self.polarity = polarity;
                match self.polarity {
                    true => {
                        self.axioms
                            .push(normalized_formula::NormalizedFormula::DoubleImplication {
                                left: (
                                    normalized_formula::AtomizedSubformula {
                                        name: remove_outer_parens(and.to_string()),
                                        arguments: self
                                            .variables
                                            .get(&remove_outer_parens(and.to_string()))
                                            .unwrap()
                                            .clone()
                                            .into_iter()
                                            .chain(vec![String::from("CurrentWorld")].into_iter())
                                            .map(|x| normalized_formula::PlainTerm::Variable {
                                                name: x,
                                            })
                                            .collect(),
                                    },
                                    normalized_formula::AtomizedSubformula {
                                        name: String::from("false"),
                                        arguments: vec![],
                                    },
                                ),
                                right: normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(fof_binary_nonassoc.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(fof_binary_nonassoc.to_string()))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                            })
                    }
                    false => self
                        .axioms
                        .push(normalized_formula::NormalizedFormula::LeftAnd {
                            left: vec![
                                normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(and.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(and.to_string()))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                                normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(fof_binary_nonassoc.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(fof_binary_nonassoc.to_string()))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                            ],
                            right: normalized_formula::AtomizedSubformula {
                                name: String::from("false"),
                                arguments: vec![],
                            },
                        }),
                }
            }
        }
    }

    fn visit_fof_or_formula(&mut self, fof_or_formula: &fof::OrFormula<'a>) {
        let mut variables = vec![];
        let polarity = self.polarity;
        for fof_unit_formula in &*fof_or_formula.0 {
            self.visit_fof_unit_formula(fof_unit_formula);
            variables.extend(
                self.variables
                    .get(&remove_outer_parens(fof_unit_formula.to_string()))
                    .unwrap()
                    .clone(),
            );
            self.polarity = polarity;
        }
        variables.sort_unstable();
        variables.dedup();
        self.variables.insert(
            remove_outer_parens(fof_or_formula.to_string()),
            variables.clone(),
        );

        match self.polarity {
            true => self
                .axioms
                .push(normalized_formula::NormalizedFormula::LeftOr {
                    left: fof_or_formula
                        .0
                        .iter()
                        .map(|x| normalized_formula::AtomizedSubformula {
                            name: remove_outer_parens(x.to_string()),
                            arguments: self
                                .variables
                                .get(&remove_outer_parens(x.to_string()))
                                .unwrap()
                                .clone()
                                .into_iter()
                                .chain(vec![String::from("CurrentWorld")].into_iter())
                                .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                .collect(),
                        })
                        .collect(),
                    right: normalized_formula::AtomizedSubformula {
                        name: remove_outer_parens(fof_or_formula.to_string()),
                        arguments: self
                            .variables
                            .get(&remove_outer_parens(fof_or_formula.to_string()))
                            .unwrap()
                            .clone()
                            .into_iter()
                            .chain(vec![String::from("CurrentWorld")].into_iter())
                            .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                            .collect(),
                    },
                }),
            false => self
                .axioms
                .push(normalized_formula::NormalizedFormula::RightOr {
                    left: normalized_formula::AtomizedSubformula {
                        name: remove_outer_parens(fof_or_formula.to_string()),
                        arguments: self
                            .variables
                            .get(&remove_outer_parens(fof_or_formula.to_string()))
                            .unwrap()
                            .clone()
                            .into_iter()
                            .chain(vec![String::from("CurrentWorld")].into_iter())
                            .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                            .collect(),
                    },
                    right: fof_or_formula
                        .0
                        .iter()
                        .map(|x| normalized_formula::AtomizedSubformula {
                            name: remove_outer_parens(x.to_string()),
                            arguments: self
                                .variables
                                .get(&remove_outer_parens(x.to_string()))
                                .unwrap()
                                .clone()
                                .into_iter()
                                .chain(vec![String::from("CurrentWorld")].into_iter())
                                .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                .collect(),
                        })
                        .collect(),
                }),
        }
    }

    fn visit_fof_and_formula(&mut self, fof_and_formula: &fof::AndFormula<'a>) {
        let mut variables = vec![];
        let polarity = self.polarity;
        for fof_unit_formula in &*fof_and_formula.0 {
            self.visit_fof_unit_formula(fof_unit_formula);
            variables.extend(
                self.variables
                    .get(&(fof_unit_formula.to_string()))
                    .unwrap()
                    .clone(),
            );
            self.polarity = polarity;
        }
        variables.sort_unstable();
        variables.dedup();
        self.variables
            .insert(fof_and_formula.to_string(), variables.clone());
        match self.polarity {
            true => self
                .axioms
                .push(normalized_formula::NormalizedFormula::LeftAnd {
                    left: fof_and_formula
                        .0
                        .iter()
                        .map(|x| normalized_formula::AtomizedSubformula {
                            name: remove_outer_parens(x.to_string()),
                            arguments: self
                                .variables
                                .get(&remove_outer_parens(x.to_string()))
                                .unwrap()
                                .clone()
                                .into_iter()
                                .chain(vec![String::from("CurrentWorld")].into_iter())
                                .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                .collect(),
                        })
                        .collect(),
                    right: normalized_formula::AtomizedSubformula {
                        name: remove_outer_parens(fof_and_formula.to_string()),
                        arguments: self
                            .variables
                            .get(&remove_outer_parens(fof_and_formula.to_string()))
                            .unwrap()
                            .clone()
                            .into_iter()
                            .chain(vec![String::from("CurrentWorld")].into_iter())
                            .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                            .collect(),
                    },
                }),
            false => self
                .axioms
                .push(normalized_formula::NormalizedFormula::RightAnd {
                    left: normalized_formula::AtomizedSubformula {
                        name: remove_outer_parens(fof_and_formula.to_string()),
                        arguments: self
                            .variables
                            .get(&remove_outer_parens(fof_and_formula.to_string()))
                            .unwrap()
                            .clone()
                            .into_iter()
                            .chain(vec![String::from("CurrentWorld")].into_iter())
                            .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                            .collect(),
                    },
                    right: fof_and_formula
                        .0
                        .iter()
                        .map(|x| normalized_formula::AtomizedSubformula {
                            name: remove_outer_parens(x.to_string()),
                            arguments: self
                                .variables
                                .get(&remove_outer_parens(x.to_string()))
                                .unwrap()
                                .clone()
                                .into_iter()
                                .chain(vec![String::from("CurrentWorld")].into_iter())
                                .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                .collect(),
                        })
                        .collect(),
                }),
        }
    }

    fn visit_fof_unary_formula(&mut self, fof_unary_formula: &fof::UnaryFormula<'a>) {
        match fof_unary_formula {
            fof::UnaryFormula::Unary(_, fof_unit_formula) => {
                let polarity = self.polarity;
                self.polarity = !self.polarity;
                self.visit_fof_unit_formula(&fof_unit_formula);
                self.polarity = polarity;
                self.variables.insert(
                    remove_outer_parens(fof_unary_formula.to_string()),
                    self.variables
                        .get(&remove_outer_parens(fof_unit_formula.to_string()))
                        .unwrap()
                        .clone(),
                );
                match self.polarity {
                    true => {
                        self.f.insert(
                            String::from("f_") + &fof_unary_formula.to_string(),
                            (0..self
                                .variables
                                .get(&remove_outer_parens(fof_unary_formula.to_string()))
                                .unwrap()
                                .len())
                                .map(|z| format!("Z_{}", z))
                                .collect(),
                        );
                        self.axioms
                            .push(normalized_formula::NormalizedFormula::DoubleImplication {
                                left: (
                                    normalized_formula::AtomizedSubformula {
                                        name: remove_outer_parens(fof_unit_formula.to_string()),
                                        arguments: self
                                            .variables
                                            .get(&remove_outer_parens(fof_unit_formula.to_string()))
                                            .unwrap()
                                            .clone()
                                            .into_iter()
                                            .map(|x| normalized_formula::PlainTerm::Variable {
                                                name: x,
                                            })
                                            .chain(
                                                vec![normalized_formula::PlainTerm::Function {
                                                name: String::from("f_")
                                                    + &(fof_unary_formula.to_string()),
                                                arguments: self
                                                    .variables
                                                    .get(&(fof_unary_formula.to_string()))
                                                    .unwrap()
                                                    .clone()
                                                    .into_iter()
                                                    .chain(
                                                        vec![String::from("CurrentWorld")]
                                                            .into_iter(),
                                                    )
                                                    .map(|x| {
                                                        normalized_formula::PlainTerm::Variable {
                                                            name: x,
                                                        }
                                                    })
                                                    .collect(),
                                            }]
                                                .into_iter(),
                                            )
                                            .collect(),
                                    },
                                    normalized_formula::AtomizedSubformula {
                                        name: String::from("false"),
                                        arguments: vec![],
                                    },
                                ),
                                right: normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(fof_unary_formula.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(fof_unary_formula.to_string()))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                            })
                    }
                    false => self
                        .axioms
                        .push(normalized_formula::NormalizedFormula::LeftAnd {
                            left: vec![
                                normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(fof_unit_formula.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(fof_unit_formula.to_string()))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                                normalized_formula::AtomizedSubformula {
                                    name: remove_outer_parens(fof_unary_formula.to_string()),
                                    arguments: self
                                        .variables
                                        .get(&remove_outer_parens(fof_unary_formula.to_string()))
                                        .unwrap()
                                        .clone()
                                        .into_iter()
                                        .chain(vec![String::from("CurrentWorld")].into_iter())
                                        .map(|x| normalized_formula::PlainTerm::Variable {
                                            name: x,
                                        })
                                        .collect(),
                                },
                            ],
                            right: normalized_formula::AtomizedSubformula {
                                name: String::from("false"),
                                arguments: vec![],
                            },
                        }),
                }
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
        let polarity = self.polarity;
        self.visit_fof_unit_formula(&fof_quantified_formula.formula);
        self.polarity = polarity;
        for v in self
            .variables
            .get(&remove_outer_parens(
                fof_quantified_formula.formula.to_string(),
            ))
            .unwrap()
            .iter()
        {
            if !namelist.contains(v) {
                variables.push(v.clone());
            }
        }
        variables.sort_unstable();
        variables.dedup();
        self.variables.insert(
            remove_outer_parens(fof_quantified_formula.to_string()),
            variables.clone(),
        );
        match self.polarity {
            true => match fof_quantified_formula.quantifier {
                fof::Quantifier::Forall => {
                    self.f.insert(
                        String::from("f_")
                            + &remove_outer_parens(fof_quantified_formula.to_string()),
                        (0..self
                            .variables
                            .get(&remove_outer_parens(fof_quantified_formula.to_string()))
                            .unwrap()
                            .len())
                            .map(|z| format!("Z_{}", z))
                            .collect(),
                    );
                    self.axioms
                        .push(normalized_formula::NormalizedFormula::LeftForall {
                            variables: fof_quantified_formula
                                .bound
                                .0
                                .iter()
                                .map(|x| x.to_string())
                                .collect(),
                            left: normalized_formula::AtomizedSubformula {
                                name: remove_outer_parens(
                                    fof_quantified_formula.formula.to_string(),
                                ),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(
                                        fof_quantified_formula.formula.to_string(),
                                    ))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .chain(
                                        vec![normalized_formula::PlainTerm::Function {
                                            name: String::from("f_")
                                                + &(fof_quantified_formula.to_string()),
                                            arguments: self
                                                .variables
                                                .get(&(fof_quantified_formula.to_string()))
                                                .unwrap()
                                                .clone()
                                                .into_iter()
                                                .chain(
                                                    vec![String::from("CurrentWorld")].into_iter(),
                                                )
                                                .map(|x| normalized_formula::PlainTerm::Variable {
                                                    name: x,
                                                })
                                                .collect(),
                                        }]
                                        .into_iter(),
                                    )
                                    .collect(),
                            },
                            right: normalized_formula::AtomizedSubformula {
                                name: remove_outer_parens(fof_quantified_formula.to_string()),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(fof_quantified_formula.to_string()))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .chain(vec![String::from("CurrentWorld")].into_iter())
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .collect(),
                            },
                        })
                }
                fof::Quantifier::Exists => {
                    self.axioms
                        .push(normalized_formula::NormalizedFormula::LeftExists {
                            variables: fof_quantified_formula
                                .bound
                                .0
                                .iter()
                                .map(|x| x.to_string())
                                .collect(),
                            left: normalized_formula::AtomizedSubformula {
                                name: remove_outer_parens(
                                    fof_quantified_formula.formula.to_string(),
                                ),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(
                                        fof_quantified_formula.formula.to_string(),
                                    ))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .chain(vec![String::from("CurrentWorld")].into_iter())
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .collect(),
                            },
                            right: normalized_formula::AtomizedSubformula {
                                name: remove_outer_parens(fof_quantified_formula.to_string()),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(fof_quantified_formula.to_string()))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .chain(vec![String::from("CurrentWorld")].into_iter())
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .collect(),
                            },
                        })
                }
            },
            false => match fof_quantified_formula.quantifier {
                fof::Quantifier::Forall => {
                    self.axioms
                        .push(normalized_formula::NormalizedFormula::RightForall {
                            left: normalized_formula::AtomizedSubformula {
                                name: remove_outer_parens(fof_quantified_formula.to_string()),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(fof_quantified_formula.to_string()))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .chain(vec![String::from("CurrentWorld")].into_iter())
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .collect(),
                            },
                            variables: fof_quantified_formula
                                .bound
                                .0
                                .iter()
                                .map(|x| x.to_string())
                                .collect(),
                            right: normalized_formula::AtomizedSubformula {
                                name: remove_outer_parens(
                                    fof_quantified_formula.formula.to_string(),
                                ),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(
                                        fof_quantified_formula.formula.to_string(),
                                    ))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .chain(vec![String::from("CurrentWorld")].into_iter())
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .collect(),
                            },
                        })
                }
                fof::Quantifier::Exists => {
                    self.axioms
                        .push(normalized_formula::NormalizedFormula::RightExists {
                            left: normalized_formula::AtomizedSubformula {
                                name: remove_outer_parens(fof_quantified_formula.to_string()),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(fof_quantified_formula.to_string()))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .chain(vec![String::from("CurrentWorld")].into_iter())
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .collect(),
                            },
                            variables: fof_quantified_formula
                                .bound
                                .0
                                .iter()
                                .map(|x| x.to_string())
                                .collect(),
                            right: normalized_formula::AtomizedSubformula {
                                name: remove_outer_parens(
                                    fof_quantified_formula.formula.to_string(),
                                ),
                                arguments: self
                                    .variables
                                    .get(&remove_outer_parens(
                                        fof_quantified_formula.formula.to_string(),
                                    ))
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .chain(vec![String::from("CurrentWorld")].into_iter())
                                    .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                    .collect(),
                            },
                        })
                }
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
                self.visit_fof_logic_formula(boxed_fof_logic_formula.as_ref());
                self.variables.insert(
                    fof_unitary_formula.to_string(),
                    self.variables
                        .get(&remove_outer_parens(boxed_fof_logic_formula.to_string()))
                        .unwrap()
                        .clone(),
                );
            }
        }
    }

    fn visit_fof_annotated(&mut self, fof_annotated: &tptp::top::FofAnnotated<'a>) {
        match fof_annotated.0.role.0 .0 {
            "axiom" | "hypothesis" => {
                self.polarity = false;
                self.visit_fof_formula(&fof_annotated.0.formula);
                self.axioms
                    .push(normalized_formula::NormalizedFormula::PlainAxiom {
                        formula: normalized_formula::AtomizedSubformula {
                            name: remove_outer_parens(fof_annotated.0.formula.to_string()),
                            arguments: self
                                .variables
                                .get(&remove_outer_parens(fof_annotated.0.formula.to_string()))
                                .unwrap()
                                .clone()
                                .into_iter()
                                .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                                .chain(
                                    vec![normalized_formula::PlainTerm::Variable {
                                        name: String::from("CurrentWorld"),
                                    }]
                                    .into_iter(),
                                )
                                .collect(),
                        },
                    });
            }
            "lemma" | "theorem" => {}
            "conjecture" => {
                self.polarity = true;
                self.visit_fof_formula(&fof_annotated.0.formula);
                self.goal = Some(normalized_formula::AtomizedSubformula {
                    name: remove_outer_parens(fof_annotated.0.formula.to_string()),
                    arguments: self
                        .variables
                        .get(&remove_outer_parens(fof_annotated.0.formula.to_string()))
                        .unwrap()
                        .clone()
                        .into_iter()
                        .map(|x| normalized_formula::PlainTerm::Variable { name: x })
                        .collect(),
                });
            }
            _ => panic!(
                "\"{}\" is neither axiom, conjecture nor hypothesis",
                fof_annotated.to_string()
            ),
        }
    }
}

fn remove_outer_parens(s: String) -> String {
    if s.chars().next() != Some('(') || s.chars().last() != Some(')') {
        return s;
    } else {
        let mut counter = 1;
        let mut it = s.chars().skip(1);
        while counter != 0 {
            match it.next().unwrap() {
                '(' => counter += 1,
                ')' => counter -= 1,
                _ => {}
            }
        }
        match it.next() {
            Some(_) => return s,
            None => {
                let mut s = s.chars();
                s.next();
                s.next_back();
                return remove_outer_parens(String::from(s.as_str()));
            }
        }
    }
}

fn extend_atom<'a>(fof_atomic_formula: &fof::AtomicFormula<'a>) -> fof::AtomicFormula<'a> {
    match fof_atomic_formula {
        fof::AtomicFormula::Plain(plain_atomic_formula) => {
            fof::AtomicFormula::Plain(fof::PlainAtomicFormula(match plain_atomic_formula.0 {
                fof::PlainTerm::Constant(ref constant) => fof::PlainTerm::Function(
                    common::Functor(constant.0 .0.clone()),
                    Box::new(fof::Arguments(vec![fof::Term::Variable(common::Variable(
                        common::UpperWord("CurrentWorld"),
                    ))])),
                ),
                fof::PlainTerm::Function(ref functor, ref arguments_box) => {
                    fof::PlainTerm::Function(
                        functor.clone(),
                        Box::new(fof::Arguments(
                            (*arguments_box.clone())
                                .0
                                .into_iter()
                                .chain(
                                    vec![fof::Term::Variable(common::Variable(common::UpperWord(
                                        "CurrentWorld",
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
                                                common::UpperWord("CurrentWorld"),
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
            fof::DefinedAtomicFormula::Infix(_) => fof_atomic_formula.clone(),
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
                                        "CurrentWorld",
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
