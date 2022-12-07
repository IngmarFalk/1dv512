
import sys
from os import listdir
from os.path import isfile, join

def main() -> None:
    path: str = sys.argv[1]
    files: list[str] = [path + "/" + f for f in listdir(path) if isfile(join(path, f))]

    cnt: int = 0
    for file in files:
        with open(file) as f:
            for line in f.readlines():
                cnt += 1

    print(cnt)



if __name__ == "__main__":
    main()
