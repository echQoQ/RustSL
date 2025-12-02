def get_main_stylesheet():
    return """
    QWidget {
        background: #f7f7f7;
        color: #222;
        font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Segoe UI', 'Microsoft YaHei', Arial;
        font-size: 16px;
    }
    QGroupBox {
        border: 1px solid #bfc4cc;
        border-radius: 8px;
        margin-top: 10px;
        background: #ffffff;
        font-weight: bold;
        padding-top: 10px;
        font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Segoe UI', 'Microsoft YaHei', Arial;
        font-size: 18px;
    }
    QGroupBox:title {
        subcontrol-origin: margin;
        left: 10px;
        padding: 0 3px 0 3px;
    }
    QLabel {
        color: #333;
        font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Segoe UI', 'Microsoft YaHei', Arial;
    }
    QLineEdit, QComboBox, QTextEdit {
        background: #fff;
        border: 1px solid #bfc4cc;
        border-radius: 5px;
        color: #222;
        padding: 4px;
        font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Segoe UI', 'Microsoft YaHei', Arial;
    }
    QPushButton {
        background: qlineargradient(x1:0, y1:0, x2:0, y2:1, stop:0 #e3eaff, stop:1 #b3cfff);
        color: #222;
        border-radius: 6px;
        padding: 6px 18px;
        font-weight: bold;
        font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Segoe UI', 'Microsoft YaHei', Arial;
    }
    QPushButton:hover {
        background: #d0e3ff;
    }
    QProgressBar {
        border: 1px solid #bfc4cc;
        border-radius: 6px;
        text-align: center;
        background: #fff;
        height: 18px;
    }
    QProgressBar::chunk {
        background: qlineargradient(x1:0, y1:0, x2:1, y2:0, stop:0 #b3cfff, stop:1 #a7e2d8);
        border-radius: 6px;
    }
    QTextEdit {
        font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Consolas', 'Fira Mono', 'Microsoft YaHei';
        font-size: 15px;
        background: #f7f7f7;
        color: #333;
    }
    """