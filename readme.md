[Basic Rust FTP library]
- 
    - supports features such as:
    - FTP file reading
    - Safe and contained filereading
    - EPSV mode
    - basic file storage.
    - folder creation on server.
[getting started]
- 
Use Cargo to build a release i.e cargo build -r then copy paste the .exe into a directory of your choosing and place a paths.txt file in the directory as well. This directory will contain the absolute paths of files that ftp can access separated by a newline.

To run the program run ./name [IPADDRESS] [root_dir] to run the program, the root_dir path is for
the absolute storage path that you want people to store files if they store files in the root directory
