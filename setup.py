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
    print('\nCreating uikit symlink')
    os.symlink(here + '/src/uikit-custom', here + '/uikit/custom', True)



if __name__ == '__main__':
    main()
