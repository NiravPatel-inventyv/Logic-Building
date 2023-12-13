// 1! -3! 5! -7! 9! -11!....
let n = parseInt(prompt("Enter number of terms: "))
let fact = 1
for(let i = 3; i <= (2 * n + 1) ; i +=2){
console.log(fact)
fact *= i * (i - 1) * -1
}

