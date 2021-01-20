#! /usr/bin/env python3

import os
import binascii


def main():
    print('Setting up secret key')
    key = str(binascii.b2a_base64(os.urandom(32)))
    template = open('src/Rocket.toml.template', 'r').read()
    output = template.replace('%%%SECRET%%%', key[2:-3])
    open('Rocket.toml', 'w').write(output)

    print('Setting up uikit')
    os.chdir('src/www/uikit')
    os.system('git submodule init')
    os.system('git submodule update')
    os.system('yarn install')

    os.chdir('..')
    here = os.path.abspath(os.getcwd())
    src = here + '/uikit-custom'
    dst = here + '/uikit/custom'
    if os.path.exists(dst):
        print('\nReplacing old symlink:')
        os.remove(dst)
    else:
        print('\nCreating uikit symlink:')
    print(src + " -> " + dst)
    os.symlink(src, dst, True)


if __name__ == '__main__':
    main()
