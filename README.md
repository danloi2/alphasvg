# AlphaSVG

![GitHub release (latest by date)](https://img.shields.io/github/v/release/danloi2/transparente?style=flat-square&color=blue)
![Python Version](https://img.shields.io/badge/python-3.11-blue?style=flat-square&logo=python)
![Rust Version](https://img.shields.io/badge/Rust-1.93.0-blue?style=flat-square&logo=rust)
![OpenCV](https://img.shields.io/badge/OpenCV-5.x-white?style=flat-square&logo=opencv&logoColor=white&color=5C3EE8)
![ONNX Runtime](https://img.shields.io/badge/ONNX-Runtime-orange?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)    

**AlphaSVG** is a Rust-powered background removal and vectorization toolkit that transforms any photo into production-ready transparent SVGs. Using state-of-the-art ONNX models (17+ variants: U2Net, BiRefNet, SAM, Bria-RMBG) with automatic caching, it generates crisp alpha masks via AI inference, then offers **5 artistic SVG styles**: color logos (16 hues), rich illustrations (48 hues), posterized grayscale, classic halftone dots, and clean lineart contours—all leveraging Potrace for pixel-perfect vector paths.

## ✨ Features

| **AI Background Removal** | **Vector Styles** | **Production Ready** |
|------------------------|-------------------|---------------------|
| 17+ SOTA models (4-358MB) | Color Logo (16 hues) | Auto model download |
| ONNX Runtime inference | Rich Illustration (48 hues) | Smart session caching |
| Lanczos3 resize | Posterized Grayscale | Transparent SVGs |
| ImageNet preprocessing | Halftone Dots | Thread-safe |
| `~/.transparente_models/` cache | Lineart Contours | Zero-config |

## 🎯 Quick Start

### Prerequisites
- **Conda** (recommended: [Miniforge](https://github.com/conda-forge/miniforge))
- **potrace** CLI (`brew install potrace` / `apt install potrace`)

### 1. Setup Environment
```bash
conda env create -f environment.yml
conda activate alphasvg
```

### 2. Run GUI
```bash
python gui_main.py
```

## 🛠️ Build Native macOS App (.dmg)

```bash
# PyInstaller bundle
pyinstaller --onedir --windowed --noconfirm \
  --name "AlphaSVG" \
  --collect-all cv2 onnxruntime scipy \
  gui_main.py

# Create DMG installer
dmgbuild -s dmg_settings.py "AlphaSVG" dist/AlphaSVG-Installer.dmg
```

**Note**: Un-signed macOS app → Allow in **System Settings > Privacy & Security**.

## 🏗️ Architecture

```
📷 Input Image → 🧠 Rust AI Core → 🎨 SVG Generator → 📄 Vector Output
                    ↓                           ↓
              ONNX Models (17+)          Potrace Vectorization
                    ↓                           ↓
           ~/.transparente_models/     5 Artistic Styles
```

- **Rust Core** (`rust/`): Model registry, ONNX inference, Luma masks
- **Python GUI** (`py/`): User interface + style selection
- **Dual power**: Performance-critical AI in Rust, UX in Python

## 📁 Structure
```
alphasvg/
├── rust/           # AI inference + model management
│   ├── models.rs   # 17+ model configs (U2Net, BiRefNet...)
│   ├── core.rs     # ONNX Runtime + caching
│   └── svg/        # Color/monochrome vectorization
├── py/             # Python GUI + integration
├── environment.yml # Conda deps
└── gui_main.py     # Main app entry
```

## 🎨 Output Examples

| **Color Logo** | **Illustration** | **Grayscale** | **Halftone** | **Lineart** |
|---|---|---|---|---|
|  |  |  |  |  |

## 📦 Releases & Packages

Download [v1.0.0](https://github.com/danloi2/alphasvg/releases/tag/v1.0.0) for macOS.

## 🤝 Contributing

1. Fork & clone
2. `conda env create -f environment.yml`
3. `conda activate alphasvg`
4. Hack away! 🎉
5. PR with tests

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 👥 Author

**Developed by Daniel Losada**

[![GitHub](https://img.shields.io/badge/GitHub-danloi2-181717?style=for-the-badge&logo=github)](https://github.com/danloi2)
[![Researcher EHU](https://img.shields.io/badge/Researcher-EHU-blue?style=for-the-badge&logo=researchgate)](https://github.com/danloi2)

---

_Developed with ❤️ for the educational community._
