# Command-line text rating application
Write a simple command-line textfile grading program. Use this project as a template. 

The program should accept directory path as it's input, print contents of each file in the directory and ask user for a rating from 1 to 5. The program should output summary of the ratings to stdout.

## Example
Assume existance of the following directory:
```
input/
├── 1.txt
├── 2.txt
└── 3.txt
```

This is how the program should be run: `cargo run ./input`

This is how the file content should be displayed:
```
This is the content of the first file.

Rating (1-5): *user input here*
```

Output:
```
./input/1.txt : 5
./input/2.txt : 2
./input/3.txt : 1
```