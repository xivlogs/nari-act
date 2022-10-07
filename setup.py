from setuptools import setup, find_namespace_packages

with open('README.md', 'r', encoding='utf-8') as f:
    long_description = f.read()

dev_requirements = [
    'mypy==0.942',
    'pylint==2.13.4',
]

docs_requirements = [
    'pdoc3',
]

version = '0.1.0'

try:
    from setuptools_rust import Binding, RustExtension

    extra_opts = {'rust_extensions': [RustExtension("nari_act_rust", debug=False, quiet=True, binding=Binding.PyO3)]}
except ImportError:
    extra_opts = {}

setup_opts = {
    'name': 'nari-act',
    'version': version,
    'author': 'Nonowazu',
    'author_email': 'oowazu.nonowazu@gmail.com',
    'description': 'ACT-specific additions for nari',
    'long_description': long_description,
    'long_description_content_type': 'text/markdown',
    'python_requires': '>=3.10',
    'packages': find_namespace_packages(include=['nari.ext.*']),
    'package_data': {'nari.ext.act': ['py.typed']},
    'extras_require': {
        'dev': dev_requirements,
        'docs': docs_requirements,
    }
} | extra_opts

setup(**setup_opts)
