## 上线功能
2023-10-18 15:06:01
- [x] 浏览器正常使用，windows 11, windows 10
- [x] 正常更新
- [x] 更新代码移到 wei-update.ps1
- [x] 测试：任务停止，才更新新版本
- [x] 合并 wei-server 到 wei
- [x] 新装系统无法运行 wei-ui
- [x] 任务模块联调

## 0.2

- [x] wei-ui 界面，安装软件，查询软件安装进度
- [x] 测试各种文件夹情况，中文，文件夹带空格等
- [x] 创建本地服务：碰见重复的端口，则端口加1，写进配置文件
- [x] 自动化提交新版本到微软白名单
- [x] windows docker 安装
- [x] windows docker 容器
- [x] windows frp 穿透
- [ ] 显示版本号以及产品名在界面和托盘
- [ ] git submodule 其它目录

## 0.3

- [ ] linux docker 安装 
- [ ] linux docker 容器
- [ ] linux frp 穿透
- [ ] linux frp server 端口
- [ ] zuiyue.com 接口对接
- [ ] dragonfly2 功能对接
- [ ] 详细了解/购买EV证书

## 0.4 

- [ ] 集群化管理
- [ ] 集群界面编写

## 0.1

- [x] 守护进程功能：统一管理其它进程，保证他们被关闭的时候再次开启
- [x] 主进程唯一性：只允许存在一个主进程
- [x] 默认进程列表：主进程开启需要拉起的进程，和守护进程合并
- [x] 单次启动：进程互相调用只执行一次
- [x] 下载程序：自动下载最新的功能模块
- [x] 守护类型自动退出：守护类型的程序当接收到退出代码，自动退出
- [x] 退出关闭wei-ui,wei-tray
- [x] 程序检测完整性

## 误报解决方案

- [ ] 误报思路：
    - [ ] 360优先，微软次之，Google最后
    - [ ] 第一步：编译文件超过50MB
    - [ ] 第二步：代码签名
    - [ ] 第三步：加大使用量，装机量
    - [ ] 第四步：当装机量达到一定程度之后，观察阈值并记录
    - [ ] 第五步：缩减文件大小

- [x] 当文件大小超过一定50MB，误报降至1%以下

- [x] 0.2.15 查询结果
    - [x] 55.86 MB wei-ui.exe 误报 https://www.virustotal.com/gui/file/5a77d504f8e4f02ed2c0258106b7c546d8c7b0e0953808239a944ba4f3173dee?nocache=1
    - [x] 51.75 MB wei.exe 无误报 https://www.virustotal.com/gui/file/6d9f20c53d132ddc7933ae3d5d99307e796be1ebb43d40c99fbb30029b545ef9?nocache=1
    - [x] 53.49 MB wei-qbittorrent.exe 无误报 https://www.virustotal.com/gui/file/752f2ed8d6167ad00728406b9843aea5ba37a5e0177f6d6bf3e1d020220ec826
    - [x] 53.65 MB wei-sd.exe 无误报 https://www.virustotal.com/gui/file/4c1128f1d5e5ca118e6b17e5ed9eec0310270a6f7dc15d07f799ef0fcdc45ed5
    - [x] 53.71 MB wei-task.exe 无误报 https://www.virustotal.com/gui/file/b23797a52878bf5adf2e736d503769d7932241793beb30846f2b8e77127b9407
    - [x] 52.98 MB wei-updater.exe 无误报 https://www.virustotal.com/gui/file/1962203f17eec5bc73a7825995391f952e655a446225568c7b7283a72a53ffe6?nocache=1
    
- [x] 代码签名：不能解决误报，但是能建立信誉。
- [x] Microsoft: 使用最高级别上报
- [x] 上报visualtotal，仅通过Microsoft
    - [x] wei-updater build 转移到 wei-build
    - [x] wei-build 区分不同系统和产品
    - [x] 调用 visualtotal api
    - [x] 显示 微软 报告情况及总误报数量
- [x] 代码混淆
    - [x] VMProtect: 报毒率为28%
    - [x] ConfuseRename: 只能混淆.NET代码
    - [x] ASM Guard: 直接报病毒，无法打开
- [x] 尝试使用不同电脑编译程序
- [x] strip：https://rustmagazine.github.io/rust_magazine_2021/chapter_4/hw_bin_opt.html
- [x] 微软365 defender上报病毒
- [x] 代码编译成dll：后期只更新dll，上传的dll什么也没有干，还是报毒
- [x] Nim免杀: 其它种类语言，并非工具
- [x] 完善右键信息
- [x] 加大代码量，合成一个应用程序:不符合现在的开发逻辑
- [x] 特定rust toolchain的版本的报毒率更低,使用debug版本报毒量少于release版本
- [x] 微软误报
    - [x] wei 合并 wei-server 功能
    - [x] 分离wei-daemon为应用程序，wei-daemon只负责启动其它程序，如果报毒直接上传到微软
    - [x] wei-updater，以及wei-daemon不再进行更新，当出现报毒的情况，直接上传到微软
    - [x] wei-build 先检测 product/windows/stable/wei-updater.exe 是否存在，如果存在则不再编译,复制wei-updater.exe到 product/windows/version/data/wei-updater.exe
    - [x] 测试上报 security.microsoft.com
    - [x] 进一步测试 wei-daemon.exe 是否会被误报，测试时间10天
    - [x] 代码里面如果有包含 .exe 的代码需要清除
    - [x] 代码分离ps1,在wei-updater kill()调用 wei-daemon-close.ps1, wei 区分不同系统调用 wei-daemon.ps1 或者 wei-daemon.exe
