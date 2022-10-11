import subprocess
import tqdm
import os

VAMPIRE_PATH = "/home/lambdax/.vampire/vampire"
TRANSLATOR = "./eiicl/target/debug/eiicl"

INCLUDE_ROOT = "./ILTP-v1.1.2-firstorder/"
PROBLEM_ROOT = "./ILTP-v1.1.2-firstorder/Problems/"
TRANSLATION_ROOT = "./ILTP-v1.1.2-firstorder-translated-embedding-idemp/"
RESULT_ROOT = "./ILTP-v1.1.2-firstorder-results-embedding-idemp/"

total_proven = 0
total_disproven = 0
total_unresolved = 0
total_total = 0

if not os.path.isdir(TRANSLATION_ROOT):
    os.mkdir(TRANSLATION_ROOT)
if not os.path.isdir(RESULT_ROOT):
    os.mkdir(RESULT_ROOT)
for directory in os.listdir(PROBLEM_ROOT):
    proven = 0
    disproven = 0
    unresolved = 0
    if not os.path.isdir(os.path.join(TRANSLATION_ROOT, directory)):
        os.mkdir(os.path.join(TRANSLATION_ROOT, directory))
    if not os.path.isdir(os.path.join(RESULT_ROOT, directory)):
        os.mkdir(os.path.join(RESULT_ROOT, directory))
    for translation in tqdm.tqdm(os.listdir(os.path.join(PROBLEM_ROOT, directory)), desc=f"{directory}"):
        problem_path = os.path.join(os.path.join(
            PROBLEM_ROOT, directory), translation)
        translation_path = os.path.join(os.path.join(
            TRANSLATION_ROOT, directory), translation)
        with open(translation_path, "w") as output_file:
            subprocess.run([TRANSLATOR, problem_path,
                           INCLUDE_ROOT], stdout=output_file)
        result_path = os.path.join(os.path.join(
            RESULT_ROOT, directory), translation)
        with open(result_path, "w") as output_file:
            subprocess.run([VAMPIRE_PATH, translation_path,
                           "--time_limit", "10"], stdout=output_file)
        with open(result_path, "r") as result:
            result_text = result.read()
        with open(problem_path, "r") as result:
            problem_text = result.read()    
        if "Refutation found." in result_text:
            proven += 1
            assert "Status (intuit.) : Non-Theorem" not in problem_text
        elif "SZS status CounterSatisfiable" in result_text:
            disproven += 1
            assert "Status (intuit.) : Theorem" not in problem_text
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
