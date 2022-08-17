import os

RESULT_ROOT = "./ILTP-v1.1.2-firstorder-results/"

k = 0
i = 0
print("\\begin{minipage}{0.45\\textwidth}\n\t\\[\\begin{matrix}\n\t\t\\text{Problem}&\\text{Result}&\\text{Time}\\\\")
if not os.path.isdir(RESULT_ROOT):
    os.mkdir(RESULT_ROOT)
for directory in os.listdir(RESULT_ROOT):
    for result in os.listdir(os.path.join(RESULT_ROOT, directory)):
        result_path = os.path.join(os.path.join(RESULT_ROOT, directory), result)
        with open(result_path, "r") as result:
            result_text = result.read()
        if "Refutation found." in result_text:
            print(f"\t\t\\text{{{result.name.split('/')[-1]}}}&\\text{{Proven}}&{result_text.split('Time elapsed:')[1].splitlines()[0]}\\\\")
            i += 1
            if (k < 2 and i == 32) or (k >= 1 and i == 48):
                print("\t\\end{matrix}\\]\n\\end{minipage}")
                if k % 2 == 1:
                    print("\n\\pagebreak\n")
                k = k + 1
                i = 0
                print("\\begin{minipage}{0.45\\textwidth}\n\t\\[\\begin{matrix}\n\t\t\\text{Problem}&\\text{Result}&\\text{Time}\\\\")
        if "SZS status CounterSatisfiable" in result_text:
            print(f"\t\t\\text{{{result.name.split('/')[-1]}}}&\\text{{Refuted}}&{result_text.split('Time elapsed:')[1].splitlines()[0]}\\\\")
            i += 1
            if (k < 2 and i == 32) or (k >= 1 and i == 48):
                print("\t\\end{matrix}\\]\n\\end{minipage}")
                if k % 2 == 1:
                    print("\n\\pagebreak\n")
                k = k + 1
                i = 0
                print("\\begin{minipage}{0.45\\textwidth}\n\t\\[\\begin{matrix}\n\t\t\\text{Problem}&\\text{Result}&\\text{time}\\\\")
        

if i!=38:
    print("\t\\end{matrix}\\]\n\\end{minipage}")
