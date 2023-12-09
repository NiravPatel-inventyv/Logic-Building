//Max between three
let x = parseInt(prompt("Enter no1 : "))
let y = parseInt(prompt("Enter no2 : "))
let z = parseInt(prompt("Enter no3 : "))

// logic
if(x > y){
    if(x > z) {
        console.log(`${x} is maximum.`)
    }
}else if(y > z ){

    console.log(`${y} is maximum.`)
}else{
    console.log(`${z} is maximum.`)

}
