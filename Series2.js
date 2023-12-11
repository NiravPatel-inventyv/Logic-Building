// 1 -3 5 -7 9 -11....
let n = parseInt(prompt("Enter Last number: "))
// Type to check positive negative
let type = 1
//Function to find factorial
function factorial(no){
let fact = 1
for(let j = 1;j <= no ;j++){
    fact *= j
}
return fact
}

// loop iteration 
for(let i = 1; i <= n ; i +=2){
    if(type > 0){
        console.log(factorial(i) * type)
    }else{
        console.log(factorial(i) * type)
    }
    type *= -1
}

