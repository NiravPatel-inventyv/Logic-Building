// program to find prime fibonacci number and also find the difference between them and sum of the difference
function fibonacci(num) {
  if (num <= 0) {
    return [];
  } else if (num === 1) {
    return [0];
  } else if (num === 2) {
    return [0, 1];
  } else {
    const fibArr = fibonacci(num - 1);
    fibArr.push(fibArr[fibArr.length - 1] + fibArr[fibArr.length - 2]);
    return fibArr;
  }
}

function isPrime(num) {
  if (num <= 1) return false;
  for (let i = 2; i < Math.sqrt(num); i++) {
    if (num % i == 0) {
      return false;
    }
  }
  return true;
}

function Difference(primeArr) {
  let diff = [];
  for (let i = 1; i < primeArr.length; i++) {
    diff.push(primeArr[i] - primeArr[i - 1]);
  }
  return diff;
}

function Addition(arr) {
  let sum = 0;
  for (let i of arr) {
    sum += i;
  }
  return sum;
}

function PrimeFibbonaci(n) {
  let fibArr = fibonacci(n);
  let primes = fibArr.filter(isPrime);
  let differences = Difference(primes);
  let sumOfDifferences = Addition(differences);
  console.log("Prime Fibonacci Numbers:", primes);
  console.log("Differences:", differences);
  console.log("Sum of Differences:", sumOfDifferences);
}

PrimeFibbonaci(1001);
