from setuptools import setup
from setuptools.dist import Distribution

# with open('../README.md', 'rb') as f:
#     readme = f.read().decode('utf-8')

setup(
    name='wasmer',
    version='0.3.0',
    author='The Wasmer Engineering Team',
    author_email='engineering@wasmer.io',
    license='MIT',
    packages=['wasmer'],
    description='Python extension to run WebAssembly binaries',
    # long_description=readme,
    # long_description_content_type='text/markdown',
    zip_safe=False,
    platforms='any',
    classifiers=[
        "Programming Language :: Python",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
)
