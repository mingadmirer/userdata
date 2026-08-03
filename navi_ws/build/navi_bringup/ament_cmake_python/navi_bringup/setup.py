from setuptools import find_packages
from setuptools import setup

setup(
    name='navi_bringup',
    version='0.1.0',
    packages=find_packages(
        include=('navi_bringup', 'navi_bringup.*')),
)
