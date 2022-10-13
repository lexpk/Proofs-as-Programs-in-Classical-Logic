import os

from itertools import chain, combinations

def powerset(iterable):
    "list(powerset([1,2,3])) --> [(), (1,), (2,), (3,), (1,2), (1,3), (2,3), (1,2,3)]"
    s = list(iterable)
    return chain.from_iterable(combinations(s, r) for r in range(len(s)+1))

import tqdm

RESULT_ROOT = "./ILTP-v1.1.2-firstorder-results"
TRANSLATION_ROOT = "./ILTP-v1.1.2-firstorder-translated-embedding-idemp"
PROVERS = ["-embedding", "-embedding-idemp", "-leancop", "-nanocop"]
PROOF = {
    "-embedding" : "Refutation found.",
    "-embedding-idemp" : "Refutation found.",
    "-leancop" : "is an Intuitionistic Theorem",
    "-nanocop" : "is an intuitionistic Theorem"
}
COUNTERMODEL = {
    "-embedding" : "SZS status CounterSatisfiable",
    "-embedding-idemp" : "SZS status CounterSatisfiable",
    "-leancop" : "is an Intuitionistic Non-Theorem",
    "-nanocop" : "is an intuitionistic Non-Theorem"
}

result = {tuple(provers) : 0 for provers in powerset(PROVERS)}

for directory in os.listdir(TRANSLATION_ROOT):
    for translation in tqdm.tqdm(os.listdir(os.path.join(TRANSLATION_ROOT, directory)), desc=f"{directory}"):
        solvedby = []
        for p in PROVERS:
            root = RESULT_ROOT + p
            result_path = os.path.join(os.path.join(root, directory), translation)
            try:
                with open(result_path, "r") as result_text:
                    result_text = result_text.read()
                if PROOF[p] in result_text or COUNTERMODEL[p] in result_text:
                    solvedby.append(p)
            except:
                pass
        result[tuple(solvedby)] += 1

for k, v in result.items():
    print(f"{k}: {v}")