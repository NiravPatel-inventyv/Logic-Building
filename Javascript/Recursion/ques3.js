// program to find prime fibonacci number and also find the difference between them and sum of the difference
function fibonacci(num){
  if(num == 0) return []
  if(num == 1) return [BigInt(0)]
  if(num == 2) return [BigInt(0),BigInt(1)]
  else{
    let fibArr = fibonacci(num -1)
    fibArr.push(fibArr[fibArr.length - 1] + fibArr[fibArr.length - 2]);
    return fibArr;
  }
}

function isPrime(num) {
  if (num <= 1n) return false;
  if (num === 2n || num === 3n) return true;
  if (num % 2n === 0n || num % 3n === 0n) return false;
  let i = 5n;
  let w = 2n;

  while (i * i <= num) {
    if (num % i === 0n) return false;
    i += w;
    w = 6n - w;
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
  let sum = 0n;
  for (let i of arr) {
    sum += i;
  }
  return sum;
}

function PrimeFibbonaci(n) {
  // if(n > 45){
  //   console.log("Opps!! n is too big for calculation.\nPlease enter value of n less than 45")
  // }else{

    let fibArr = fibonacci(n);
    let primes = fibArr.filter(isPrime);
    let differences = Difference(primes);
    let sumOfDifferences = Addition(differences);
    console.log("Fibbonaci series : ", fibArr)
    console.log("length of series : ", fibArr.length)
    console.log("Prime Fibonacci Numbers:", primes);
    console.log("Prime Length:", primes.length);
    console.log("Differences:", differences);
    console.log("Sum of Differences:", sumOfDifferences);
  // }
}

PrimeFibbonaci(50);
