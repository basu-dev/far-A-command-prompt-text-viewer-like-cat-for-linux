# far (A file reader for Windows System like `cat` for linux.)
A file opener for Windows command prompt and PowerShell like 'cat' for linux distributions.

## Motivation
Using linux it was very comforting to see the file contents on the terminal using `cat` command.
But while using windows , there is none, at least not preinstalled.So I tried to create the similar application that uses command `far` for Windows Command Prompt and Powershell.

## Usage
Using `far` you can do same stuff as `cat` like 
```sh 
far src/main.js
```

The syntax is 
```sh
	far <filepath> < --c | --copy | copy>
```
where
- far is the command
- replace <filepath> with actual filepath like 	`src/main.js` or `abc.txt`
Now with this command it will print the file like 
```sh
 C:\Users\ABC\Desktop>far main.js

 1  | var name="John Doe";
 2  | function printName(name){
 3  |	console.log(name)
 4  | }
 5  | printName(name);
 6  |

```
But if you want to select and copy you don't want the row counter at the left. So to sort it out there is third argument.
The third argument is optional. You can put any of `--c` or `--copy` or `copy` as the third argument and the output looks like following:

```sh
 C:\Users\ABC\Desktop>far main.js --c

 var name="John Doe";
 function printName(name){
	console.log(name)
 }
 printName(name);

```
And you can select and copy any part of the code and paste wherever you want it.

## Install
For now there is no way to directly download the executable. So you can clone this git repository.
```sh
git clone https://github.com/basu-dev/far-A-command-prompt-text-viewer-like-cat-for-linux.git
```
After the command completes, go to the folder and in `target/debug ` folder you see the executable named `far.exe`.
You can directly paste the path to that executable to environment variable path and you can use `far` in your command prompt.
If you dont care about the rest of the folders and files, just copy the executable and paste it somewhere else and put that path to your environment variable and remove the rest of the folders.

