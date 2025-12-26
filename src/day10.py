import re
from z3 import Int, Optimize, sat

def main():
    print("Day 10: Part 2")

    result_part2 = 0
    with open("./src/inputs/day10_input.txt") as f:

        lines = f.readlines()
        for line in lines:
            line = line.strip()

            switches_input = re.findall(r"\((.*?)\)", line)
            switches = []
            for switch in switches_input:
                switch_list = list(map(int, switch.split(",")))
                switches.append(switch_list)

            joltages_input = re.findall(r"\{(.*?)\}", line)[0]
            joltages = list(map(int, joltages_input.split(",")))

            variables = []
            for i in range(len(switches)):
                variables.append(Int(f"x{i}"))

            opt = Optimize()
            # Equations for sum(switches) = joltages
            for i, joltage in enumerate(joltages):
                corresponding_switches = []
                for j, switch in enumerate(switches):
                    if i in switch:
                        corresponding_switches.append(variables[j])
                opt.add(sum(corresponding_switches) == joltage)

            for var in variables:
                opt.add(var >= 0)

            # Minimize sum of all variables
            total_sum = sum(variables)
            opt.minimize(total_sum)

            if opt.check() == sat:
                model = opt.model()
                result = sum([model[var].as_long() for var in variables])
                result_part2 += result

    print(f"Part 2: {result_part2}")

if __name__ == "__main__":
    main()