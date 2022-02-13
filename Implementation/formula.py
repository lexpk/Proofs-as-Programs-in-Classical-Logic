from ast import arguments
from term import Term, Signature

from functools import reduce
from typing import Dict, Set


class Signature:
    def __init__(self, atoms: Dict[str, int], connectors: Dict[str, int], quantifiers: Set[str], termType: type):
        self._d = {**atoms, **connectors}
        self.termType = termType
        self.atoms = set(atoms.keys()) | {c for c, a in connectors.items() if a == 0}
        self.connectors = {c for c, a in connectors.items() if a > 0}
        self.quantifiers = quantifiers
        self.symbols = self.connectors | self.atoms | self.quantifiers
        self.arity = lambda x: self._d[x]

    def __repr__(self):
        return f"""
        Atoms: {", ".join([f"{s}/{self.termArity(s)}" for s in self.atoms])}
        Connectors: {", ".join([f"{s}/{self.formulaArity(s)}" for s in self.connectors])}
        Quantifiers: {", ".join([f"{s}" for s in self.quantifiers])}
        """

    def __eq__(self, other) -> bool:
        return self._d == other._d

def Formula(sig: Signature):
    class _Formula:
        signature = sig
        
        def __init__(self, name: str, arguments = [], boundVariables = []):
            if name in _Formula.signature.quantifiers:
                self.name = name
                self.boundVariables = boundVariables
                self.arguments = arguments
            else:
                if name in _Formula.signature.atoms | _Formula.signature.connectors:
                    assert len(arguments) == _Formula.signature.arity(name)
                    assert all(type(arg) == _Formula.signature.termType if name in _Formula.signature.atoms else type(arg) == type(self) for arg in arguments) 
                    self.name = name
                    self.arguments = arguments
                else:
                    assert len(arguments) == 0
                    self.name = name
                    self.arguments = []

        def __repr__(self) -> str:
            if self.name not in _Formula.signature.quantifiers:   
                return self.name + (("(" + ", ".join(map(str, self.arguments)) + ")") if self.arguments else "")
            else:
                return self.name + (("[" + ", ".join(map(str, self.boundVariables)) + "]")if self.boundVariables else "") + (("(" + ", ".join(map(str, self.arguments)) + ")") if self.arguments else "")


        def __eq__(self, other) -> bool:
            return self.name == other.name and self.arguments == other.arguments

        def __hash__(self) -> int:
            return hash(repr(self))

        def variables(self) -> set():
            return reduce(lambda x, y: x | y, map(lambda arg: arg.variables(), self.arguments), set())

        def freeVariables(self) -> set():
            if self.isQuantifierFree():
                return self.variables()
            if self.name in _Formula.signature.quantifiers:
                return reduce(lambda x, y: x | y, map(lambda arg: arg.freeVariables(), self.arguments), set()) - set(self.boundVariables)
            return reduce(lambda x, y: x | y, map(lambda arg: arg.freeVariables(), self.arguments), set())
                
        def isAtomic(self):
            return self.name in _Formula.signature.atoms

        def isQuantifierFree(self):
            return (self.name not in _Formula.signature.quantifiers) and (all(arg.isQuantifierFree() for arg in self.arguments) if self.name in _Formula.signature.connectors else True)

        def superformula(self, other):
            return self == other or any([arg.superterm(other) for arg in self.arguments])

        def subformula(self, other):
            return other.superterm(self)

        def substitute(self, d):
            if self in d.keys():
                return d[self]
            else:
                return _Formula(name=self.name, arguments=list(map(lambda x: x.substitute(d), self.arguments)))

        def mgu(self, other):
            if self.name == other.name:
                def f(s, x):
                    t = x[0].substitute(s).mgu(x[1].substitute(s))
                    return {**{k : v.substitute(t) for k, v in s.items()}, **t}
                return reduce(f, zip(self.arguments, other.arguments), {})
            return None

    return _Formula
