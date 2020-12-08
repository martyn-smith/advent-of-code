with open("8.txt") as f:
    image = f.read()

width = 25
height = 6

layers = [image[i*(width*height):(i+1)*(width*height)] for i in range(len(image) // (width*height))]
min_layer = min(layers, key = lambda l: l.count("0"))
print(min_layer.count("1") * min_layer.count("2"))