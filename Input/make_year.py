import os
import shutil

abspath = os.path.abspath(__file__)
os.chdir(os.path.dirname(abspath))

year = input("Year [eg 2019]:")

if not os.path.exists(f"./{year}"):
    print(f"Making directory ./{year}")
    os.makedirs(f"./{year}")

for day in range(1, 13):
    d = f"{day:02d}"
    cs = f"./{year}/{d}.txt"
    if not os.path.isfile(cs):
        open(cs, 'a').close()
