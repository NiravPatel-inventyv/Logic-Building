// 1 -3 5 -7 9 -11....
let n = parseInt(prompt("Enter the last number : "))
let i = 1
while(i <= n ){
    if(i > 0){
    console.log(i)
    i += 2
}else{
    console.log(i)
    i -= 2
}
i*= -1
}