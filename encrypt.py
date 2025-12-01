
import os
import base64
import argparse
import importlib.util
import sys
import binascii
from pathlib import Path


def read_binary_file(file_path):
    try:
        with open(file_path, 'rb') as f:
            return f.read()
    except FileNotFoundError:
        print(f"Error: The file {file_path} does not exist.")
        return None


def save(file_path, b64_data):
    with open(file_path, 'wb') as f:
        f.write(b64_data)


PLUGIN_DIR = Path(__file__).parent / 'encrypt_lib'


class PluginLoadError(Exception):
    pass


def load_plugins(plugin_dir=PLUGIN_DIR):
    plugins = []
    if not plugin_dir.exists():
        return plugins
    for p in plugin_dir.iterdir():
        if p.name.startswith('_') or not p.suffix == '.py':
            continue
        name = p.stem
        spec = importlib.util.spec_from_file_location(f"encrypt_plugins.{name}", str(p))
        if spec is None:
            continue
        module = importlib.util.module_from_spec(spec)
        try:
            spec.loader.exec_module(module)
        except Exception as e:
            print(f"Failed loading plugin {name}: {e}")
            continue
        # module must expose `name` and `process(data, args)` or a `Plugin` class
        if hasattr(module, 'Plugin'):
            try:
                inst = module.Plugin()
                plugins.append(inst)
            except Exception as e:
                print(f"Failed instantiating Plugin in {name}: {e}")
        elif hasattr(module, 'name') and hasattr(module, 'process'):
            plugins.append(module)
    return plugins


def build_base_parser(plugin_names):
    p = argparse.ArgumentParser(description="Encrypt binary to new payload format (plugin-based)")
    p.add_argument("-i", "--input", default="calc.bin", help="input binary file (default calc.bin)")
    p.add_argument("-o", "--output", default="src/encrypt.bin", help="output encoded file (default src/encrypt.bin)")
    default = plugin_names[0] if plugin_names else None
    p.add_argument("-m", "--method", default=default, choices=plugin_names,
                   help="encryption method / plugin to use")
    p.add_argument("-e", "--encode", default="base64", choices=["base64", "base32", "hex", "urlsafe_base64", "none"],
                   help="encoding method for output (default base64)")
    return p


def main():
    plugins = load_plugins()
    plugin_names = [getattr(p, 'name', None) for p in plugins]
    plugin_names = [n for n in plugin_names if n]

    parser = build_base_parser(plugin_names)
    # parse initial to know selected plugin
    args, _ = parser.parse_known_args()
    if args.method is None:
        parser.print_help()
        raise SystemExit(1)
    # find plugin object
    selected = None
    for p in plugins:
        if getattr(p, 'name', None) == args.method:
            selected = p
            break
    if selected is None:
        raise SystemExit(f"Unsupported --method: {args.method}")
    # let plugin add arguments if needed
    if hasattr(selected, 'add_arguments') and callable(selected.add_arguments):
        selected.add_arguments(parser)
    # final parse
    args = parser.parse_args()

    data = read_binary_file(args.input)
    if data is None:
        return

    # call plugin to produce final bytes (pre-encode)
    if hasattr(selected, 'process') and callable(selected.process):
        final = selected.process(data, args)
    elif hasattr(selected, 'process_data') and callable(selected.process_data):
        final = selected.process_data(data, args)
    else:
        raise SystemExit(f"Plugin for {args.method} does not expose a process function")

    # Apply encoding based on --encode option
    if args.encode == "base64":
        final = base64.b64encode(final)
    elif args.encode == "base32":
        final = base64.b32encode(final)
    elif args.encode == "hex":
        final = binascii.hexlify(final)
    elif args.encode == "urlsafe_base64":
        final = base64.urlsafe_b64encode(final)
    elif args.encode == "none":
        pass  # final remains as bytes

    save(args.output, final)
    print(f"Encrypted data (method={args.method}, encode={args.encode}) saved to {args.output}")


if __name__ == '__main__':
    main()

