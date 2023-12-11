//Program to find factorial of a number
//Input
let m = parseInt(prompt("Enter number : "))
let fact = 1
//If input is valid execute the logic
if(m > 0 ){
    for(let i = 1;i <= m ;i++){
        fact *= i
    }
    console.log(`Factorial of ${m} is : ${fact}`)
}
//show message of invalid input
else{
    console.log("Enter valid number.")
}
