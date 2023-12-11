//Max between four
let x = parseInt(prompt("Enter no1 : "))
let y = parseInt(prompt("Enter no2 : "))
let z = parseInt(prompt("Enter no3 : "))
let w = parseInt(prompt("Enter no4 : "))

// logic
if(x > y){
    if(x > z) {
       if(x > w){
        console.log(`${x} is maximum.`)
       }
    }else if(z > w){
        console.log(`${z} is maximum.`)
    }else{
        console.log(`${w} is maximum.`)
    }
}else if(y > z ){
if(y > w){

    console.log(`${y} is maximum.`)
}
}else if (z > w){
    console.log(`${z} is maximum.`)

}else{
    console.log(`${w} is maximum.`)
}
