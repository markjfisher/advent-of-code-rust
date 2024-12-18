import sys

def parse_input():
    # skip 3 registers as not needed for part2
    input()
    input()
    input()
    
    # Read empty line
    input()
    
    # Read program line and convert to list of integers
    program_line = input().strip()
    program = [int(x) for x in program_line.split(": ")[1].split(",")]
    
    return program

def part2(program):
    print(f"Program data: {program}")
    a = 0
    
    # Find the two bxl constants using chunks of 2
    bxl_values = []
    for i in range(0, len(program)-1, 2):
        if program[i] == 1:
            bxl_values.append(program[i + 1])
    
    print(f"\nFound bxl_values: {bxl_values}")
    
    if len(bxl_values) != 2:
        raise ValueError(f"Did not find two bxl commands (found {len(bxl_values)})")
    
    bxl1, bxl2 = bxl_values
    
    # Rest of the function remains the same
    for n in range(1, len(program) + 1):
        target = program[-n:]
        
        new_a = a << 3
        while True:
            digits = []
            test_a = new_a
            
            while test_a != 0:
                b = test_a & 0x07
                b = b ^ bxl1
                c = test_a >> b
                b = b ^ c
                b = b ^ bxl2
                test_a >>= 3
                
                test_digit = b & 0x07
                if test_digit != target[len(digits)]:
                    break
                
                digits.append(test_digit)
            
            if digits == target:
                a = new_a
                break
            
            new_a += 1
    
    return a

def main():
    program = parse_input()
    result = part2(program)
    print(f"Result: {result}")

if __name__ == "__main__":
    main()
