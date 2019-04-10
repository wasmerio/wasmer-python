import setuptools

with open("README.md", "r") as file:
    long_description = file.read()

setuptools.setup(
    description="Python extension to run WebAssembly binaries",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/wasmerio/python-ext-wasm",
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
)
