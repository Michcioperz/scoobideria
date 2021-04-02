from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="scoobideria",
    version="0.1.0",
    rust_extensions=[RustExtension("scoobideria.dice", binding=Binding.PyO3)],
    packages=["scoobideria"],
    install_requires=["python-telegram-bot>=13.3,<14.0"],
    entry_points={"console_scripts": ["scoobideria = scoobideria.main:main"]},
    zip_safe=False,
)
