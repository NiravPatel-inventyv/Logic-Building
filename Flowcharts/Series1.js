// 1 -2 3 -4....
let n = parseInt(prompt("Enter the last number : "))
let type = 1
for(let i=1; i <=n ; i++ ){
    console.log(i * type)
    type *= -1
}