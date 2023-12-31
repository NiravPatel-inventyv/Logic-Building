const isPrime = (number)=>{
    const no = BigInt(number)
    if (no <= 1n) {
        return false;
    }
    if (no <= 3n) {
        return true;
    }
    if (no % 2n === 0n || no % 3n === 0n) {
        return false;
    }
    let i = 5n;
    while (i * i <= no) {
        if (no % i === 0n || no % (i + 2n) === 0n) {
            return false;
        }
        i += 6n;
    }
    return true;
}

const  primeFibonacci = (start,end)=>{
    let primes = []
let a = 0n;
let b = 1n;
for (let i = 1; i <= end; i++) {
    const c = a + b;
    a = b;
    b = c;
    if (i >= start && isPrime(c)) {
        primes.push(c);
    }
}
return primes;
}


const batchSize = 100; // Define batch size
const startTerm = 1; // Starting term of the batch
const endTerm = 1001; // Ending term of the batch

console.time('Calculation');

let allPrimes = [];
for (let i = startTerm; i <= endTerm; i += batchSize) {
    const start = i;
    const end = Math.min(i + batchSize - 1, endTerm);
    const primes = primeFibonacci(start, end);
    allPrimes = allPrimes.concat(primes);
}

console.log(allPrimes);
console.timeEnd('Calculation');
// function Difference(primeArr) {
//     let diff = [];
//     for (let i = 1; i < primeArr.length; i++) {
//       diff.push(BigInt(primeArr[i-1]) - BigInt(primeArr[i]));
//     }
//     return diff;
//   }
  
//   function Addition(arr) {
//     let sum = 0n;
//     for (let i of arr) {
//       sum += i;
//     }
//     return sum;
//   }

// console.time('Calculation')
// let ans = primeFibbonaci(100)
// console.log(ans)
// console.log("Difference is :" + Addition(Difference(ans)))
// console.timeEnd('Calculation')

