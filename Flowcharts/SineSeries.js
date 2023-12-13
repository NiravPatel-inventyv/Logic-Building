let x = prompt("Enter x :");
let n = prompt("Enter n :");
x= (x *3.14159)/180;
let result = 0;
let fact = 1;
let i = 1;
while (i <= n) {
  result += x ** (2 * i - 1) / fact;
  i += 1;
  fact = fact * (2 * i - 1) * (2 * i - 2) * -1;
}

console.log(result);
