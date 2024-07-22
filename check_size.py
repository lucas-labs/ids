"""
small script to run after build, to check if there was a significant
change on executable size, compared to the previous build.

this aims to detect unwanted big differences before it's too late
"""


import os
import pathlib

curr_dir = pathlib.Path(os.getcwd())
sizefile_path = pathlib.Path(curr_dir.joinpath('.task'))


def bad(txt):
    return '\033[91m' + txt + '\033[0m'


def good(txt):
    return '\033[92m' + txt + '\033[0m'


def head(txt):
    return '\033[94m' + txt + '\033[0m'


files = {
    'release': curr_dir.joinpath('target/release/ids.exe'),
    'debug': curr_dir.joinpath('target/debug/ids.exe'),
}

print("\n🧉 » exe file sizes change\n")

for key, exe in files.items():
    if exe.is_file():
        sizefile = sizefile_path.joinpath(key)
        new_size: float = os.stat(exe).st_size / 1024
        old_size: float

        try:
            with open(sizefile, 'r') as f:
                old_size = float(f.read())
        except FileNotFoundError:
            old_size = 0

        # diff: str = f'{old_size:.0f}kb'
        diff = new_size - old_size
        diff_str = f"{'+' if diff > 0 else '=' if diff == 0 else ''}{diff:.0f}kb"

        fmt = bad if diff > 10 else good

        print(f'{key}: {fmt(diff_str)} (prev: {old_size}kb, now: {new_size}kb)')
        sizefile.parent.mkdir(parents=True, exist_ok=True)

        with open(sizefile, 'w') as f:
            f.write(f'{new_size}')
