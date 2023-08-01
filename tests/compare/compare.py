import urllib.request
import os
import subprocess
from pathlib import Path

ORIGINAL_CODE_LINK = 'https://drive.usercontent.google.com/download?id=1m9SCro-h3Wff0t4AGSE1yYsiOdUgIyoZ&export=download'

CODE_FOLDER = "./tmp/cppcode"
DATA_FOLDER = "./tmp/data"
WEIGHTS_BIN_PATH = os.path.join(DATA_FOLDER, "weights.bin")
GRAPH_BIN_PATH = os.path.join(DATA_FOLDER, "graph.bin")
CPP_OUTPUT = os.path.join(DATA_FOLDER, "cpp_hierarchy.txt")
RUST_OUTPUT = os.path.join(DATA_FOLDER, "rust_assignment.csv")
RUST_OUTPUT_HIERARCHY = os.path.join(DATA_FOLDER, "rust_hierarchy.tmp")


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


def convert_csv_graph_to_original_louvain_graph(graph_file: str) -> (str, bool):
    cppgraph_name = Path(graph_file).stem + '_cppready.txt'
    cppgraph_path = os.path.join(DATA_FOLDER, cppgraph_name)

    has_weights = False
    with open(cppgraph_path, 'w') as out, open(graph_file, 'r') as csv:
        # Header
        header = csv.readline()
        assert header.strip().lower() in ('source,target,weight', 'source,target')

        for line in csv:
            # Skip empty lines
            if line.strip() == '':
                continue

            parts = line.strip().split(',')
            if len(parts) > 2:
                has_weights = True
                out.write(f'{parts[0]} {parts[1]} {parts[2]}')
            else:
                if has_weights:
                    raise ValueError('Some edges have weights, some have not. '
                                     'Fix this in the original CSV graph file. '
                                     'There might also be a trailing comma that '
                                     'is causing this issue.')
                out.write(f'{parts[0]} {parts[1]}')
            out.write('\n')

    return cppgraph_path, has_weights


def run_original_code(graph_file):
    # Convert graph file format
    print(f'▶ Convert graph file to original cpp code format')
    cppgraph_path, has_weights = convert_csv_graph_to_original_louvain_graph(
        graph_file)
    print(f'Graph file has {"" if has_weights else "NO"} weights')

    # Convert graph file format internally (original code)
    print('▶ Converting graph to weights file and binary format')
    weights_arg = f'-w {WEIGHTS_BIN_PATH}' if has_weights else ''
    convert_command = (f'{os.path.join(CODE_FOLDER, "convert")} '
                       f'-i {cppgraph_path} {weights_arg}-o {GRAPH_BIN_PATH}')
    print(convert_command)
    subprocess.run(convert_command, shell=True)

    # Run
    print(f"▶ Running original cpp code on {graph_file}")
    run_command = (f'{os.path.join(CODE_FOLDER, "community")} {GRAPH_BIN_PATH} '
                   f'-l -1 -v -w {WEIGHTS_BIN_PATH} > {CPP_OUTPUT}')
    print(run_command)
    subprocess.run(run_command, shell=True)

    # Output
    print(f'▶ Final community assignment from original cpp code')
    print(f'⚠ See this file: {CPP_OUTPUT}')


def run_rust_code(graph_file):
    print('▶ Running rust code')
    run_command = f'cargo run --release community {graph_file} -s {RUST_OUTPUT} -h {RUST_OUTPUT_HIERARCHY}'
    print(run_command)
    subprocess.run(run_command, shell=True)


if __name__ == '__main__':
    TEST_GRAPH_FILE = './tests/graphs/original_paper_graph.csv'
    download_and_build_original_code()

    os.makedirs(DATA_FOLDER, exist_ok=True)
    run_original_code(TEST_GRAPH_FILE)

    run_rust_code(TEST_GRAPH_FILE)
