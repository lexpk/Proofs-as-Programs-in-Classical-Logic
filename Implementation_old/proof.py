from dataclasses import dataclass
from formula import BFalse, BTrue, Formula, Disjunction, Negation, Atom, Forall, Exists
from typing import Dict, Sequence
from term import Term, Variable

class Proof:
    def parse(d):
        head_id = max(d.keys())
        if d[head_id][0] == 'input':
            return Input([], None, d[head_id][2])
        if d[head_id][0] == 'negated conjecture ':
            return NegatedConjecture([], None, d[head_id][2])
        if d[head_id][0] == 'rectify ':
            return Rectify([Proof.parse({k : d[k] for k in d.keys() if k <= id}) for id in d[head_id][1]], None, d[head_id][2])
        if d[head_id][0] == 'ennf transformation ':
            return EnnfTransformation([Proof.parse({k : d[k] for k in d.keys() if k <= id}) for id in d[head_id][1]], None, d[head_id][2])
        if d[head_id][0] == 'nnf transformation ':
            return NnfTransformation([Proof.parse({k : d[k] for k in d.keys() if k <= id}) for id in d[head_id][1]], None, d[head_id][2])
        if d[head_id][0] == 'cnf transformation ':
            return CnfTransformation([Proof.parse({k : d[k] for k in d.keys() if k <= id}) for id in d[head_id][1]], None, d[head_id][2])
        if d[head_id][0] == 'flattening ':
            return Flattening([Proof.parse({k : d[k] for k in d.keys() if k <= id}) for id in d[head_id][1]], None, d[head_id][2])
        if d[head_id][0] == 'choice axiom':
            return ChoiceAxiom([], None, d[head_id][2])
        if d[head_id][0] == 'equality resolution ':
            return EqualityResolution([Proof.parse({k : d[k] for k in d.keys() if k <= id}) for id in d[head_id][1]], None, d[head_id][2])
        if d[head_id][0] == 'skolemisation ':
            return Skoemisation([Proof.parse({k : d[k] for k in d.keys() if k <= id}) for id in d[head_id][1]], None, d[head_id][2])
        if d[head_id][0] == 'superposition ':
            return Superposition([Proof.parse({k : d[k] for k in d.keys() if k <= id}) for id in d[head_id][1]], None, d[head_id][2])
        if d[head_id][0] == 'forward demodulation ':
            return ForwardDemodulation([Proof.parse({k : d[k] for k in d.keys() if k <= id}) for id in d[head_id][1]], None, d[head_id][2])
        if d[head_id][0] == 'resolution ':
            temp = Resolution([Proof.parse({k : d[k] for k in d.keys() if k <= id}) for id in d[head_id][1]], None, None)
            return Resolution([Proof.parse({k : d[k] for k in d.keys() if k <= id}) for id in d[head_id][1]], temp.compute_mgu(), d[head_id][2])

    def variables(self):
        result = self.head.variables()
        for arg in self.arguments:    
            result.update(arg.variables())
        return result

    def compute_mgus(self):
        if type(self) not in [Resolution, ForwardDemodulation, Superposition]:
            return self
        else:
            for arg in self.arguments:
                arg.compute_mgus()
            self.compute_mgu()
            
    
    def make_unique_variables(self):
        if type(self) in [Superposition, ForwardDemodulation, Resolution]:
            args = [arg.make_unique_variables() for arg in self.arguments]
            for i, arg in enumerate(args):
                d = {var : Variable(var.name + str(i)) for var in arg.variables()}
                args[i] = arg.substitute(d)
            if type(self) == Resolution:
                d = Resolution(args, None, None).compute_mgu()
                if type(self.head) in [BFalse, BTrue]:
                    f = self.head
                if type(self.head) in [Atom, Negation]:
                    i = 0 if type(args[0].head) == Disjunction else 1
                    f = args[i].head.arguments[1].substitute(d[i])
                if type(self.head) == Disjunction:
                    formulas = []
                    for i, arg in enumerate(args):
                        print(arg.head)  
                        if type(arg.head) == Disjunction:
                            print('Here')
                            formulas += arg.head.substitute(d[i]).arguments[1:]
                return Resolution(args, d, f)
        else:
            return self
  
    def substitute(self, d):
        if self.mgu:
            m = [{k.substitute(d): v.substitute(d) for k, v in self.mgu[i].items()} for i in [0, 1]]
            return type(self)(arguments=[arg.substitute(d) for arg in self.arguments], mgu=m, head=self.head.substitute(d))
        else:
            return type(self)(arguments=[arg.substitute(d) for arg in self.arguments], mgu=None, head=self.head.substitute(d))


    def step1(self):
        return self._step1({})

    def _step1(self, d):
        if self.mgu:
            _arguments = [arg._step1(_d) for _d, arg in zip(self.mgu, self.arguments)]
            return type(self)(arguments=[arg._step1(d) for arg in _arguments], mgu=self.mgu ,head=self.head.substitute(d))
        else:
            return type(self)(arguments=[arg._step1(d) for arg in self.arguments], mgu=None, head=self.head.substitute(d))

    def _str(self, counter = 1):
        result = f'{counter}. ' + str(self.head) + f' [{self.__class__.__name__}'
        counter += 1
        partial = ''
        for arg in self.arguments:
            result += f' {counter}'
            _partial, counter = arg._str(counter = counter)
            partial += _partial
        result += f']'
        if self.mgu:
            result += str({str(k) + ' -> ' + str(v) for k, v in self.mgu[0].items()}) + str({str(k) + ' -> ' + str(v) for k, v in self.mgu[1].items()})
        result += '\n' + partial
        return result, counter

    def step2(self):
        return self._step2()[1]

    def _step2(self):
        if type(self) == NegatedConjecture:
            return True, None
        else:
            for i, arg in enumerate(self.arguments):
                conjecture, newArg = arg._step2()
                if conjecture:
                    if type(self.head) == Negation and type(self.arguments[0].head) in [Forall, Exists]:
                        return self.head.arguments[0], Input(
                            arguments=[],
                            mgu=None,
                            head=Disjunction(arguments=[self.head, self.head.arguments[0]])
                        )
                    if type(self.head) == Disjunction:
                        return conjecture, type(self)(
                            arguments=[newArg if i == j else _arg for j, _arg in enumerate(self.arguments)],
                            mgu=self.mgu,
                            head=Disjunction(arguments=self.head.arguments + [conjecture])
                        )
                    if type(self.head) == BFalse:
                        return conjecture, type(self)(
                            arguments=[newArg if i == j else _arg for j, _arg in enumerate(self.arguments)],
                            mgu=self.mgu,
                            head=conjecture
                        )
                    else:
                        return conjecture, type(self)(
                            arguments=[newArg if i == j else _arg for j, _arg in enumerate(self.arguments)],
                            mgu=self.mgu,
                            head=Disjunction(arguments=[self.head, conjecture])
                        )           
            else:
                return None, self

    def __str__(self):
        return self._str()[0]



@dataclass
class Input(Proof):
    arguments : Sequence[Proof]
    mgu : Sequence[Dict[Variable, Term]]
    head : Formula

@dataclass
class NegatedConjecture(Proof):
    arguments : Sequence[Proof]
    mgu : Sequence[Dict[Variable, Term]]
    head : Formula

@dataclass
class Rectify(Proof):
    arguments : Sequence[Proof]
    mgu : Sequence[Dict[Variable, Term]]
    head : Formula

@dataclass
class EnnfTransformation(Proof):
    arguments : Sequence[Proof]
    mgu : Sequence[Dict[Variable, Term]]
    head : Formula
    
@dataclass
class NnfTransformation(Proof):
    arguments : Sequence[Proof]
    mgu : Sequence[Dict[Variable, Term]]
    head : Formula

@dataclass
class CnfTransformation(Proof):
    arguments : Sequence[Proof]
    mgu : Sequence[Dict[Variable, Term]]
    head : Formula

@dataclass
class Flattening(Proof):
    arguments : Sequence[Proof]
    mgu : Sequence[Dict[Variable, Term]]
    head : Formula

@dataclass
class ChoiceAxiom(Proof):
    arguments : Sequence[Proof]
    mgu : Sequence[Dict[Variable, Term]]
    head : Formula

@dataclass
class Skoemisation(Proof):
    arguments : Sequence[Proof]
    mgu : Sequence[Dict[Variable, Term]]
    head : Formula

@dataclass
class EqualityResolution(Proof):
    arguments : Sequence[Proof]
    mgu : Sequence[Dict[Variable, Term]]
    head : Formula

@dataclass
class Superposition(Proof):
    arguments : Sequence[Proof]
    mgu : Sequence[Dict[Variable, Term]]
    head : Formula

@dataclass
class ForwardDemodulation(Proof):
    arguments : Sequence[Proof]
    mgu : Sequence[Dict[Variable, Term]]
    head : Formula

@dataclass
class Resolution(Proof):
    arguments : Sequence[Proof]
    mgu : Sequence[Dict[Variable, Term]]
    head : Formula

    def compute_mgu(self):
        clauses = []
        for arg in self.arguments:
            if type(arg.head) == Disjunction:
                clauses.append(arg.head.arguments[0])
            else:
                clauses.append(arg.head)
        for i, c in enumerate(clauses):
            if type(c) == Negation:
                clauses[i] = c.arguments[0]
        s = [{}, {}]
        for arg1, arg2 in zip(clauses[0].arguments, clauses[1].arguments):
            arg1, arg2 = arg1.substitute(s[0]), arg2.substitute(s[1])
            si = arg1.mgu(arg2)
            for i in [0, 1]:
                if s[i]:
                    s[i].update(si[i])
                else:
                    s[i] = si[i]
        self.mgu = s
        return s
