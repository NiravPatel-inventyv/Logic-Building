//program to find sum of numbers 
//Input
let m = parseInt(prompt("Enter number : "))
let sum = 0
//If input is valid execute the logic
if(m > 0){
    for(let i = 1;i <= m ;i++){
        sum += i
    }
    console.log(`sum of numbers is : ${sum}`)
}
//show message of invalid input
else{
    console.log("Enter valid number.")
}
