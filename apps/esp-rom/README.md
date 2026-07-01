# ESP-ROM 开发指引

## 环境准备

### 1. 安装 Rust ESP 工具链

```bash
# 安装 espup
cargo install espup

# 安装 ESP32-S3 工具链
espup install

# 如果在vscode，zed等上使用rust-analyzer 出现 '`cargo metadata` failed and returning succeeded result with `--no-deps` error=`cargo metadata` exited with an error: error: unexpected argument '--lockfile-path' ...'
# 参考 https://github.com/ariel-os/ariel-os/issues/1468
espup update -v 1.96.0.0 # 更新一个支持ra的版本
```

安装完成后，确保 `xtensa-esp32s3-espidf` 目标可用：

```bash
rustup show | grep esp
```

### 2. 安装 espflash

```bash
cargo install espflash
```


### 3. 安装 ldproxy

```bash
cargo install ldproxy
```

## WSL 开发环境 USB 设备转发

在 WSL 中开发时，ESP32 设备通过 USB 连接在 Windows 宿主机上，需要使用 `usbipd-win` 将 USB 设备转发到 WSL 中。

### 1. Windows 端安装 usbipd-win

在 Windows 上使用 winget 安装：

```powershell
winget install usbipd
```

### 2. WSL 端安装 USB 工具

```bash
sudo apt install linux-tools-generic hwdata
sudo update-alternatives --install /usr/local/bin/usbip usbip /usr/lib/linux-tools/*-generic/usbip 20
```

### 3. 转发 USB 设备到 WSL

在 **Windows PowerShell（管理员）** 中操作：

```powershell
# 列出所有 USB 设备，找到 ESP32 对应的 BUSID
usbipd list

# 绑定设备（首次使用需要）
usbipd bind --busid <BUSID>

# 将设备附加到 WSL
usbipd attach --wsl --busid <BUSID>
```

### 4. 在 WSL 中验证设备

```bash
# 查看是否识别到设备
lsusb

# 查看串口设备
ls /dev/ttyUSB* /dev/ttyACM*
```

### 5. 串口权限配置

```bash
# 将当前用户添加到 dialout 组（一次性操作）
sudo usermod -aG dialout $USER

# 或者临时修改设备权限
sudo chmod 666 /dev/ttyUSB0
```

### 6. 断开设备

在 Windows PowerShell 中：

```powershell
usbipd detach --busid <BUSID>
```

> **提示**：每次重新插拔 USB 设备后，都需要重新执行 `usbipd attach` 命令。

