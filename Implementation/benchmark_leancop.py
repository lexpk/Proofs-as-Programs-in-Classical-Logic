import subprocess
import tqdm
import os

LEAN_PATH = "/home/lambdax/Implementation/provers/ileancop/ileancop.sh"
TRANSLATOR = "/home/lambdax/Implementation/ILTP-v1.1.2-firstorder/TPTP2X/tptp2x"

PROBLEM_ROOT = "/home/lambdax/Implementation/ILTP-v1.1.2-firstorder/Problems"
TRANSLATION_ROOT = "/home/lambdax/Implementation/ILTP-v1.1.2-firstorder-translated-leancop"
RESULT_ROOT = "/home/lambdax/Implementation/ILTP-v1.1.2-firstorder-results-leancop"

total_proven = 0
total_disproven = 0
total_unresolved = 0
total_total = 0

if not os.path.isdir("ILTP-v1.1.2-firstorder-translated-leancop"):
    os.mkdir("ILTP-v1.1.2-firstorder-translated-leancop")
if not os.path.isdir("ILTP-v1.1.2-firstorder-results-leancop"):
    os.mkdir("ILTP-v1.1.2-firstorder-results-leancop")
for directory in os.listdir(PROBLEM_ROOT):
    proven = 0
    disproven = 0
    unresolved = 0
    if not os.path.isdir(os.path.join(TRANSLATION_ROOT, directory)):
        os.mkdir(os.path.join(TRANSLATION_ROOT, directory))
    if not os.path.isdir(os.path.join(RESULT_ROOT, directory)):
        os.mkdir(os.path.join(RESULT_ROOT, directory))
    for problem in tqdm.tqdm(os.listdir(os.path.join(PROBLEM_ROOT, directory)), desc=f"{directory}"):
        problem_path = os.path.join(os.path.join(
            PROBLEM_ROOT, directory), problem)      
        translation_path = os.path.join(os.path.join(
            TRANSLATION_ROOT, directory), problem)
        with open(translation_path, "w") as output_file:
            subprocess.run([TRANSLATOR, "-f", "leancop", "-q2", "-t", "stdfof+add_equality", "-d", "-", problem_path], stdout=output_file)
        result_path = os.path.join(os.path.join(
            RESULT_ROOT, directory), problem)
        with open(result_path, "w") as output_file:
            try:
                subprocess.run([LEAN_PATH, translation_path], stdout=output_file, timeout=10)
            except subprocess.TimeoutExpired:
                subprocess.run(["killall", "eclipse.exe"])
                print("timeout", file=output_file)
        with open(result_path, "r") as result:
            result_text = result.read()
        with open(problem_path, "r") as result:
            problem_text = result.read()    
        if "is an Intuitionistic Theorem" in result_text:
            proven += 1
            assert "Status (intuit.) : Non-Theorem" not in problem_text or "Problem negated" in problem_text
        elif "is an Intuitionistic Non-Theorem" in result_text:
            disproven += 1
            assert "Status (intuit.) : Theorem" not in problem_text or "Problem negated" in problem_text
        else:
            unresolved += 1          
            
    total_proven += proven
    total_disproven += disproven
    total_unresolved += unresolved
    total = proven + disproven + unresolved
    total_total += total
    print(
        f"Proven/Disproven/Unresolved:\t{proven}/{disproven}/{unresolved}\t{100*proven/total:.1f}%/{100*disproven/total:.1f}%/{100*unresolved/total:.1f}%")
print("Total Result:")
print(f"Proven/Disproven/Unresolved:\t{total_proven}/{total_disproven}/{total_unresolved}\t{100*total_proven/total_total:.1f}%/{100*total_disproven/total_total:.1f}%/{100*total_unresolved/total_total:.1f}%")
