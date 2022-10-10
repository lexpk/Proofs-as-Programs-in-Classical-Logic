import subprocess
import tqdm
import os

NANO_PATH = "/home/lambdax/Implementation/provers/nanoCoP-i20/nanocopi.sh"

PROBLEM_ROOT = "./ILTP-v1.1.2-firstorder/Problems/"
RESULT_ROOT = "./ILTP-v1.1.2-firstorder-results-nanocop/"

total_proven = 0
total_disproven = 0
total_unresolved = 0
total_total = 0

if not os.path.isdir(RESULT_ROOT):
    os.mkdir(RESULT_ROOT)
for directory in os.listdir(PROBLEM_ROOT):
    proven = 0
    disproven = 0
    unresolved = 0
    if not os.path.isdir(os.path.join(RESULT_ROOT, directory)):
        os.mkdir(os.path.join(RESULT_ROOT, directory))
    for translation in tqdm.tqdm(os.listdir(os.path.join(PROBLEM_ROOT, directory)), desc=f"{directory}"):
        problem_path = os.path.join(os.path.join(
            PROBLEM_ROOT, directory), translation)
        result_path = os.path.join(os.path.join(
            RESULT_ROOT, directory), translation)
        with open(result_path, "w") as output_file:
            try:
<<<<<<< Updated upstream
                subprocess.run([NANO_PATH, problem_path], stdout=output_file)
            except subprocess.TimeoutExpired:
                subprocess.run(["killall", "swipl"])
=======
                subprocess.run([NANO_PATH, problem_path],
                           timeout=10,
                           stdout=output_file)
            except subprocess.TimeoutExpired:
>>>>>>> Stashed changes
                print("timeout", file=output_file)
        with open(result_path, "r") as result:
            result_text = result.read()
        with open(problem_path, "r") as result:
            problem_text = result.read()    
<<<<<<< Updated upstream
        if "intuitionistic Theorem" in result_text:
=======
        if "is an intuitionistic Theorem" in result_text:
>>>>>>> Stashed changes
            proven += 1
            assert "Status (intuit.) : Non-Theorem" not in problem_text
        elif "timeout" in result_text:
            unresolved += 1
        else:
            disproven += 1
            assert "Status (intuit.) : Theorem" not in problem_text
    total_proven += proven
    total_disproven += disproven
    total_unresolved += unresolved
    total = proven + disproven + unresolved
    total_total += total
    print(
        f"Proven/Disproven/Unresolved:\t{proven}/{disproven}/{unresolved}\t{100*proven/total:.1f}%/{100*disproven/total:.1f}%/{100*unresolved/total:.1f}%")
print("Total Result:")
print(f"Proven/Disproven/Unresolved:\t{total_proven}/{total_disproven}/{total_unresolved}\t{100*total_proven/total_total:.1f}%/{100*total_disproven/total_total:.1f}%/{100*total_unresolved/total_total:.1f}%")
