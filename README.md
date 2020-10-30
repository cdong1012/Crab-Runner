# CrabRunner
> A shellcode runner written in Rust

# Motivation
During Flare-On 7, I came across a lot of challenges where shellcode analysis was crucial, and I solved them using shellcode runners that I found online.


Up until that point, I never really thought much about how these programs were implemented. In the course of my internship interview with FireEye, I was asked about how I would implement my own runner to solve the challenges, and that had inspired me to write one in my favorite language now!


## Screenshots
![](/images/Prompt.PNG)


## Usage


### Running with Command Prompt

    .\crab_runner.exe <shellcode file>

### Params
Specifying the offset for the shellcode to start. 
*Note: offset can be in both hex and decimal form*

    .\crab_runner.exe <shellcode file> --offset 0x10
    .\crab_runner.exe <shellcode file> --offset 16

Dumping shellcode to another file to capture self-modification.
*Note: see [Shellcode Dumping](#dump-and-run)*

    .\crab_runner.exe <shellcode file> --dump

Print the version of the program

    .\crab_runner.exe --version

Print help for all command-line options:

    .\crab_runner.exe
    .\crab_runner.exe --help


## Features

### Normal run

During a normal run, the shellcode is read from the file and written to a buffer allocated by [VirtualAlloc](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc).


Then, the program creates a thread in the suspended state with the entry point equals to the base of the *buffer* added by the *offset* (if provided).

### Dump and run

When the param **"--dump"** is given, the program will copy the content of the given file into a new file. The name of this new file will be the name of the original file appended with *_dump.out*.


This new file will be mapped into memory, and its base address will be the entry point for the suspended thread.


When the shellcode runs, if there is any self-modification, the changes should be capture in this file we dump.

For example, if the shellcode unpacks itself, this dumped file will contain the unpacked version after the first run.


### Attaching a debugger

![](/images/attach.PNG)

Before resuming the thread, make sure to attach a debugger through the following steps:

    1. Open your favorite debugger
    2. Attach your debugger to the given thread
    3. Go to the given entry point and place a breakpoint (Important!!)
    4. Hit "Run"
    5. Enter anything to begin in your command prompt
    6. Hit "Run" until you hit this breakpoint!


## For developers

Clone the source locally:

    $ git clone https://github.com/cdong1012/Crab-Runner

Build project

    $ cargo build --release

Run project

    $ cargo run


## Acknowledgement

OALabs - [BlobRunner](https://github.com/OALabs/BlobRunner)

adamkramer - [jmp2it](https://github.com/adamkramer/jmp2it)

Omar Sardar and Michael Bailey - Inspire me to write my own shellcode runner in our interview

## License

MIT © [Chuong Dong](http://chuongdong.com/)