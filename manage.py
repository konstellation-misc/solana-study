import os
from pathlib import Path
import shutil


PARTICIPANTS = ["hyunjin", "lalit", "burhan", "hyun", "beoms"]


def touch(cur, top, participants):
    assert top in ('zero_to_production',)
    dname = f'{top}/'
    for p in participants:
        path = f'{dname}/{p}'
        os.makedirs(path, exist_ok=True)
        Path(f'{path}/i_didnt_study_chapter_{cur}').touch()


if __name__ == '__main__':
    touch('2', 'zero_to_production', PARTICIPANTS)
