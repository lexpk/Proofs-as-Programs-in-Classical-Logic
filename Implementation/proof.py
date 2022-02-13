from typing import Any, Dict
from formula import Signature, Formula

class Logic:
    def __init__(self, sig : Signature, rules : Dict[str, Any]):
        self.signature = Signature
        self.rules = rules

    def addAxiom(self, name, ax):
        self.rules[name] = lambda : ax

def Proof(log: Logic):
    class _Proof:
        logic = log
        
        def __init__(self, name: str, arguments = [], parameters = []):
            self.name = name
            self.arguments = arguments
            self.parameters = parameters
            self.head = self.logic.rules[name](**arguments, **parameters)

        def __repr__(self) -> str:
            return self.name + (("(" + ", ".join(map(str, self.arguments)) + ")") if self.arguments else "")

        def __eq__(self, other) -> bool:
            return self.name == other.name and self.arguments == other.arguments

        def __hash__(self) -> int:
            return hash(repr(self))

    return _Proof
