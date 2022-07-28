#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""The setup script."""

from setuptools import setup, find_packages, Extension

with open('README.rst') as readme_file:
    readme = readme_file.read()

requirements = [
    "cffi>=1.0.0"
]

setup_requirements = [ 
    "cffi>=1.0.0"
]

test_requirements = [ 
]

setup(
    author="Romain Picard",
    author_email='romain.picard@softathome.com',
    classifiers=[
        'Development Status :: 2 - Pre-Alpha',
        'Natural Language :: English',
        'License :: Other/Proprietary License',
        'Intended Audience :: Developers',
        'Programming Language :: Python :: 3',
    ],
    scripts=[
    ],
    description="RxSci in Rust",
    install_requires=requirements,
    long_description=readme,
    #include_package_data=True,
    keywords='',
    name='rrs',
    packages=find_packages(),
    setup_requires=setup_requirements,
    #ext_modules = [sahmfcc],
    cffi_modules=["build_rrs.py:ffibuilder"],
    #test_suite='tests',
    tests_require=test_requirements,
    url='',
    version='0.1.0',
)
