# wei

## 0.1

- [x] 守护进程功能：统一管理其它进程，保证他们被关闭的时候再次开启
- [x] 主进程唯一性：只允许存在一个主进程
- [x] 默认进程列表：主进程开启需要拉起的进程，和守护进程合并
- [x] 单次启动：进程互相调用只执行一次
- [x] 下载程序：自动下载最新的功能模块
- [x] 守护类型自动退出：守护类型的程序当接收到退出代码，自动退出
- [x] 退出关闭wei-ui,wei-tray
- [x] 程序检测完整性

## 0.2

- [ ] wei-ui 界面，安装软件，查询软件安装进度
- [ ] 测试各种文件夹情况，中文，文件夹带空格等
- [ ] 创建端口提供访问：进程通讯方式,碰见和自己重复的端口，则为自己的端口加1，直到不重复为止，并写进配置文件
- [ ] linux docker 安装
- [ ] linux docker 容器开启
- [ ] linux frp 穿透
- [ ] linux frp server 端口

## todo



## 基础文件夹结构

```
wei.exe
data/
data/version.dat
data/daemon.dat
data/checksum.dat
data/wei-task.exe
data/wei-updater.exe
```