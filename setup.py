#! /usr/bin/env python3

import os


def main():
    print('Setting up uikit')
    os.chdir('client/uikit')
    os.system('git submodule init')
    os.system('git submodule update')
    os.system('yarn install')

    os.chdir('..')
    here = os.path.abspath(os.getcwd())
    src = here + '/src/uikit-custom'
    dst = here + '/uikit/custom'
    if os.path.exists(dst):
        print('\nReplacing old symlink')
        os.remove(dst)
    else:
        print('\nCreating uikit symlink')
    os.symlink(src, dst, True)



if __name__ == '__main__':
    main()
