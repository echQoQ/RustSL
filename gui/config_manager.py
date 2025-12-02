import os
import json


def load_plugins_manifest():
    path = os.path.join('config', 'plugins.json')
    with open(path, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    enc = data.get('encryption') or []
    runm = data.get('run_modes') or []
    vmc = data.get('vm_checks') or []
    alloc_mem_modes = data.get('alloc_mem_modes') or []
    encodings = data.get('encodings') or []
    defaults = data.get('defaults') or {}
    
    if not enc or not runm:
        raise ValueError('plugins.json 缺少必要字段(encryption/run_modes)')
    
    return {
        'encryption': enc,
        'run_modes': runm,
        'vm_checks': vmc,
        'alloc_mem_modes': alloc_mem_modes,
        'encodings': encodings,
        'defaults': defaults,
    }


def get_encryption_map():
    manifest = load_plugins_manifest()
    return {e['id']: e.get('encrypt_arg', e['id']) for e in manifest['encryption']}


def get_vm_checks_map():
    manifest = load_plugins_manifest()
    return {v['id']: v['feature'] for v in manifest['vm_checks']}


def get_encryption_feature_map():
    manifest = load_plugins_manifest()
    return {e['id']: e['feature'] for e in manifest['encryption']}


def get_run_mode_map():
    manifest = load_plugins_manifest()
    return {r['id']: r['feature'] for r in manifest['run_modes']}


def get_alloc_mem_feature_map():
    manifest = load_plugins_manifest()
    return {m['id']: m['feature'] for m in manifest.get('alloc_mem_modes', [])}


def get_encodings():
    manifest = load_plugins_manifest()
    return manifest.get('encodings', [])


def get_encoding_feature_map():
    manifest = load_plugins_manifest()
    return {e['id']: e['feature'] for e in manifest.get('encodings', [])}


def get_default_value(key):
    manifest = load_plugins_manifest()
    return manifest['defaults'].get(key)
