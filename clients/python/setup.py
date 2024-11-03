from setuptools import setup, find_packages

setup(
    name="ekodb-client-python",
    packages=find_packages(),
    install_requires=[
        "requests>=2.31.0",
        "pydantic>=2.0.0",
        "typing-extensions>=4.0.0",
    ],
    python_requires=">=3.8",
)