import urllib.request
import os
import subprocess

ORIGINAL_CODE_LINK = 'https://drive.usercontent.google.com/download?id=1m9SCro-h3Wff0t4AGSE1yYsiOdUgIyoZ&export=download&confirm=t&uuid=84fb0ce4-b216-4742-b29f-fab452e12781'
CODE_FOLDER = "./tmp/cppcode"
WEIGHTS_BIN_PATH = os.path.join(CODE_FOLDER, "data/weights.bin")
GRAPH_BIN_PATH = os.path.join(CODE_FOLDER, "data/graph.bin")
OUTPUT_FILE = os.path.join(CODE_FOLDER, "data/output.txt")


def fix_missing_import_in_original_louvain_code():
    FILENAME = os.path.join(CODE_FOLDER, "main_community.cpp")

    with open(FILENAME, 'r') as f:
        lines = f.readlines()
        lines.insert(0, '#include <unistd.h>    /* for getpid */')

    with open(FILENAME, 'w') as f:
        f.writelines(lines)


def download_and_build_original_code():
    if os.path.exists(CODE_FOLDER):
        print(
            f"▶ Folder {CODE_FOLDER} already exists. Skipping download & build.")
        return

    print('▶ Downloading and building original cpp code')
    os.makedirs(CODE_FOLDER, exist_ok=True)
    ZIP_FILE = os.path.join(CODE_FOLDER, 'cppcode.tgz')
    urllib.request.urlretrieve(ORIGINAL_CODE_LINK, os.path.join(ZIP_FILE))
    os.system(f"tar -xzf {ZIP_FILE} --strip-components=1 -C {CODE_FOLDER}")
    fix_missing_import_in_original_louvain_code()
    os.system(f"make -C {CODE_FOLDER}")
    os.makedirs(os.path.join(CODE_FOLDER, 'data'), exist_ok=True)


def run_original_code(graph_file):
    # Convert graph to weights file and binary format
    convert_command = f'{os.path.join(CODE_FOLDER, "convert")}\
        -i {graph_file} -w {WEIGHTS_BIN_PATH} -o {GRAPH_BIN_PATH}'
    subprocess.run(convert_command, shell=True)

    # Run
    print(f"▶ Running original cpp code on {graph_file}")
    run_command = f'{os.path.join(CODE_FOLDER, "community")} {GRAPH_BIN_PATH}\
        -l -1 -v -w {WEIGHTS_BIN_PATH} > {OUTPUT_FILE}'
    subprocess.run(run_command, shell=True)

    # Output
    print(f'▶ Final community assignment from original cpp code')
    with open(OUTPUT_FILE, 'r') as f:
        for line in f.readlines():
            print(line, end='')

    print(f'⚠ You can revisit the output here: {OUTPUT_FILE}')


if __name__ == '__main__':
    TEST_GRAPH_FILE = './tests/graphs/weighted_graph_1.txt'
    download_and_build_original_code()
    run_original_code(TEST_GRAPH_FILE)
