# far (A file reader for Windows System like `cat` for linux.)
A file opener for Windows command prompt and PowerShell like 'cat' for linux distributions.

## Motivation
Using linux it was very comforting to see the file contents on the terminal using `cat` command.
But while using windows , there is none, at least not preinstalled.So I tried to create the similar application that uses command `far` for Windows Command Prompt and Powershell.

## Install
To download the `far` visit https://farcli.netlify.app/ and follow accordingly.

After the command completes, You can directly paste the path to that executable file `far.exe` to environment variable and you can use `far` in your command prompt.
Now go to Command Prompt and type `far`, it should work. If it doesn't work, the path to the executable is not placed in environment variable. 
If you don't know that you can make a google on how to set environment variable and you are ready to go on.


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



