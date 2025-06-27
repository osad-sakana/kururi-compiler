def main():
    print("掛け算九九の表")
    print("=================")
    for i in range(int(9)):
        row = ""
        for j in range(int(9)):
            num1 = str(i) + str(1)
            num2 = str(j) + str(1)
            result = num1 * num2
            if result < 10:
                row = str(str(str(row) + str(" ")) + str(result)) + str(" ")
            else:
                row = str(str(row) + str(result)) + str(" ")
        print(row)

if __name__ == "__main__":
    main()