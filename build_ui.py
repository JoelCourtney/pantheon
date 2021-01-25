#! /usr/bin/env python3

import os
import shutil


def main():
    os.chdir('src/www')
    here = os.path.abspath(os.getcwd())
    src = here + '/uikit-custom'
    dst = here + '/uikit/custom'
    if os.path.exists(dst):
        os.remove(dst)
    os.symlink(src, dst, True)

    os.chdir('uikit')
    os.system('yarn compile')
    os.chdir('..')

    if not os.path.exists('css'):
        os.makedirs('css')

    shutil.copyfile('uikit/dist/css/uikit.dndcent-theme.min.css', 'css/style.css')
    shutil.copyfile('uikit/dist/js/uikit.min.js', 'scripts/uikit.min.js')
    shutil.copyfile('uikit/dist/js/uikit-icons.min.js', 'scripts/uikit-icons.min.js')


if __name__ == '__main__':
    main()
