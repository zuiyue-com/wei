#wei

## 0.1

- [x] Daemon process function: unified management of other processes to ensure that they are reopened when they are closed
- [x] Main process uniqueness: only one main process is allowed to exist
- [x] Default process list: processes that need to be pulled up when the main process starts, merged with daemon processes
- [x] Single start: processes call each other only once
- [x] Download program: automatically download the latest function modules
- [x] Automatic exit of daemon type: When a daemon type program receives an exit code, it will automatically exit.
- [x] Exit and close wei-ui,wei-tray
- [x] Program check integrity

## 0.2

- [ ] wei-ui interface, install software, query software installation progress
- [ ] Test various folder situations, Chinese, folders with spaces, etc.
- [ ] Create a port to provide access: process communication method. If you encounter a port that is the same as your own, add 1 to your own port until it is no longer repeated, and write it into the configuration file
- [ ] linux docker installation
- [ ] linux docker container opening
- [ ] linux frp penetration
- [ ] linux frp server port

## todo



## Basic folder structure

```
wei.exe
data/
data/version.dat
data/daemon.dat
data/checksum.dat
data/wei-task.exe
data/wei-updater.exe
```