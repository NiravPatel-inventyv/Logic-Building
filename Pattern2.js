// print 1 12 123 1234
let n = parseInt(prompt("Enter number of rows : "))
//to print in single line
let series = ""

//outer loop
for(let i = 1 ; i <= n; i++){
    //to combine no togeather
    let str = ""
    //inner loop1 for incresing order
    for(let j = 1 ; j <= i; j++){
    str += j
    }
     //inner loop2 for descresing order
    for(let j = i-1; j > 0; j--){
        str += j 
    }  

    //concat with space between items
    series += " " + str
}
console.log(series)