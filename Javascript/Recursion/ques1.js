// find nCr using recursion

function Factorial(n) {
  if (n == 0 || n == 1) return 1;
  return n * Factorial(n - 1);
}

function combine(n, r) {
  return Factorial(n) / (Factorial(r) * Factorial(n - r));
}

function Main(n, r) {
  if (r == 0 || r == n) return 1;
  return combine(n, r);
}

const n = 5;
const r = 3;
const result = Main(n, r);
console.log(`The combination ${n}C${r} is: ${result}`);
