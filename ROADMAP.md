## 误报解决方案

- [ ] 上报visualtotal，仅通过Microsoft
    - [ ] wei-updater build 转移到 wei-build
    - [ ] wei-build 区分不同系统和产品
    - [ ] 调用 visualtotal api
    - [ ] 显示 微软 报告情况及总误报数量
- [ ] 代码混淆
    - [ ] VMProtect
    - [ ] ConfuseRename
- [ ] 代码签名：不能解决误报，但是能建立信誉。
- [ ] 尝试使用不同电脑编译程序
- [ ] strip：https://rustmagazine.github.io/rust_magazine_2021/chapter_4/hw_bin_opt.html
- [ ] 微软365 defender上报病毒

## 0.2

- [x] wei-ui 界面，安装软件，查询软件安装进度
- [x] 测试各种文件夹情况，中文，文件夹带空格等
- [x] 创建本地服务：碰见重复的端口，则端口加1，写进配置文件
- [x] 自动化提交新版本到微软白名单
- [ ] windows docker 安装
- [ ] windows docker 容器
- [ ] windows frp 穿透
- [ ] linux frp server 端口自动查找
- [ ] 下载进度条，显示当前下载内容
- [ ] 显示版本号在界面和托盘

## 0.3

- [ ] linux docker 安装 
- [ ] linux docker 容器
- [ ] linux frp 穿透
- [ ] linux frp server 端口
- [ ] zuiyue.com 接口对接
- [ ] dragonfly2 功能对接

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
