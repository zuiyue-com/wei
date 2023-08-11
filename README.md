# wei

- [ ] 主进程唯一性：只允许存在一个主进程
- [ ] 守护进程功能：统一管理其它进程，保证他们被关闭的时候再次开启
- [ ] 单次启动：进程互相调用只执行一次
- [ ] 创建端口提供访问：进程通讯方式
- [ ] 默认进程列表：主进程开启需要拉起的进程
- [ ] 下载程序：自动下载最新的功能模块

# 功能拆分

- [x] wei-tray: 托盘程序
- [x] wei-daemon: 守护进程
- [x] wei-log: 日志
- [x] wei-docker: docker 管理程序
- [x] wei-env: 配置文件
- [x] wei-hardware: 硬件参数
- [x] wei-docker-install: docker 安装程序
- [x] wei-update: 自动更新
- [x] wei-ui: UI界面
