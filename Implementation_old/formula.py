from dataclasses import dataclass, field
from typing import Sequence
from term import Term, Variable

class Formula:  
    def variables(self):
        vars = {}
        for arg in self.arguments:
            if vars:
                vars.update(arg.variables())
            else:
                vars = arg.variables()
        return vars
    
    def substitute(self, d):
        if type(self) == Atom:
            return type(self)(name=self.name, arguments=[arg.substitute(d) for arg in self.arguments])
        if type(self) in [Conjunction, Disjunction, Implication, Equivalence, Negation]:
            return type(self)(arguments=[arg.substitute(d) for arg in self.arguments])
        if type(self) in [Forall, Exists]:
            return type(self)(bound_variables=self.bound_variables, arguments=[self.arguments[0].substitute({k : d[k] for k in d.keys() if k not in self.bound_variables})])
        else:
            return self

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
            return '~(' + str(self.arguments[0]) + ')'
        if type(self) == BTrue:
            return 'True'
        if type(self) == BFalse:
            return 'False'
        if type(self) == Forall:
            return '! [' + ','.join(list(map(str, self.bound_variables))) + ']: ' + str(self.arguments[0])
        if type(self) == Exists:
            return '? [' + ','.join(list(map(str, self.bound_variables))) + ']: ' + str(self.arguments[0])

    def normalize(self):
        if type(self) == Forall:
            return self.arguments[0].substitute({x : Variable("input" + x.name[1:], []) for x in self.bound_variables}).normalize()
        if type(self) == Exists:
            return self.arguments[0].substitute({x : Variable("output" + x.name[1:], []) for x in self.bound_variables}).normalize()
        return self

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
    arguments : Sequence[Formula]

@dataclass
class Equivalence(Formula):
    arguments : Sequence[Formula]


@dataclass
class BTrue(Formula):
    arguments : Sequence[Formula] = field(default_factory=list)

@dataclass
class BFalse(Formula):
    arguments : Sequence[Formula] = field(default_factory=list)

@dataclass
class Forall(Formula):
    bound_variables : Sequence[Variable]
    arguments : Sequence[Formula]

@dataclass
class Exists(Formula):
    bound_variables : Sequence[Variable]
    arguments : Sequence[Formula]