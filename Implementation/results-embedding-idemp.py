import os

RESULT_ROOT = "./ILTP-v1.1.2-firstorder-results-embedding-idemp/"

total_proven = 0
total_disproven = 0
total_unresolved = 0
total_total = 0

if not os.path.isdir(RESULT_ROOT):
    os.mkdir(RESULT_ROOT)
for directory in os.listdir(RESULT_ROOT):
    proven = 0
    disproven = 0
    unresolved = 0
    for translation in os.listdir(os.path.join(RESULT_ROOT, directory)):
        result_path = os.path.join(os.path.join(
            RESULT_ROOT, directory), translation)
        with open(result_path, "r") as result:
            result_text = result.read()
        if "Refutation found." in result_text:
            proven += 1
        elif "SZS status CounterSatisfiable" in result_text:
            disproven += 1
        else:
            unresolved += 1
    total_proven += proven
    total_disproven += disproven
    total_unresolved += unresolved
    total = proven + disproven + unresolved
    total_total += total
    print(
        f"Category {directory}, Proven/Disproven/Unresolved:\t{proven}/{disproven}/{unresolved}\t{100*proven/total:.1f}%/{100*disproven/total:.1f}%/{100*unresolved/total:.1f}%")
print("Total Result:")
print(f"Proven/Disproven/Unresolved:\t{total_proven}/{total_disproven}/{total_unresolved}\t{100*total_proven/total_total:.1f}%/{100*total_disproven/total_total:.1f}%/{100*total_unresolved/total_total:.1f}%")
