#! /usr/bin/env python3

import os
import shutil


def main():
    here = os.path.abspath(os.getcwd())
    src = here + '/uikit-custom'
    dst = here + '/uikit/custom'
    if os.path.exists(dst):
        os.remove(dst)
    os.symlink(src, dst, True)

    os.chdir('uikit')
    os.system('yarn compile')
    os.chdir('..')

    if not os.path.exists('public/scripts'):
        os.makedirs('public/scripts')

    shutil.copyfile('uikit/dist/css/uikit.dndcent-theme.min.css', 'public/dndcent.min.css')
    shutil.copyfile('uikit/dist/js/uikit.min.js', 'public/scripts/uikit.min.js')
    shutil.copyfile('uikit/dist/js/uikit-icons.min.js', 'public/scripts/uikit-icons.min.js')


if __name__ == '__main__':
    main()
