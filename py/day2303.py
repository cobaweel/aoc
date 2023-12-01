import re


def part1(path):
    grid = parse_grid(path)
    xx = len(grid)
    yy = len(grid[0])
    ret = 0

    def check(mid,lo,hi):
        for x in mid-1,mid,mid+1:
            for y in range(lo-1,hi+2):
                if not (0<=x<xx and 0<=y<yy):
                    continue
                if grid[x][y] not in "0123456789.":
                    return True
        return False
    
    for mid,lo,hi,num in parse_numbers(path):
        if check(mid,lo,hi):
            ret += num

    return ret          

def parse_grid(path):
    return [line.strip() for line in open(path).readlines()]

def parse_numbers(path):
    numbers = []
    for i, line in enumerate(open(path).readlines()):
        for match in re.finditer(r"\d+", line):
            num = int(match[0])
            lo, hi = match.span()
            numbers.append((i, lo, hi, num))
    return numbers


assert part1('input/230301.txt') == 4361
