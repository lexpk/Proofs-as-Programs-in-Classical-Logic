from dataclasses import dataclass
from typing import Sequence
from term import Term, Variable

class Formula:  
    def variables(self):
        if type(self) == Atom:
            vars = {}
            for arg in self.arguments:
                if vars:
                    vars.update(arg.variables())
                else:
                    vars = arg.variables()
            return vars
        if type(self) in [Conjunction, Disjunction, Implication, Equivalence]:
            vars = {}
            for arg in self.arguments:
                if vars:
                    vars.update(arg.variables())
                else:
                    vars = arg.variables()
            return vars
        if type(self) == Negation:
            return self.argument.variables()
        if type(self) in [Forall, Exists]:
            return self.formula.variables()
        else:
            return {}
    
    def substitute(self, d):
        if type(self) == Atom:
            return type(self)(name=self.name, arguments=[arg.substitute(d) for arg in self.arguments])
        if type(self) in [Conjunction, Disjunction, Implication, Equivalence]:
            return type(self)(arguments=[arg.substitute(d) for arg in self.arguments])
        if type(self) == Negation:
            return Negation(argument=self.argument.substitute(d))
        if type(self) in [BTrue, BFalse]:
            return self
        if type(self) in [Forall, Exists]:
            return type(self)(bound_variables=self.bound_variables, formula=self.formula.substitute({k : d[k] for k in d.keys() if k not in self.bound_variables}))

    def __str__(self):
        if type(self) == Atom:
            return self.name + '(' + ','.join(list(map(str, self.arguments))) +')'
        if type(self) == Conjunction:
            return '(' + ') & ('.join(list(map(str, self.arguments))) + ')'
        if type(self) == Disjunction:
            return '(' + ') | ('.join(list(map(str, self.arguments))) + ')'
        if type(self) == Implication:
            return '(' + ') => ('.join(list(map(str, self.arguments))) + ')'
        if type(self) == Equivalence:
            return '(' + ') <=> ('.join(list(map(str, self.arguments))) + ')'
        if type(self) == Negation:
            return '~(' + str(self.argument) + ')'
        if type(self) == BTrue:
            return 'True'
        if type(self) == BFalse:
            return 'False'
        if type(self) == Forall:
            return '! [' + ','.join(list(map(str, self.bound_variables))) + ']: ' + str(self.formula)
        if type(self) == Exists:
            return '! [' + ','.join(list(map(str, self.bound_variables))) + ']: ' + str(self.formula)

@dataclass
class Atom(Formula):
    name : str
    arguments : Sequence[Term]


@dataclass
class Conjunction(Formula):
    arguments : Sequence[Formula]

@dataclass
class Disjunction(Formula):
    arguments : Sequence[Formula]

@dataclass
class Implication(Formula):
    arguments : Sequence[Formula]

@dataclass
class Negation(Formula):
    argument : Formula

@dataclass
class Equivalence(Formula):
    arguments : Sequence[Formula]


@dataclass
class BTrue(Formula):
    pass

@dataclass
class BFalse(Formula):
    pass

@dataclass
class Forall(Formula):
    bound_variables : Sequence[Variable]
    formula : Formula = BTrue()

@dataclass
class Exists(Formula):
    bound_variables : Sequence[Variable]
    formula : Formula = BTrue()
