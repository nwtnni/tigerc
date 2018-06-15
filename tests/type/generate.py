def generate():
    for i in range(1, 50):
        with open("appel_{:0>2}.typedsol".format(i), "w") as outfile:
            outfile.write("Valid Tiger Program")


if __name__ == "__main__":
    generate()
