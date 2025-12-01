"""
配置管理模块
负责加载和管理 plugins.json 配置文件
"""
import os
import json


def load_plugins_manifest():
    """
    加载插件配置清单
    返回包含加密方式、运行模式、VM检测、内存分配方式等配置的字典
    """
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
    """获取加密方式的 ID 到参数的映射"""
    manifest = load_plugins_manifest()
    return {e['id']: e.get('encrypt_arg', e['id']) for e in manifest['encryption']}


def get_vm_checks_map():
    """获取 VM 检测的 ID 到 feature 的映射"""
    manifest = load_plugins_manifest()
    return {v['id']: v['feature'] for v in manifest['vm_checks']}


def get_encryption_feature_map():
    """获取加密方式的 ID 到 feature 的映射"""
    manifest = load_plugins_manifest()
    return {e['id']: e['feature'] for e in manifest['encryption']}


def get_run_mode_map():
    """获取运行模式的 ID 到 feature 的映射"""
    manifest = load_plugins_manifest()
    return {r['id']: r['feature'] for r in manifest['run_modes']}


def get_alloc_mem_feature_map():
    """获取内存分配方式的 ID 到 feature 的映射"""
    manifest = load_plugins_manifest()
    return {m['id']: m['feature'] for m in manifest.get('alloc_mem_modes', [])}


def get_encodings():
    """获取编码方式列表"""
    manifest = load_plugins_manifest()
    return manifest.get('encodings', [])


def get_encoding_feature_map():
    """获取编码方式的 ID 到 feature 的映射"""
    manifest = load_plugins_manifest()
    return {e['id']: e['feature'] for e in manifest.get('encodings', [])}


def get_default_value(key):
    """获取默认配置值"""
    manifest = load_plugins_manifest()
    return manifest['defaults'].get(key)
