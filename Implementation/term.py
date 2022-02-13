from dataclasses import dataclass
from functools import reduce
from typing import Dict


class Signature:
    def __init__(self, d: Dict[str, int]):
        self._d = d
        self.constants = {c for c, a in self._d.items() if a == 0}
        self.functions = {c for c, a in self._d.items() if a > 0}
        self.symbols = self.constants | self.functions
        self.arity = lambda x: self._d[x]

    def __repr__(self):
        return "[" + ", ".join([f"{s}/{self.arity(s)}" for s in self.symbols]) + "]"

    def __eq__(self, other) -> bool:
        return self._d == other._d


def Term(sig: Signature):
    class _Term():
        signature = sig

        def __init__(self, name: str, arguments = []):
            assert all(type(self).signature == type(arg).signature for arg in arguments)
            if name in _Term.signature.symbols:
                assert len(arguments) == _Term.signature.arity(name)
                self.name = name
                self.arguments = arguments
            else:
                assert len(arguments) == 0
                self.name = name
                self.arguments = []

        def __repr__(self) -> str:
            return self.name + (("(" + ", ".join(map(str, self.arguments)) + ")") if self.arguments else "")

        def __eq__(self, other) -> bool:
            return self.name == other.name and self.arguments == other.arguments

        def __hash__(self) -> int:
            return hash(repr(self))
        
        def isVariable(self):
            return self.name not in _Term.signature.symbols

        def variables(self) -> set():
            return (set() if self.name in _Term.signature.symbols else set([self])) | reduce(lambda x, y: x | y, map(lambda arg: arg.variables(), self.arguments), set())

        def superterm(self, other):
            return self == other or any([arg.superterm(other) for arg in self.arguments])

        def subterm(self, other):
            return other.superterm(self)

        def substitute(self, d):
            if self in d.keys():
                return d[self]
            else:
                return _Term(name=self.name, arguments=list(map(lambda x: x.substitute(d), self.arguments)))

        def mgu(self, other):
            if self == other:
                return {}
            if other.isVariable():
                if other not in self.variables():   
                    return {other: self}
                else:
                    return None
            if self.isVariable():
                if self not in other.variables():
                    return {self: other}
                else:
                    return None
            if self.name == other.name:
                def f(s, x):
                    t = x[0].substitute(s).mgu(x[1].substitute(s))
                    return {**{k : v.substitute(t) for k, v in s.items()}, **t}
                return reduce(f, zip(self.arguments, other.arguments), {})
            return None

    return _Term
