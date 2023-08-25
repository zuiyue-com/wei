# wei

## 0.1

- [] Daemon function: Unified management of other processes, ensuring that they are restarted when they are shut down
- [] Main process uniqueness: Only one main process is allowed to exist
- [] Default process list: The main process starts the process that needs to be pulled
- [x] Single start: processes call each other and only execute once
- [] Download program: Automatically download the latest functional modules
- [] Automatic exit of daemon type: When a program of daemon type receives an exit code

## todo

- [] Create a port to provide access: Process communication method. If you encounter a duplicate port with yourself, add 1 to your own port until there are no duplicates, and write it into the configuration file