#! /usr/bin/env python

import os


def main():
    print('Setting up uikit')
    os.chdir('uikit')
    os.system('git submodule init')
    os.system('git submodule update')
    os.system('yarn install')

    os.chdir('..')
    here = os.path.abspath(os.getcwd())
    print('\nCreating uikit symlink')
    os.symlink(here + '/src/www/uikit-custom', here + '/uikit/custom', True)



if __name__ == '__main__':
    main()
