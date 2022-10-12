import os

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

buckets = {0 : [0, 2, 5, 10, 20, 1000], 2 : [0, 2, 5, 10, 1000], 5 : [0, 2, 5, 10, 1000], 1000 : [2, 1000] }
tally = {}

for directory in os.listdir(TRANSLATION_ROOT):
    for translation in tqdm.tqdm(os.listdir(os.path.join(TRANSLATION_ROOT, directory)), desc=f"{directory}"):
        translation_path = os.path.join(os.path.join(
            TRANSLATION_ROOT, directory), translation)
        with open(translation_path, "r") as translation_text:
            try:
                translation_text = translation_text.read()
                q = translation_text.count("a,axiom,'f_!")
                x = (translation_text.count("a,axiom,'f") - q)//2
            except MemoryError:
                q = 100
                x = 100
        i = min([b for b in buckets.keys() if b >= q])
        j = min([b for b in buckets[i] if b >= x])
        if not (i, j) in tally:
            tally[(i, j)] = {p : 0 for p in PROVERS + ["total"]}
        tally[(i, j)]["total"] += 1
        for p in PROVERS:
            root = RESULT_ROOT + p
            result_path = os.path.join(os.path.join(root, directory), translation)
            try:
                with open(result_path, "r") as result_text:
                    result_text = result_text.read()
                if PROOF[p] in result_text or COUNTERMODEL[p] in result_text:
                    tally[(i, j)][p] += 1
            except FileNotFoundError:
                pass
for k, v in tally.items():
    print(f"{k}: {v}")
for k, v in tally.items():
    print(f"{k}: {dict((l, v/tally[k]['total']) for (l, v) in tally[k].items())}")