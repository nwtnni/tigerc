def generate(category, n):
    for i in range(1, n + 1):
        inpath = "{}_{:0>2}.tig".format(category, i)
        outpath = "{}_{:0>2}.lexedsol".format(category, i)

        with open(inpath, "r") as infile:
            with open(outpath, "w") as outfile:
                keyword = infile.readline().strip()
                outfile.write("1:1 {} {}".format(category.upper(), keyword))


def main():
    generate("keyword", 17)
    generate("operator", 14)
    generate("symbol", 9)


if __name__ == "__main__":
    main()
