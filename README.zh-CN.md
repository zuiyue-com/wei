# wei

## 0.1

- [x] 守护进程功能：统一管理其它进程，保证他们被关闭的时候再次开启
- [x] 主进程唯一性：只允许存在一个主进程
- [x] 默认进程列表：主进程开启需要拉起的进程，和守护进程合并
- [x] 单次启动：进程互相调用只执行一次
- [ ] 下载程序：自动下载最新的功能模块

```
先检查 data/checksum.dat,如果不存在直接下载https://download.zuiyue.com/os/latest/data/checksum.dat，通过 checksum.dat 下载所有最新的文件和应用程序。

碰到 data/checksum.dat 文件不统一的情况，先从 data/new/0.1.2 版本里面复制对应的文件到 data/ 目录下面，然后再检查，如果还是不统一，则从远程对应系统里面的latest下载所有最新的文件和应用程序。

```

- [ ] 守护类型自动退出：守护类型的程序当接受到退出代码，自动退出
- [ ] 退出关闭wei-ui,wei-tray

## 0.2

- [ ] 程序检测完整性
- [ ] 测试各种文件夹情况，中文，文件夹带空格等
- [ ] linux docker 安装
- [ ] linux docker 容器开启
- [ ] linux frp 穿透
- [ ] linux frp server 端口

## todo

- [ ] 创建端口提供访问：进程通讯方式,碰见和自己重复的端口，则为自己的端口加1，直到不重复为止，并写进配置文件


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