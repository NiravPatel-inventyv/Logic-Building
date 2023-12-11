// 1 -2 3 -4....
let n = parseInt(prompt("Enter the last number : "))
let i = 1
while(i <= n ){
    if(i > 0){
    console.log(i)
    i += 1
}else{
    console.log(i)
    i -= 1
}
i*= -1
}