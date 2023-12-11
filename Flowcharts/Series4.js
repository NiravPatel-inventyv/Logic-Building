// 1 232 34543 ...
let n = prompt("Enter number :")
for(let i = 1; i <=n; i++){
    let str = " "
    for(let j = i ; j < (2 * i - 1); j++){
        str += j 
    }
    for(let j = (2* i - 1); j > i-1 ; j-- ){
        str += j 
    }
    console.log(str)
}