#! /usr/bin/env python

import os


def main():
    here = os.path.abspath(os.getcwd())
    os.symlink(here + '/src/www/uikit-custom', here + '/uikit/custom', True)


if __name__ == '__main__':
    main()
