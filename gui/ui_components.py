import os
from PyQt5.QtCore import QSize
from PyQt5.QtWidgets import QComboBox, QCheckBox, QGridLayout, QListView, QStyledItemDelegate
from PyQt5.QtGui import QIcon
from .config_manager import load_plugins_manifest, get_default_value



def get_folder_icon():
    icon_path = os.path.join('gui', 'icons', 'folder.ico')
    return QIcon(icon_path) if os.path.exists(icon_path) else QIcon()


def get_icon(icon_name):
    icon_path = os.path.join('gui', 'icons', f'{icon_name}.ico')
    return QIcon(icon_path) if os.path.exists(icon_path) else QIcon()


def create_encryption_combobox():
    combo = QComboBox()
    combo.setIconSize(QSize(20, 20))
    
    enc_icon = get_icon('enc')
    manifest = load_plugins_manifest()
    
    for e in manifest['encryption']:
        combo.addItem(enc_icon, e.get('label', e['id']), e['id'])
    
    default_enc = get_default_value('encryption')
    if default_enc:
        for i in range(combo.count()):
            if combo.itemData(i) == default_enc:
                combo.setCurrentIndex(i)
                break
    
    return combo


def create_mem_mode_combobox():
    combo = QComboBox()
    mem_icon = get_icon('mem')
    
    manifest = load_plugins_manifest()
    mem_modes = manifest.get('alloc_mem_modes', [])
    
    for m in mem_modes:
        combo.addItem(mem_icon, m.get('label', m['id']), m['id'])
    
    default_mem = get_default_value('alloc_mem_mode')
    if default_mem:
        for i in range(combo.count()):
            if combo.itemData(i) == default_mem:
                combo.setCurrentIndex(i)
                break
    
    return combo


def create_load_payload_combobox():
    combo = QComboBox()
    load_icon = get_icon('load')  # Assuming 'load.ico' exists or will fallback to empty
    
    manifest = load_plugins_manifest()
    load_modes = manifest.get('load_payload_modes', [])
    
    for m in load_modes:
        combo.addItem(load_icon, m.get('label', m['id']), m['id'])
    
    default_load = get_default_value('load_payload_mode')
    if default_load:
        for i in range(combo.count()):
            if combo.itemData(i) == default_load:
                combo.setCurrentIndex(i)
                break
    
    return combo


def create_vm_checks_grid():
    manifest = load_plugins_manifest()
    vm_items = manifest.get('vm_checks', [])
    
    if not vm_items:
        vm_items = [
            {'id': t, 'label': t} for t in [
                'c_drive', 'desktop_files', 'tick', 'memory', 'api_flood',
                'mouse', 'common_software', 'uptime'
            ]
        ]
    
    grid = QGridLayout()
    checkboxes = []
    
    for i, item in enumerate(vm_items):
        text = item.get('label', item.get('id', ''))
        vm_id = item.get('id', text)
        
        cb = QCheckBox(text)
        cb.setProperty('vm_id', vm_id)
        checkboxes.append(cb)
        
        grid.addWidget(cb, i // 4, i % 4)
    
    return grid, checkboxes


def create_run_mode_combobox():
    combo = QComboBox()
    combo.setIconSize(QSize(20, 20))
    
    run_icon = get_icon('run')
    manifest = load_plugins_manifest()
    
    for rm in manifest['run_modes']:
        combo.addItem(run_icon, rm.get('label', rm['id']), rm['id'])
    
    default_rm = get_default_value('run_mode')
    if default_rm:
        for i in range(combo.count()):
            if combo.itemData(i) == default_rm:
                combo.setCurrentIndex(i)
                break
    
    return combo



def create_target_combobox():
    combo = QComboBox()
    combo.setView(QListView())
    combo.setItemDelegate(QStyledItemDelegate())
    
    target_icon = get_icon('target')
    
    targets = [
        ('x86_64-pc-windows-msvc', 'Windows MSVC (x64)'),
        ('i686-pc-windows-msvc', 'Windows MSVC (x86)'),
        ('x86_64-pc-windows-gnu', 'Windows GNU (x64)'),
        ('i686-pc-windows-gnu', 'Windows GNU (x86)'),
        ('aarch64-pc-windows-msvc', 'Windows MSVC (ARM64)'),
    ]
    
    for target, label in targets:
        combo.addItem(target_icon, label, target)
    
    import platform
    os_name = platform.system().lower()
    if os_name == "windows":
        default_target = "x86_64-pc-windows-msvc"
    else:
        default_target = "x86_64-pc-windows-gnu"
    
    for i in range(combo.count()):
        if combo.itemData(i) == default_target:
            combo.setCurrentIndex(i)
            break
    
    return combo
