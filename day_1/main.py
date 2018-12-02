import argparse


def part1(path):
    frequency = 0
    with open(path) as file:
        for line in file:
            frequency = eval("frequency + " + line)
    print(frequency)


def part2(path):
    frequencies = {}
    frequency = 0
    while True:
        with open(path) as file:
            for line in file:
                frequency = eval("frequency + " + line)
                if frequency in frequencies:
                    print(frequency)
                    return
                frequencies[frequency] = True


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='A very unsafe day one solution. :)')
    parser.add_argument('input', type=str)
    args = parser.parse_args()

    part1(args.input)
    part2(args.input)
