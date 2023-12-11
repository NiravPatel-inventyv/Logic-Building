let n = prompt("Enter number : ")
let a = 0
let b = 1
let str = ""
while( a < n){
    str += " " + a
    c = a+b
    a = b
    b = c
}
console.log(str)