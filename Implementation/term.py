from dataclasses import dataclass
from typing import Sequence


class Term:
    def variables(self):
        if type(self) == FunctionApplication:
            vars = None
            for arg in self.arguments:
                if vars:
                    vars.update(arg.variables())
                else:
                    vars = arg.variables()
            return vars
        if type(self) == Constant:
            return {}
        if type(self) == Variable:
            return {self}

    def substitute(self, d):
        if not d:
            return self
        if type(self) == FunctionApplication:
            return FunctionApplication(self.name, arguments=[arg.substitute(d) for arg in self.arguments])
        if type(self) == Constant:
            return Constant(self.name)
        if type(self) == Variable:
            if self in d.keys():
                return(d[self])
            else:
                return(Variable(self.name))

    def mgu(self, f):
        if type(self) == Variable:
            return ({self : f}, {})
        if type(f) == Variable:
            return ({}, {f : self})
        if type(self) == FunctionApplication and type(f) == FunctionApplication:
            s = [{}, {}]
            for arg1, arg2 in zip(self.arguments, f.arguments):
                arg1, arg2 = arg1.substitute(s[0]), arg2.substitute(s[1])
                si = arg1.mgu(arg2)
                for i in [0, 1]:
                    if s[i]:
                        s[i].update(si[i])
                    else:
                        s[i] = si[i]
            return s
        return ({}, {})

    def __str__(self):
        if type(self) in [Variable, Constant]:
            return self.name
        if type(self) == FunctionApplication:
            return self.name + '(' + ','.join(list(map(str, self.arguments))) + ')'


@dataclass
class Variable(Term):
    name : str

    def __hash__(self):
        return hash(self.name)

@dataclass
class Constant(Term):
    name : str

@dataclass
class FunctionApplication(Term):
    name : str
    arguments : Sequence[Term]
