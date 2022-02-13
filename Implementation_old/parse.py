from parsimonious.grammar import Grammar, NodeVisitor
from formula import Disjunction, Conjunction, Implication, Equivalence, Forall, Exists, Negation, Atom, BTrue, BFalse
from term import Variable, FunctionApplication, Constant


proof = Grammar(
    """
    proof = "\\n"? (rule "\\n"?)*
    rule = id ". " formula " [" rulename id* "]"
    id = ~"[0-9]+"","?
    rulename = ~"[a-z\s]+"
    formula = disjunction / conjunction / implication / equivalence / parensFormula / quantifiedFormula / negation / atom
    disjunction = (quantifiedFormula / negation / atom / parensFormula) (" | " (quantifiedFormula / negation / atom / parensFormula))+
    conjunction = (quantifiedFormula / negation / atom / parensFormula) (" & " (quantifiedFormula / negation / atom / parensFormula))+
    implication = (quantifiedFormula / negation / atom / parensFormula) (" => " (quantifiedFormula / negation / atom / parensFormula))+
    equivalence = (quantifiedFormula / negation / atom / parensFormula) (" <=> " (quantifiedFormula / negation / atom / parensFormula))+
    parensFormula = "(" formula ")"
    quantifiedFormula = forall / exists
    forall = "! [" variable+ "] : " (parensFormula / quantifiedFormula / negation / atom)
    exists = "? [" variable+ "] : " (parensFormula / quantifiedFormula / negation / atom)
    negation = "~" (quantifiedFormula / atom / parensFormula)
    atom = equality / inequality / predicate / bool
    equality = term " = " term 
    inequality = term " != " term
    predicate = (name "(" term+ ")")
    bool = "$true" / "$false"
    term = (functionApplication / variable / constant)","?
    functionApplication = (skolemName / name) "(" term+ ")"
    name = ~"[a-z A-Z]([a-z A-Z 0-9]*)"
    variable = ~"X([0-9]+)"","?
    constant = skolemName / name
    skolemName = ~"sK([0-9]+)"
    """
)

class ProofVisitor(NodeVisitor):
    def __init__(self) -> None:
        self.result = {}

    def visit_rule(self, node, visited_children):
        id, _, formula , _, rulename, ids, _ = visited_children
        self.result[id] = (rulename, [] if ids == '' else ids, formula)

    def visit_id(self, node, visited_children):
        id, *_ = visited_children
        return int(id)

    def visit_formula(self, node, visited_children):
        return visited_children[0]

    def visit_disjunction(self, node, visited_children):
        arg0, args = visited_children
        return Disjunction([arg0[0]] + [i[1][0] for i in args])

    def visit_conjunction(self, node, visited_children):
        arg0, args = visited_children
        return Conjunction([arg0[0]] + [i[1][0] for i in args])

    def visit_implication(self, node, visited_children):
        arg0, args = visited_children
        return Implication([arg0[0]] + [i[1][0] for i in args])

    def visit_equivalence(self, node, visited_children):
        arg0, args = visited_children
        return Equivalence([arg0[0]] + [i[1][0] for i in args])
    
    def visit_parensFormula(self, node, visited_children):
        return visited_children[1]

    def visit_quantifiedFormula(self, node, visited_children):
        return visited_children[0]

    def visit_forall(self, node, visited_children):
        _, vars, _, f = visited_children
        return Forall(vars, f)

    def visit_exists(self, node, visited_children):
        _, vars, _, f = visited_children
        return Exists(vars, f)

    def visit_negation(self, node, visited_children):
        return Negation(visited_children[1])
    
    def visit_atom(self, node, visited_children):
        return visited_children[0]

    def visit_equality(self, node, visited_children):
        term1, _, term2 = visited_children
        return Atom("equals", [term1, term2])

    def visit_inequality(self, node, visited_children):
        term1, _, term2 = visited_children
        return Negation(Atom("equals", [term1, term2]))

    def visit_predicate(self, node, visited_children):
        name, _, args, _ = visited_children
        return Atom(name, args)

    def visit_bool(self, node, visited_children):
        return BTrue() if node.text == '$true' else BFalse()

    def visit_term(self, node, visited_children):
        term, *_ = visited_children
        return term[0]

    def visit_functionApplication(self, node, visited_children):
        name, _, args, _ = visited_children
        return FunctionApplication(name[0], args)

    def visit_variable(self, node, visited_children):
            var, *_ = visited_children
            return Variable(var)

    def visit_constant(self, node, visited_children):
            return Constant(node.text)

    def generic_visit(self, node, visited_children):
        return visited_children or node.text