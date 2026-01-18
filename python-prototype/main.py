import os

#examples = ["test-add","test-division","test-exponentiation","test-mult","test-values"]
examples = ["test-symbolics"]
clearOnStart = True

if clearOnStart: os.system('cls' if os.name=='nt' else 'clear')
for i, filename in enumerate(examples):
    print(f"\n#{i+1} {filename} {'='*20}\n")
    os.system(f"python3 -m examples.{filename}")

