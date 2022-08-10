

build:

cargo build
cd python
python setup.py build --force
python setup.py develop
python test.py