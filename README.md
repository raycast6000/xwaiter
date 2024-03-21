# XWaiter
Static file hosting for dummies and nerds.
### Why?
- It's memory safe! (as it's written in Rust)
- It's extremely fast.
- It's simple and straightforward.
# How to use it
For obvious reasons, XWaiter only requires one argument `--directory`, also known as "The Root Directory", which specifies the location of the files you want to host.\
Here are two examples on how to provide the `--directory` argument.  
```bash
$ xwaiter --directory ~/torrents --port 4040 
```
```bash
$ xwaiter --directory ./public --port 1245 
```
Although the `--directory` argument is mandatory, if you don't want to use command line arguments you can use a configuration file `xwaiter.config.json`. Just like this:
```json
{
    "directory": "./public",
    "port": 1234,
    "threads": 8 // XWaiter does not support multithreading yet.
}
```
XWaiter automatically detects a configuration file if it's in the same directory as you.
# Installation
Just copy and paste this command:
```bash
cargo install --git https://github.com/raycast6000/xwaiter.git
```
### Windows (for normies)
Go to [Releases](https://github.com/raycast6000/xwaiter/releases), download the latest MSI installer and just run it.