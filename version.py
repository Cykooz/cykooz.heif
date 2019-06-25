# -*- coding: utf-8 -*-
"""Calculates the current version number.

If possible, uses output of “git describe” modified to conform to the
visioning scheme that setuptools uses (see PEP 440).  Releases must be
labelled with annotated tags (signed tags are annotated) of the following
format:

   v<num>(.<num>)+[{a|b|c|rc}<num>(.<num>)*]

If “git describe” returns an error (likely because we're in an unpacked copy
of a release tarball, rather than a git working copy), or returns a tag that
does not match the above format, version is read from RELEASE-VERSION file.

To use this script, simply import it your setup.py file, and use the results
of getVersion() as your package version:

    import version
    setup(
        version=version.get_version(),
        .
        .
        .
    )

This will automatically update the RELEASE-VERSION file.  The RELEASE-VERSION
file should *not* be checked into git but it *should* be included in sdist
tarballs (as should version.py file).  To do this, run:

    echo include RELEASE-VERSION version.py >>MANIFEST.in
    echo RELEASE-VERSION >>.gitignore

With that setup, a new release can be labelled by simply invoking:

    git tag -a v1.0
"""
from __future__ import print_function

import re
import subprocess
import sys
from distutils.version import StrictVersion

__author__ = ('Douglas Creager <dcreager@dcreager.net>',
              'Michal Nazarewicz <mina86@mina86.com>',
              'Kirill Kuzminykh <cykooz@gmail.com')
__license__ = 'This file is placed into the public domain.'
__maintainer__ = 'Kirill Kuzminykh'
__email__ = 'cykooz@gmail.com'

__all__ = ('get_version',)


CHANGES_FILE = 'CHANGES.rst'
RELEASE_VERSION_FILE = 'RELEASE-VERSION'

# http://legacy.python.org/dev/peps/pep-0440/
_PEP440_SHORT_VERSION_RE = r'\d+(?:\.\d+)+(?:(?:[abc]|rc)\d+(?:\.\d+)*)?'
_PEP440_VERSION_RE = r'^%s(?:\.post\d+)?(?:\.dev\d+)?$' % _PEP440_SHORT_VERSION_RE
_GIT_DESCRIPTION_RE = r'^v(?P<ver>%s)-(?P<commits>\d+)-g(?P<sha>[\da-f]+)$' % _PEP440_SHORT_VERSION_RE


def read_git_version():
    try:
        proc = subprocess.Popen(('git', 'describe', '--long',
                                 '--match', 'v[0-9]*.*'),
                                stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        data, _ = proc.communicate()
        if proc.returncode:
            return None, None
        ver = data.decode('utf-8').splitlines()[0].strip()
    except:
        return None, None

    if not ver:
        return None, None
    m = re.search(_GIT_DESCRIPTION_RE, ver)
    if not m:
        sys.stderr.write('version: git description (%s) is invalid, '
                         'ignoring\n' % ver)
        return None, None

    commits = int(m.group('commits'))
    ver = StrictVersion(m.group('ver'))
    if not commits:
        return ver, commits
    else:
        if len(ver.version) == 2:
            ver.version += (0,)
        ver.version = (ver.version[0], ver.version[1], ver.version[2] + commits)
        return ver, commits


def read_release_version():
    try:
        fd = open(RELEASE_VERSION_FILE)
        try:
            ver = fd.readline().strip()
        finally:
            fd.close()
        if not re.search(_PEP440_VERSION_RE, ver):
            sys.stderr.write('version: release version (%s) is invalid, '
                             'will use it anyway\n' % ver)
        return StrictVersion(ver)
    except:
        return None


def write_release_version(version):
    fd = open(RELEASE_VERSION_FILE, 'w')
    fd.write('%s\n' % version)
    fd.close()


def get_version():
    release_version = read_release_version()
    version = read_git_version()[0] or release_version
    if not version:
        print('ERROR: Cannot find the version number for "mountbit"')
        version = StrictVersion('0.0')
    if not release_version or version != release_version:
        write_release_version(version)
    return str(version)


def get_version_for_new_release():
    version, commits = read_git_version()
    if not version:
        raise ValueError('Cannot find the version number')
    return version, commits


def main():
    import argparse
    import datetime
    parser = argparse.ArgumentParser(description='Run periodic tasks for users')
    parser.add_argument('-u', dest='version',
                        help='replace title "Next release" in CHANGES.txt on given version number '
                             'and current date. You can use "auto" as version number for generate '
                             'version number automatically')
    args = parser.parse_args()
    if not args.version:
        print(get_version())
        return

    try:
        version, commits = get_version_for_new_release()
        if commits == 0:
            print(version)
            return
    except ValueError:
        if args.version == 'auto':
            raise
        version = args.version

    with open(CHANGES_FILE, 'rt') as f:
        changes = f.read()
    find = 'Next release\n============\n'
    if find in changes:
        if args.version == 'auto':
            # Increase the patch number to account for the additional commit.
            version.version = (version.version[0], version.version[1], version.version[2] + 1)
        else:
            version = StrictVersion(args.version)
        replace = '%s (%s)' % (version, datetime.date.today().strftime('%Y-%m-%d'))
        replace = '%s\n%s\n' % (replace, '=' * len(replace))
        new_changes = changes.replace(find, replace)
        if new_changes != changes:
            with open(CHANGES_FILE, 'wt') as f:
                f.write(new_changes)
            print(version)


if __name__ == '__main__':
    main()
