from setuptools import setup
from setuptools.dist import Distribution

setup(
    name='wasmer',
    version='0.4.1',
    author='The Wasmer Engineering Team',
    author_email='engineering@wasmer.io',
    license='MIT',
    packages=['wasmer'],
    description='Python extension to run WebAssembly binaries',
    zip_safe=False,
    platforms='any',
    classifiers=[
        "Programming Language :: Python",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
)
