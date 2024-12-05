<div align="center">

# 🎄 Advent of Code 2024

</div>

You're probably already familiar with [Advent of Code](https://adventofcode.com/), so let's skip that. This repository contains my solutions for this year, where my primary goal is to really dive deeper into Rust and become fluent in the language.

## 📁 Folder structure

    .
    ├── days                            # Individual Rust crates for each challenge
    │   ├── day01                      
    │   │   ├── Cargo.lock
    │   │   ├── Cargo.toml
    │   │   └──  src                    # Entry point for cargo to build and run both parts
    │   │       ├── main.rs             
    │   │       ├── part1.rs   
    │   │       └── part2.rs
    │   └── ...
    ├── inputs                          # Separate input files for each challenge
    │   ├── day01.txt                  
    │   └── ...
    ├── templates                       # Boilerplate files
    │   ├── main.rs
    │   └── part.rs
    ├── .gitignore
    ├── Makefile                        # Automated build and run instructions
    ├── README.MD
    └── generator.sh                    # Helper script to quickly generate all files

## 🚀 Quick Start

### 1. Clone the project

```bash
git clone https://github.com/alex-croitoriu/Advent-of-Code-2024.git
```

### 2. Go to the project directory

```bash
cd Advent-of-Code-2024
```

### 3. Build and run

Running these commands will process the same input for both parts of the daily challenge and print out the two separate answers.

```bash
# Build and run the solution for the current day 
# (only makes sense during advent season)
make

# Build and run the solution for a specified day
# (will work for the range 01..25)
make DAY=01
```
