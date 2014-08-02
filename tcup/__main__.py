import argparse
import importlib
import sys


def main(argv):
    p = argparse.ArgumentParser()
    p.add_argument('command')

    ns, remaining_argv = p.parse_known_args(argv)
    module = importlib.import_module('tcup.command.' + ns.command)
    return module.run(ns, remaining_argv)


sys.exit(main(sys.argv[1:]) or 0)
