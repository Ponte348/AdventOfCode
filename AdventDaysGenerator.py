# O que a preguiça não faz...

import os
import sys

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 AdventDaysGenerator.py <year>")
        return

    year = sys.argv[1]
    if not year.isdigit():
        print("Usage: python3 AdventDaysGenerator.py <year>")
        return

    year = int(year)
    if year < 2015:
        print("Advent of Code only started in 2015!\nUsage: python3 AdventDaysGenerator.py <year>")
        return

    # create folder in current directory named "AdventOfCode<year>"
    folder_name = "AdventOfCode" + str(year)
    if os.path.exists(folder_name):
        print("Folder already exists!")
        return
    os.mkdir(folder_name)
    
    # create a file named "README.md" inside the folder
    file_name = folder_name + "/README.md"
    f = open(file_name, "w")
    f.write("# Advent of Code " + str(year) + "\n\n")
    
    # now inside the folder, create 25 folders, one for each day, days should always be 2 digits
    for i in range(1, 26):
        day = str(i)
        if i < 10:
            day = "0" + day
        os.mkdir(folder_name + "/Day" + day)
        
        # inside each day folder, create a file named "Day<day>.ipynb" and an "input.txt" file
        file_name = folder_name + "/Day" + day + "/Day" + day + ".ipynb"
        input_name = folder_name + "/Day" + day + "/input.txt"
        f = open(file_name, "w")
        f.close()
        f = open(input_name, "w")
        f.close()
        
if __name__ == "__main__":
    main()
