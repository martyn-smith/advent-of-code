with open("data/8.txt") as f:
    image = f.read()
    width = 25
    height = 6
    layers = [image[i*(width*height):(i+1)*(width*height)] for i in range(len(image) // (width*height))]

def part_1():
    min_layer = min(layers, key = lambda l: l.count("0"))
    return min_layer.count("1") * min_layer.count("2")

if __name__ == "__main__":
    print(part_1())