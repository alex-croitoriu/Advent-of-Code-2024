## Introduction
You're probably already familiar with [Advent of Code](https://adventofcode.com/), so let's skip that. This repository contains my solutions for this year, where my primary goal is to really dive deeper into Rust and become fluent in the language.

## Project structure

    .
    ├── days                    # Daily challenges, each with its own Rust crate
    ├── inputs                  # Input data for each challenge
    ├── templates               # Boilerplate code for starting each challenge
    ├── .gitignore                    
    ├── generator.sh            # Helper script to quickly generate all files
    ├── Makefile                # Automated build and run instructions
    └── README.md

## Quick Start

### 1. Clone the project

```bash
git clone https://github.com/alex-croitoriu/Advent-of-Code-2024.git
```

### 2. Go to the project directory

```bash
cd Advent-of-Code-2024
```

### 3. Run

Running these will process the same input for both parts of the daily challenge and print out the two separate answers.

```bash
# Build and run the solution for the current day 
# (only makes sense during advent season)
make

# Build and run the solution for a specified day
# (will work for the range 01..25)
make DAY=01
```
